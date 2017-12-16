use diesel::sqlite::SqliteConnection;
use db::sqlite;

pub type DsjConnection = SqliteConnection;

pub fn establish_connection() -> DsjConnection {
    sqlite::establish_connection()
}