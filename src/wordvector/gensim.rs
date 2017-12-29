use std::io::{BufRead, Cursor};
use std::rc::Rc;

use wordvector::{Record, Iter};
use progressbar::IncSignal;

pub struct Gensim<T>
    where T: BufRead + Sized
{
    reader: T,
    signal: Option<Rc<IncSignal>>,
    buf: Vec<u8>,
    delimiter: u8,
}

impl<T: 'static> Gensim<T>
    where T: BufRead + Sized
{
    pub fn new(reader: T, signal: Option<Rc<IncSignal>>) -> Gensim<T> {
        Gensim {
            reader,
            signal,
            buf: Vec::new(),
            delimiter: b']',
        }
    }

    fn read_line(&mut self) -> Option<Record> {
        self.buf.clear();

        match self.reader.read_until(self.delimiter, &mut self.buf) {
            Ok(0) => None,
            Ok(delta) => {
                self.inc(delta);

                if self.buf[self.buf.len() - 1] == self.delimiter {
                    self.buf.pop();
                }

                Some(self.parse(&self.buf))
            }
            Err(_) => None
        }
    }

    fn parse_word(&self, line: &str) -> String {
        let mut parser = line
            .split_whitespace()
            .take(2)
            .skip(1);

        parser.next().unwrap_or_default().trim().to_string()
    }

    fn parse_weights(&self, line: &str) -> Vec<f32> {
        line
            .split_whitespace()
            .map(|word| word.parse::<f32>().unwrap_or_default())
            .collect()
    }

    fn parse(&self, line: &[u8]) -> Record {
        let line = String::from_utf8_lossy(line);

        let mut reader = Cursor::new(line.as_ref());

        let word = {
            let mut header_buf: Vec<u8> = vec![];

            match reader.read_until(b'[', &mut header_buf) {
                Ok(_) => self.parse_word(&String::from_utf8_lossy(&header_buf)),
                Err(_) => "".to_string()
            }
        };

        let weights = self.parse_weights(&line[reader.position() as usize..]);

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

impl<T: 'static> Iterator for Gensim<T>
    where T: BufRead + Sized
{
    type Item = Record;

    fn next(&mut self) -> Option<Record> {
        self.read_line()
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;
    use wordvector::testing::TestIncCounter;

    fn test_gensim_iter(iter: &mut Iter) {
        match iter.next() {
            Some(record) => {
                assert_eq!(
                    record.word,
                    "академиков",
                    "check first word"
                );

                let ln = record.vec.len();

                assert_eq!(
                    &record.vec[..5],
                    &[0.55785882f32, -0.768264, -0.17276528, 0.0782531, 0.80086225],
                    "check first piece of word vector"
                );

                assert_eq!(
                    &record.vec[ln - 5..],
                    &[0.9875887f32, -0.19531961, -0.13729578, -0.18418171, 0.53765053],
                    "check last piece of word vector"
                );
            }
            None => assert!(false, "failed to read an any records"),
        }

        match iter.last() {
            Some(record) => {
                assert_eq!(
                    record.word,
                    "шаров",
                    "check word"
                );

                let ln = record.vec.len();

                assert_eq!(
                    &record.vec[..5],
                    &[0.20803888f32, 0.2093377, -0.32918587, -0.61297339, -0.15897623],
                    "check first piece of the last word vector"
                );

                assert_eq!(
                    &record.vec[ln - 5..],
                    &[-0.10956758f32, 0.5555467, -0.9010241, -1.01419806, 0.0154422],
                    "check last piece of the last word vector"
                );
            }
            None => assert!(false, "failed to read the last record"),
        }
    }

    fn test_gensim_iter_count(iter: &mut Iter) {
        let count = iter.count();

        assert_eq!(count, 5, "check word count");
    }

    #[test]
    fn test_gensim_buffer_iter() {
        let test_data = include_str!("../../test/data/gensim.tsv");

        let expected_data_len = test_data.as_bytes().len() as u64;
        let exist_counter = Rc::new(TestIncCounter::default());

        test_gensim_iter(&mut Gensim::new(
            BufReader::new(Cursor::new(test_data)),
            Some(exist_counter.clone()),
        ).into_iter());

        assert_eq!(exist_counter.value(), expected_data_len, "check counter");

        test_gensim_iter_count(&mut Gensim::new(
            BufReader::new(Cursor::new(test_data)),
                None,
        ).into_iter());
    }

    #[test]
    fn test_gensim_file_iter() {
        let file_name = "test/data/gensim.tsv";

        match File::open(&file_name) {
            Ok(file) => {
                test_gensim_iter(&mut Gensim::new(
                    BufReader::new(file),
                    None,
                ).into_iter());
            }
            Err(err) => assert!(false, "failed to open file '{}' to test iter values with {:?}", &file_name, err),
        }

        match File::open(&file_name) {
            Ok(file) => {
                test_gensim_iter_count(&mut Gensim::new(
                    BufReader::new(file),
                    None,
                ).into_iter());
            }
            Err(err) => assert!(false, "failed to open file '{}' to test iter count with {:?}", &file_name, err),
        }
    }
}