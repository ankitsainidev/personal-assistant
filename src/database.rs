use sqlx::{query, Connect, Connection, SqliteConnection};
use std::path::Path;

const DB_PATH: &str = "/home/ankit/.pat/data.db";

pub fn db_exists() -> bool {
    Path::new(DB_PATH).exists()
}

async fn create_db() -> Result<(), sqlx::Error> {
    // creates a new database and all the tables required by crate

    let mut conn = SqliteConnection::connect(&format!("sqlite://{}", DB_PATH)).await?;
    query(include_str!("static/schema.sql"))
        .execute(&mut conn)
        .await?;

    conn.close().await?;

    Ok(())
}

pub async fn db() -> Result<SqliteConnection, sqlx::Error> {
    if !db_exists() {
        create_db().await.expect("Problem connecting to db");
    }
    let db_url = &format!("sqlite://{}", DB_PATH);
    let conn = SqliteConnection::connect(db_url).await?;
    Ok(conn)
}
