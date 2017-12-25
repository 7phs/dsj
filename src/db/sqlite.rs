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

#[cfg(test)]
mod testing {
    use super::*;
    use diesel::RunQueryDsl;

    #[test]
    fn test_migration() {
        use db::schema::kinds::dsl::kinds;
        use db::models::kind::Kind;

        let connection = match establish_connection(":memory:") {
            Ok(connection) => connection,
            Err(err) => {
                assert!(false, "failed to establish connection to :memory with {:?}", err);
                return;
            }
        };

        if let Err(err) = run_migration(&connection) {
            assert!(false, "failed to run migration to :memory with {:?}", err);
        }

        match kinds.load::<Kind>(&connection) {
            Ok(stmt) => assert_eq!(stmt.len(), 0, "check execution"),
            Err(err) => assert!(false, "failed to execute query with {:?}", err),
        }
    }
}