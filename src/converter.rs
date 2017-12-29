use db::connection::{DsjConnection, establish_connection, run_migrations};
use db::models::word::create_word;
use db::models::kind::create_kind;
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

    pub fn convert(&self, data_iterator: &mut DataIterator) -> Result<(), String> {
        if let Some(kind) = create_kind(&self.connection, data_iterator.kind()) {
            for record in data_iterator.iter() {
                if let Some(word) = create_word(&self.connection, &record.word) {
                    add_vectors(&self.connection, &Vector::from_vec(&word, &kind, &record.vec));
                }
            }
        }

        Ok(())
    }
}
