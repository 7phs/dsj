use std::fs::File;
use std::io::BufReader;
use std::rc::Rc;
use super::{VectorFile, Iter};
use super::fasttext::FastText;
use super::gensim::Gensim;
use super::glove::Glove;
use super::word2vec::Word2Vec;
use progressbar::IncSignal;

macro_rules! file_iterator {
    ($kind: expr, $typ: ident, $file_path: ident, $signal: expr) => { match File::open(&$file_path) {
        Ok(file) => Some((
            $kind.to_string(),
            file.metadata().unwrap().len(),
            $typ::new(BufReader::new(file), $signal).into_iter()
        )),
        Err(_) => None
    }}
}

pub struct DataIterator {
    kind: String,
    max: u64,
    iterator: Iter,
}

impl DataIterator {
    pub fn make_vec(signal: Rc<IncSignal>, vec_files: &[VectorFile]) -> Vec<DataIterator> {
        vec_files.iter().filter_map(|file_name| {
            let result = match file_name {
                &VectorFile::FastText(ref file_path) => file_iterator!("fasttext", FastText, file_path, Some(signal.clone())),
                &VectorFile::Glove(ref file_path) => file_iterator!("glove", Glove, file_path, Some(signal.clone())),
                &VectorFile::Word2Vec(ref file_path) => file_iterator!("word2vec", Word2Vec, file_path, Some(signal.clone())),
                &VectorFile::Gensim(ref file_path) => file_iterator!("gensim", Gensim, file_path, Some(signal.clone())),
                &VectorFile::Unknown => None,
            };

            let (kind, max, iterator) = result?;

            Some(DataIterator {
                kind,
                max,
                iterator,
            })
        }).collect()
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn max(&self) -> u64 {
        self.max
    }

    pub fn iter(&mut self) -> &mut Iter {
        &mut self.iterator
    }
}
