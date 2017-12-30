#![feature(integer_atomics)]
#![feature(rand)]
#![feature(test)]
#![feature(universal_impl_trait)]

#[macro_use]
extern crate diesel;
extern crate diesel_infer_schema;

#[macro_use]
extern crate diesel_migrations;

extern crate clap;
extern crate indicatif;
extern crate test;

#[cfg(test)]
extern crate rand;

mod args;
mod converter;
mod db;
mod io;
mod progressbar;
mod wordvector;

#[cfg(feature = "dumb")]
mod data;

use std::rc::Rc;
use args::Args;
use converter::Converter;
use progressbar::Progress;
use wordvector::dataiterator::DataIterator;
use wordvector::VectorFile;

fn convert_process(converter: Converter, vector_files: &[VectorFile]) {
    converter.prepare();

    let progress_signal = Rc::new(Progress::start());
    let data_iterators = DataIterator::make_vec(progress_signal.clone(), vector_files);

    progress_signal.init(data_iterators.len() as u64);

    data_iterators.into_iter()
        .for_each(|mut data_iter| {
            progress_signal.start(data_iter.kind(), 100);

            converter.convert(&mut data_iter);
        });
}

fn main() {
    #[cfg(feature = "dumb")]
        data::test("test.data");

    let mut arg = Args::default();

    if arg.is_incomplete() {
        arg.print_help();
    } else {
        match Converter::new(arg.database_uri().unwrap()) {
            Ok(converter) => convert_process(converter, arg.file_path().unwrap()),
            Err(err) => println!("failed to initialise a converter with {:?}", err),
        }
    }
}

