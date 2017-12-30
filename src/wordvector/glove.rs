use std::io::BufRead;
use std::rc::Rc;

use wordvector::{Record, Iter};
use progressbar::IncSignal;
use io::IteratorWithSignal;

pub struct Glove<T>
    where T: BufRead + Sized
{
    iterator: IteratorWithSignal<T>,
    signal: Option<Rc<IncSignal>>,
}

impl<T: 'static> Glove<T>
    where T: BufRead + Sized
{
    pub fn new(reader: T, signal: Option<Rc<IncSignal>>) -> Glove<T> {
        Glove {
            iterator: IteratorWithSignal::new(reader, b'\n'),
            signal,
        }
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

impl<T: 'static> Iterator for Glove<T>
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

    fn test_glove_iter(iter: &mut Iter) {
        match iter.next() {
            Some(record) => {
                assert_eq!(
                    record.word,
                    "the",
                    "check first word"
                );

                let ln = record.vec.len();

                assert_eq!(
                    &record.vec[..5],
                    &[-0.634127f32, -1.770394, 0.131677, -0.059609, 1.111649],
                    "check first piece of word vector"
                );

                assert_eq!(
                    &record.vec[ln - 5..],
                    &[-1.152042f32, -0.683840, -0.097181, 0.070575, 1.122378],
                    "check last piece of word vector"
                );
            }
            None => assert!(false, "failed to read an any records"),
        }

        match iter.last() {
            Some(record) => {
                assert_eq!(
                    record.word,
                    "two",
                    "check word"
                );

                let ln = record.vec.len();

                assert_eq!(
                    &record.vec[..5],
                    &[0.547172f32, -1.660098, -0.250286, 1.269076, 0.440363],
                    "check first piece of the last word vector"
                );

                assert_eq!(
                    &record.vec[ln - 5..],
                    &[-1.376995f32, -0.795934, 0.957354, -0.111903, 1.148171],
                    "check last piece of the last word vector"
                );
            }
            None => assert!(false, "failed to read the last record"),
        }
    }

    fn test_glove_iter_count(iter: &mut Iter) {
        let count = iter.count();

        assert_eq!(count, 10, "check word count");
    }

    #[test]
    fn test_glove_buffer_iter() {
        let test_data = include_str!("../../test/data/glove.txt");

        let expected_data_len = test_data.as_bytes().len() as u64;
        let exist_counter = Rc::new(TestIncCounter::default());

        test_glove_iter(&mut Glove::new(
            BufReader::new(Cursor::new(test_data)),
            Some(exist_counter.clone()),
        ).into_iter());

        assert_eq!(exist_counter.value(), expected_data_len, "check counter");

        test_glove_iter_count(&mut Glove::new(
            BufReader::new(Cursor::new(test_data)),
            None,
        ).into_iter());
    }

    #[test]
    fn test_glove_file_iter() {
        let file_name = "test/data/glove.txt";

        match File::open(&file_name) {
            Ok(file) => {
                test_glove_iter(&mut Glove::new(
                    BufReader::new(file),
                    None,
                ).into_iter());
            }
            Err(err) => assert!(false, "failed to open file '{}' to test iter values with {:?}", &file_name, err),
        }

        match File::open(&file_name) {
            Ok(file) => {
                test_glove_iter_count(&mut Glove::new(
                    BufReader::new(file),
                    None,
                ).into_iter());
            }
            Err(err) => assert!(false, "failed to open file '{}' to test iter count with {:?}", &file_name, err),
        }
    }
}