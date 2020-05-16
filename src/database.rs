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
    use super::*;
    use std::env;

    fn testing_dir() -> String{
        let testing_database = env::current_dir()
        .expect("Can't get current directory")
        .join("testing_files");
        let database_path = testing_database.to_str().unwrap();
        database_path.to_string()
    }

    #[tokio::test]
    async fn table_exists() {
        let mut database = db(&testing_dir()).await.expect("hm");
        query("SELECT * from notes").execute(&mut database).await.expect("can't find");
        query("SELECT * from todos").execute(&mut database).await.expect("can't find");
        query("SELECT * from saves").execute(&mut database).await.expect("can't find");
    }

}
