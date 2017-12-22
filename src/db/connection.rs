use diesel::sqlite::SqliteConnection;
use db::sqlite;

pub type DsjConnection = SqliteConnection;

pub fn establish_connection(db_uri: &str) -> Result<DsjConnection, String> {
    sqlite::establish_connection(db_uri)
}

pub fn run_migrations(connection: &DsjConnection) -> Result<(), String> {
    sqlite::run_migration(connection)
}