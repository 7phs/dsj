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
