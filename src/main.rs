#![feature(test)]
#![feature(universal_impl_trait)]

#[macro_use]
extern crate diesel;
extern crate diesel_infer_schema;

#[macro_use]
extern crate diesel_migrations;

extern crate clap;
extern crate test;

mod args;
mod converter;
mod data;
mod db;
mod wordvector;

fn main() {
    #[cfg(feature = "dumb")]
        data::test("test.data");

    let mut arg = args::Args::default();

    if arg.is_incomplete() {
        arg.print_help();
    } else {
        match converter::Converter::new(arg.database_uri().unwrap()) {
            Ok(converter) => {
                converter.prepare();

                let mut data_iter = wordvector::dataiterator::DataIterator::make_vec(arg.file_path().unwrap());
                converter.convert(&mut data_iter);
            }
            Err(err) => {
                println!("failed to initialise a converter with {:?}", err);
            }
        }
    }
}

