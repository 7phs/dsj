use diesel::Connection;
use diesel::mysql::MysqlConnection;

embed_migrations!("migrations/mysql");

pub fn run_migration(connection: &MysqlConnection) -> Result<(), String> {
    match embedded_migrations::run_with_output(connection, &mut ::std::io::stdout()) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Error make migrations {:?}", err)),
    }
}

pub fn establish_connection(database_url: &str) -> Result<MysqlConnection, String> {
    match MysqlConnection::establish(database_url) {
        Ok(conn) => Ok(conn),
        Err(err) => Err(format!("Error connecting to {} with {:?}", database_url, err)),
    }
}
