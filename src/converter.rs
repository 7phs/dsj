use diesel::Connection;
use diesel::result::Error;

use db::connection::{DsjConnection, establish_connection, run_migrations};
use db::models::word::{create_word, get_word};
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
            let mut iter = data_iterator.iter();

            let mut count = 0;

            loop {
                self.connection.transaction::<_, Error, _>(|| {
                    count = 0;

                    for record in &mut iter {
                        let word = if let Some(word) = get_word(&self.connection, &record.word) {
                            word
                        } else if let Some(word) = create_word(&self.connection, &record.word) {
                            word
                        } else {
                            continue;
                        };

                        add_vectors(&self.connection, &Vector::from_vec(&word, &kind, &record.vec));

                        count += 1;

                        if count >= 1000 {
                            break;
                        }
                    }

                    Ok(())
                });

                if count == 0 {
                    break;
                }
            }
        }

        Ok(())
    }
}
