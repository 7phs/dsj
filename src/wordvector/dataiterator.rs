use std::fs::File;
use std::io::BufReader;
use super::{VectorFile, Iter};
use super::fasttext::FastText;
use super::gensim::Gensim;
use super::glove::Glove;
use super::word2vec::Word2Vec;

pub struct DataIterator {
    iterator: Iter,
    id: String,
}

impl DataIterator {
    pub fn make_vec(vec_files: &[VectorFile]) -> Vec<DataIterator> {
        vec_files.iter().filter_map(|file_name| {
            let result = match file_name {
                &VectorFile::FastText(ref file_path) =>
                    match File::open(&file_path) {
                        Ok(file) => Some((
                            FastText::new(BufReader::new(file)).into_iter(),
                            "fasttext".to_string()
                        )),
                        Err(_) => None
                    }
                &VectorFile::Glove(ref file_path) =>
                    match File::open(&file_path) {
                        Ok(file) => Some((
                            Glove::new(BufReader::new(file)).into_iter(),
                            "glove".to_string()
                        )),
                        Err(_) => None
                    }
                &VectorFile::Word2Vec(ref file_path) =>
                    match File::open(&file_path) {
                        Ok(file) => Some((
                            Word2Vec::new(BufReader::new(file)).into_iter(),
                            "word2vec".to_string()
                        )),
                        Err(_) => None
                    }
                &VectorFile::Gensim(ref file_path) =>
                    match File::open(&file_path) {
                        Ok(file) => Some((
                            Gensim::new(BufReader::new(file)).into_iter(),
                            "gensim".to_string()
                        )),
                        Err(_) => None
                    }
                &VectorFile::Unknown => None,
            };

            let (iterator, id) = result?;

            Some(DataIterator {
                iterator,
                id,
            })
        }).collect()
    }

    pub fn iter(&mut self) -> &mut Iter {
        &mut self.iterator
    }
}
