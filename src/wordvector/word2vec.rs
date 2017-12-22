use std::io::{BufRead, Lines};
use std::rc::Rc;

use wordvector::{Record, Iter};

pub struct Word2Vec<T>
    where T: BufRead + Sized
{
    lines: Lines<T>,
}

impl<T: 'static> Word2Vec<T>
    where T: BufRead + Sized
{
    pub fn new(reader: T) -> Word2Vec<T> {
        let mut lines = reader.lines();

        // skip the first line contained meta information
        let _ = lines.next();

        Word2Vec {
            lines
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

    pub fn into_iter(self) -> Iter {
        Iter {
            iter: Rc::new(self)
        }
    }
}

impl<T: 'static> Iterator for Word2Vec<T>
    where T: BufRead + Sized
{
    type Item = Record;

    fn next(&mut self) -> Option<Record> {
        match self.lines.next()? {
            Ok(line) => Some(self.parse(&line)),
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    use std::fs::File;
    use std::io::{BufReader, Cursor};

    fn test_word2vec_iter(iter: &mut Iter) {
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
                    &[0.002001f32, 0.002210, -0.001915, -0.001639, 0.000683],
                    "check first piece of word vector"
                );

                assert_eq!(
                    &record.vec[ln - 5..],
                    &[0.000895f32, -0.000591, 0.000099, -0.000843, -0.000563],
                    "check last piece of word vector"
                );
            }
            None => assert!(false, "failed to read an any records"),
        }

        match iter.last() {
            Some(record) => {
                assert_eq!(
                    record.word,
                    "И",
                    "check word"
                );

                let ln = record.vec.len();

                assert_eq!(
                    &record.vec[..5],
                    &[-0.001489f32, 0.001176, 0.002261, -0.002077, -0.000209],
                    "check first piece of the last word vector"
                );

                assert_eq!(
                    &record.vec[ln - 5..],
                    &[0.001394, 0.000704, 0.000648, 0.000774, 0.000112],
                    "check last piece of the last word vector"
                );
            }
            None => assert!(false, "failed to read the last record"),
        }
    }

    fn test_word2vec_iter_count(iter: &mut Iter) {
        let count = iter.count();

        assert_eq!(count, 11, "check word count");
    }

    #[test]
    fn test_word2vec_buffer_iter() {
        let test_data = include_str!("../../test/data/word2vec.txt");

        test_word2vec_iter(&mut Word2Vec::new(BufReader::new(Cursor::new(test_data))).into_iter());

        test_word2vec_iter_count(&mut Word2Vec::new(BufReader::new(Cursor::new(test_data))).into_iter());
    }

    #[test]
    fn test_word2vec_file_iter() {
        let file_name = "test/data/word2vec.txt";

        match File::open(&file_name) {
            Ok(file) => {
                test_word2vec_iter(&mut Word2Vec::new(BufReader::new(file)).into_iter());
            }
            Err(err) => assert!(false, "failed to open file '{}' to test iter values with {:?}", &file_name, err),
        }

        match File::open(&file_name) {
            Ok(file) => {
                test_word2vec_iter_count(&mut Word2Vec::new(BufReader::new(file)).into_iter());
            }
            Err(err) => assert!(false, "failed to open file '{}' to test iter count with {:?}", &file_name, err),
        }
    }
}