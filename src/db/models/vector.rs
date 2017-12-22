use diesel::{self};
use diesel::prelude::*;

use db::connection::DsjConnection;
use db::schema::vectors;
use db::models::word::Word;

#[derive(Debug, Insertable, Queryable, PartialEq)]
#[table_name = "vectors"]
pub struct Vector {
    pub word_id: i32,
    pub position: i32,
    pub point: f32,
}

impl Vector {
    pub fn new(word: &Word, position: i32, point: f32) -> Vector {
        Vector {
            word_id: word.id,
            position,
            point,
        }
    }

    pub fn from_vec(word: &Word, points: &[f32]) -> Vec<Vector> {
        points.into_iter()
            .enumerate()
            .map(|(position, &point)|
                Vector::new(&word, position as i32, point)
            )
            .collect()
    }
}

pub fn add_vectors(conn: &DsjConnection, new_vectors: &[Vector]) -> usize {
    use db::schema::vectors::dsl::vectors;

    match diesel::insert_into(vectors)
        .values(new_vectors)
        .execute(conn) {
        Ok(count) => count,
        Err(err) => {
            println!("failed to insert new vector {:?}", err);
            0
        }
    }
}

pub fn word_2_vector(conn: &DsjConnection, word: &Word) -> Option<Vec<f32>> {
    use db::schema::vectors::dsl::{vectors, word_id, position};

    match vectors
        .filter(word_id.eq(word.id))
        .order(position)
        .load::<Vector>(conn) {
        Ok(stmt) => Some(stmt.iter().map(|vec| vec.point).collect()),
        Err(_) => None,
    }
}