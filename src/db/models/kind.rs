use diesel::{self};
use diesel::prelude::*;

use db::connection::DsjConnection;
use db::schema::kinds;

#[derive(Debug, Queryable, PartialEq, Clone)]
pub struct Kind {
    pub id: i32,
    pub kind: String,
}

impl Default for Kind {
    fn default() -> Kind {
        Kind {
            id: 0,
            kind: "".to_string(),
        }
    }
}

#[derive(Debug, Insertable, PartialEq)]
#[table_name = "kinds"]
pub struct NewKind<'a> {
    pub kind: &'a str,
}

pub fn create_kind<'a>(conn: &DsjConnection, k: &'a str) -> Option<Kind> {
    use db::schema::kinds::dsl::kinds;

    let new_kind = NewKind {
        kind: k,
    };

    if let Err(err) = diesel::insert_into(kinds)
        .values(&new_kind)
        .execute(conn) {
        println!("failed to insert {:?} with {:?}", new_kind, err);
    }

    get_kind(&conn, k)
}

pub fn get_kind<'a>(conn: &DsjConnection, k: &'a str) -> Option<Kind> {
    use db::schema::kinds::dsl::{kinds, kind};

    match kinds.filter(kind.eq(k))
        .load::<Kind>(conn) {
        Ok(res) => Some(res.first()?.clone()),
        Err(err) => {
            println!("failed to get kind record for '{}' {:?}", k, err);
            None
        }
    }
}

pub fn kinds_list(conn: &DsjConnection) -> Option<Vec<Kind>> {
    use db::schema::kinds::dsl::kinds;

    match kinds.load::<Kind>(conn) {
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