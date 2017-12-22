use db::connection::{DsjConnection, establish_connection, run_migrations};
use db::models::word::create_word;
use db::models::vector::{Vector, add_vectors};
use wordvector::dataiterator::DataIterator;

pub struct Converter {
    connection: DsjConnection,
}

impl Converter {
    pub fn new(database_uri: &str) -> Result<Converter, String> {
        let connection = establish_connection(database_uri)?;

        Ok(Converter {
            connection
        })
    }

    pub fn prepare(&self) -> Result<(), String> {
        run_migrations(&self.connection)
    }

    pub fn convert(&self, data_iterator: &mut [DataIterator]) -> Result<(), String> {
        for data in data_iterator {
            for record in data.iter() {
                if let Some(word) = create_word(&self.connection, &record.word) {
                    add_vectors(&self.connection, &Vector::from_vec(&word, &record.vec));
                }
            }
        }

        Ok(())
    }
}