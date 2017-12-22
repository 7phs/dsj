use diesel::Connection;
use diesel::sqlite::SqliteConnection;

embed_migrations!("migrations/sqlite");

pub fn run_migration(connection: &SqliteConnection) -> Result<(), String> {
    match embedded_migrations::run_with_output(connection, &mut ::std::io::stdout()) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Error make migrations {:?}", err)),
    }
}

pub fn establish_connection(database_url: &str) -> Result<SqliteConnection, String> {
    match SqliteConnection::establish(database_url) {
        Ok(conn) => Ok(conn),
        Err(err) => Err(format!("Error connecting to {} with {:?}", database_url, err)),
    }
}
