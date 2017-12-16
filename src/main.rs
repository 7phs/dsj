#![feature(test)]

#[macro_use]
extern crate diesel;
extern crate diesel_infer_schema;

#[macro_use]
extern crate diesel_migrations;

extern crate dotenv;
extern crate test;

mod connection;
mod datamem;
mod datafile;
mod db;
mod models;
mod schema;

use datamem::read_mem;
use datafile::read_file;

use connection::establish_connection;
use models::word::{Word, create_word, words_list, get_word};
use models::vector::{Vector, add_vectors, word_2_vector};

embed_migrations!("migrations");

fn main() {
    read_mem();

    if let Err(err) = read_file("test.data") {
        println!("failed to read data file {:?}", err);
    }

    let connection = establish_connection();

    embedded_migrations::run_with_output(&connection, &mut std::io::stdout());

    {
        let words = create_word(&connection, "привет");

        println!("create_word:1: {:?}", words);

        let words = create_word(&connection, "свидания");

        println!("create_word:2: {:?}", words);

        let words = words_list(&connection);

        println!("words: {:?}", words);
    }

    {
        let word = get_word(&connection, "привет").unwrap();

        let insert_count = add_vectors(&connection, &vec![
            0.15f32, 14.15, 23.78, 109.0192
        ].into_iter().enumerate()
            .map(|(position, point)|
                Vector::new(&word, position as i32, point)
            )
            .collect::<Vec<_>>(),
        );

        println!("add_vector:1: {:?}", insert_count);

        let vector = word_2_vector(&connection, &word);

        println!("vector: {:?}", vector);
    }
}

