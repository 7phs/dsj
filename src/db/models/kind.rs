use diesel::{self};
use diesel::prelude::*;

use db::connection::DsjConnection;
use db::schema::kinds;

#[derive(Debug, Queryable, PartialEq, Clone)]
pub struct Kind {
    pub id: i32,
    pub name: String,
}

impl Default for Kind {
    fn default() -> Kind {
        Kind {
            id: 0,
            name: "".to_string(),
        }
    }
}

#[derive(Debug, Insertable, PartialEq)]
#[table_name = "kinds"]
pub struct NewKind<'a> {
    pub name: &'a str,
}

pub fn create_kind<'a>(conn: &DsjConnection, n: &'a str) -> Option<Kind> {
    use db::schema::kinds::dsl::kinds;

    let new_kind = NewKind {
        name: n,
    };

    if let Err(err) = diesel::insert_into(kinds)
        .values(&new_kind)
        .execute(conn) {
        println!("failed to insert {:?} with {:?}", new_kind, err);
    }

    get_kind(&conn, n)
}

pub fn get_kind<'a>(conn: &DsjConnection, n: &'a str) -> Option<Kind> {
    use db::schema::kinds::dsl::{kinds, name};

    match kinds.filter(name.eq(n))
        .load::<Kind>(conn) {
        Ok(res) => Some(res.first()?.clone()),
        Err(err) => {
            println!("failed to get kind record for '{}' {:?}", n, err);
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
