use diesel::{self};
use diesel::prelude::*;

use db::connection::DsjConnection;
use db::schema::words;

#[derive(Debug, Queryable, PartialEq, Clone)]
pub struct Word {
    pub id: i32,
    pub word: String,
}

impl Default for Word {
    fn default() -> Word {
        Word {
            id: 0,
            word: "".to_string(),
        }
    }
}

#[derive(Debug, Insertable, PartialEq)]
#[table_name = "words"]
pub struct NewWord<'a> {
    pub word: &'a str,
}

pub fn create_word<'a>(conn: &DsjConnection, w: &'a str) -> Option<Word> {
    use db::schema::words::dsl::words;

    let new_word = NewWord {
        word: w,
    };

    if let Err(err) = diesel::insert_into(words)
        .values(&new_word)
        .execute(conn) {
        println!("failed to insert {:?} with {:?}", new_word, err);
    }

    get_word(&conn, w)
}

pub fn get_word<'a>(conn: &DsjConnection, w: &'a str) -> Option<Word> {
    use db::schema::words::dsl::{words, word};

    match words.filter(word.eq(w))
        .load::<Word>(conn) {
        Ok(res) => Some(res.first()?.clone()),
        Err(err) => {
            println!("failed to get word record for '{}' {:?}", w, err);
            None
        }
    }
}

pub fn words_list(conn: &DsjConnection) -> Option<Vec<Word>> {
    use db::schema::words::dsl::words;

    match words.load::<Word>(conn) {
        Ok(stmt) => Some(stmt),
        Err(_) => None,
    }
}

#[cfg(test)]
mod testing {
    #[test]
    fn test() {
//        {
//            let words = create_word(&connection, "привет");
//
//            println!("create_word:1: {:?}", words);
//
//            let words = create_word(&connection, "свидания");
//
//            println!("create_word:2: {:?}", words);
//
//            let words = words_list(&connection);
//
//            println!("words: {:?}", words);
//        }
//
//        {
//            let word = get_word(&connection, "привет").unwrap();
//
//            let vectors = Vector::from_vec(&word, &vec![
//                0.15f32, 14.15, 23.78, 109.0192
//            ]);
//
//            let insert_count = add_vectors(&connection, &vectors);
//
//            println!("add_vector:1: {:?}", insert_count);
//
//            let vector = word_2_vector(&connection, &word);
//
//            println!("vector: {:?}", vector);
//        }
    }
}