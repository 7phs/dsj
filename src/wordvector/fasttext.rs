use std::io::BufRead;
use std::rc::Rc;
use io::Pieces;
use wordvector::{Record, Iter};
use progressbar::IncSignal;

pub struct FastText<T>
    where T: BufRead + Sized
{
    iterator: Pieces<T>,
    signal: Option<Rc<IncSignal>>,
}

impl<T: 'static> FastText<T>
    where T: BufRead + Sized
{
    pub fn new(reader: T, signal: Option<Rc<IncSignal>>) -> FastText<T> {
        let mut fasttext = FastText {
            iterator: Pieces::new(reader, b'\n'),
            signal,
        };

        // skip the first line
        if let Some((delta, _)) = fasttext.iterator.next() {
            fasttext.inc(delta);
        }

        fasttext
    }

    fn parse(&self, line: &str) -> Record {
        let mut parser = line
            .split_whitespace();

        let word = parser.next().unwrap_or_default()
            .trim().to_string();

        let weights = parser
            .map(|value|
                value.parse::<f32>().unwrap_or_default()
            )
            .collect::<Vec<_>>();

        Record::new(word, &weights)
    }

    fn inc(&self, delta: usize) {
        if let Some(ref signal) = self.signal {
            signal.inc(delta as u64);
        }
    }

    pub fn into_iter(self) -> Iter {
        Iter {
            iter: Rc::new(self)
        }
    }
}

impl<T: 'static> Iterator for FastText<T>
    where T: BufRead + Sized
{
    type Item = Record;

    fn next(&mut self) -> Option<Record> {
        let (delta, line) = self.iterator.next()?;

        self.inc(delta);

        Some(self.parse(&line))
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    use std::fs::File;
    use std::io::{BufReader, Cursor};
    use wordvector::testing::TestIncCounter;

    fn test_fasttext_iter(iter: &mut Iter) {
        match iter.next() {
            Some(record) => {
                assert_eq!(
                    record.word,
                    "</s>",
                    "check first word"
                );

                let ln = record.vec.len();

                assert_eq!(
                    &record.vec[..5],
                    &[-0.015456f32, 0.02135, -0.018898, -0.0018487, 0.022132],
                    "check first piece of word vector"
                );

                assert_eq!(
                    &record.vec[ln - 5..],
                    &[-0.027139f32, 0.011303, 0.01991, -0.0096401, -0.012568],
                    "check last piece of word vector"
                );
            }
            None => assert!(false, "failed to read an any records"),
        }

        match iter.last() {
            Some(record) => {
                assert_eq!(
                    record.word,
                    "день",
                    "check word"
                );

                let ln = record.vec.len();

                assert_eq!(
                    &record.vec[..5],
                    &[0.0051658f32, -0.0040491, 0.0068376, -0.0034568, 0.0092959],
                    "check first piece of the last word vector"
                );

                assert_eq!(
                    &record.vec[ln - 5..],
                    &[-0.0044257f32, -0.0017585, 0.010604, 0.0073663, -0.0047922],
                    "check last piece of the last word vector"
                );
            }
            None => assert!(false, "failed to read the last record"),
        }
    }

    fn test_fasttext_iter_count(iter: &mut Iter) {
        let count = iter.count();

        assert_eq!(count, 9, "check word count");
    }

    #[test]
    fn test_fasttext_buffer_iter() {
        let test_data = include_str!("../../test/data/fasttext.vec");

        let expected_data_len = test_data.as_bytes().len() as u64;
        let exist_counter = Rc::new(TestIncCounter::default());

        test_fasttext_iter(&mut FastText::new(
            BufReader::new(Cursor::new(test_data)),
            Some(exist_counter.clone()),
        ).into_iter());

        assert_eq!(exist_counter.value(), expected_data_len, "check counter");

        test_fasttext_iter_count(&mut FastText::new(
            BufReader::new(Cursor::new(test_data)),
            None,
        ).into_iter());
    }

    #[test]
    fn test_fasttext_file_iter() {
        let file_name = "test/data/fasttext.vec";

        match File::open(&file_name) {
            Ok(file) => {
                test_fasttext_iter(&mut FastText::new(
                    BufReader::new(file),
                    None,
                ).into_iter());
            }
            Err(err) => assert!(false, "failed to open file '{}' to test iter values with {:?}", &file_name, err),
        }

        match File::open(&file_name) {
            Ok(file) => {
                test_fasttext_iter_count(&mut FastText::new(
                    BufReader::new(file),
                    None,
                ).into_iter());
            }
            Err(err) => assert!(false, "failed to open file '{}' to test iter count with {:?}", &file_name, err),
        }
    }
}