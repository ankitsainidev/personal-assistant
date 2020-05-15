use sqlx::{query, Connect, Connection, SqliteConnection};
use std::env;
use std::path::Path;

pub fn get_database_path() -> String {
    let home_dir: String = env::var("HOME").expect("Can't reach home directory.");

    let home_path = Path::new(&home_dir);
    let pat_path = home_path.join(".pat").join("data.db");
    pat_path.to_str().unwrap().to_string()
}

pub fn db_exists() -> bool {
    Path::new(&get_database_path()).exists()
}

async fn create_db() -> Result<(), sqlx::Error> {
    let path = get_database_path();

    // creates a new database and all the tables required by crate

    let mut conn = SqliteConnection::connect(&format!("sqlite://{}", path)).await?;
    query(include_str!("static/schema.sql"))
        .execute(&mut conn)
        .await?;

    conn.close().await?;

    Ok(())
}

pub async fn db() -> Result<SqliteConnection, sqlx::Error> {
    let path = get_database_path();
    if !db_exists() {
        create_db().await.expect("Problem connecting to db");
    }
    let db_url = &format!("sqlite://{}", path);
    let conn = SqliteConnection::connect(db_url).await?;
    Ok(conn)
}
