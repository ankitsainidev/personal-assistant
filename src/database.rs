use sqlx::{query, Connect, Connection, SqliteConnection};
use std::path::Path;

pub fn get_database_path(home_dir: &str) -> String {
    let home_path = Path::new(home_dir);
    let pat_path = home_path.join(".pat").join("data.db");
    pat_path.to_str().unwrap().to_string()
}

pub fn db_exists(home_dir: &str) -> bool {
    Path::new(&get_database_path(home_dir)).exists()
}

async fn create_db(home_dir: &str) -> Result<(), sqlx::Error> {
    let path = get_database_path(home_dir);

    // creates a new database and all the tables required by crate

    let mut conn = SqliteConnection::connect(&format!("sqlite://{}", path)).await?;
    query(include_str!("static/schema.sql"))
        .execute(&mut conn)
        .await?;

    conn.close().await?;

    Ok(())
}

pub async fn db(home_dir: &str) -> Result<SqliteConnection, sqlx::Error> {
    let path = get_database_path(home_dir);

    if !db_exists(home_dir) {
        create_db(home_dir).await.expect("Problem connecting to db");
    }
    let db_url = &format!("sqlite://{}", path);
    let conn = SqliteConnection::connect(db_url).await?;
    Ok(conn)
}

#[cfg(test)]
mod tests {
    // interacts with file I/O and should be tested with --test-threads=1

    use super::*;
    use std::env;
    use std::fs;

    fn get_test_dir() -> String {
        let test_dir = env::current_dir()
            .expect("Can't get current directory")
            .join("testing_files");
        let test_dir = test_dir.to_str().unwrap();
        test_dir.to_string()
    }

    fn setup() {
        let directory = get_test_dir();
        use std::io::ErrorKind;
        match fs::remove_dir_all(&directory) {
            Err(e) => {
                if e.kind() != ErrorKind::NotFound {
                    panic!("Problem setting up")
                }
            }
            _ => {}
        };
        fs::create_dir_all(&Path::new(&directory).join(".pat")).expect("Problem setting up");
    }

    #[tokio::test]
    async fn table_exists() {
        setup();

        let mut database = db(&get_test_dir()[..]).await.expect("Can't get database");
        for table in ["notes", "todos", "saves"].iter() {
            query(&format!("SELECT * from {}", table)[..])
                .execute(&mut database)
                .await
                .expect("can't find");
        }
    }
}
