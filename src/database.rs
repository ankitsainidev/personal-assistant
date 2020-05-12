use sqlx::{SqliteConnection, Connect, query, Connection};
use std::path::Path;
// use tokio;

const DB_PATH: &str = "/home/akanksha/code/rust/pat/data/data.db";

pub fn db_exists() -> bool {
    Path::new(DB_PATH).exists()
}

async fn create_db() -> Result<(), sqlx::Error> {
    let mut conn = SqliteConnection::connect(&format!("sqlite://{}", DB_PATH)).await?;

    query("CREATE TABLE notes (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        message VARCHAR(128) NOT NULL ,
        time INTEGER NOT NULL)")
        .execute(&mut conn).await?;

    conn.close().await?;

    Ok(())
}

pub async fn db()-> Result<SqliteConnection, sqlx::Error> {
    if !db_exists() {
        create_db().await.expect("Problem connecting to db");
    }
    let db_url = &format!("sqlite://{}", DB_PATH);
    let conn = SqliteConnection::connect(db_url).await?;
    Ok(conn)
}
