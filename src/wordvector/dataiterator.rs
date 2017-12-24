use std::fs::File;
use std::io::BufReader;
use super::{VectorFile, Iter};
use super::fasttext::FastText;
use super::gensim::Gensim;
use super::glove::Glove;
use super::word2vec::Word2Vec;

macro_rules! file_iterator {
    ($typ: ident, $file_path: ident, $kind: expr) => { match File::open(&$file_path) {
        Ok(file) => Some((
            $kind.to_string(),
            $typ::new(BufReader::new(file)).into_iter()
        )),
        Err(_) => None
    }}
}

pub struct DataIterator {
    kind: String,
    iterator: Iter,
}

impl DataIterator {
    pub fn make_vec(vec_files: &[VectorFile]) -> Vec<DataIterator> {
        vec_files.iter().filter_map(|file_name| {
            let result = match file_name {
                &VectorFile::FastText(ref file_path) => file_iterator!(FastText, file_path, "fasttext"),
                &VectorFile::Glove(ref file_path) => file_iterator!(Glove, file_path, "glove"),
                &VectorFile::Word2Vec(ref file_path) => file_iterator!(Word2Vec, file_path, "word2vec"),
                &VectorFile::Gensim(ref file_path) => file_iterator!(Gensim, file_path, "gensim"),
                &VectorFile::Unknown => None,
            };

            let (kind, iterator) = result?;

            Some(DataIterator {
                kind,
                iterator,
            })
        }).collect()
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn iter(&mut self) -> &mut Iter {
        &mut self.iterator
    }
}
