pub mod dataiterator;
mod fasttext;
mod gensim;
mod glove;
mod word2vec;

use std::rc::Rc;

#[derive(Clone)]
pub enum VectorFile {
    FastText(String),
    Glove(String),
    Word2Vec(String),
    Gensim(String),
    Unknown,
}

pub struct Record {
    pub word: String,
    pub vec: Vec<f32>,
}

impl Record {
    fn new(word: impl ToString, vec: &[f32]) -> Record {
        Record {
            word: word.to_string(),
            vec: Vec::from(vec),
        }
    }
}

pub struct Iter {
    pub iter: Rc<Iterator<Item=Record>>
}

impl Iterator for Iter {
    type Item = Record;

    fn next(&mut self) -> Option<Record> {
        Rc::get_mut(&mut self.iter)?.next()
    }
}


#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_record() {
        let rec = Record::new("hello", &[0.12f32, 3.14, 5.16]);

        let expected_word = "hello".to_string();
        let expected_vec = vec![0.12f32, 3.14, 5.16];

        assert_eq!(rec.word, expected_word, "check word init");
        assert_eq!(rec.vec, expected_vec, "check vec init");
    }
}