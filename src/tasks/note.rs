use chrono::prelude::Local;
use chrono::{TimeZone, Utc};

use sqlx;
use sqlx::sqlite::SqliteQueryAs;
use std::fmt;
use tokio::stream::StreamExt;

#[derive(Debug, sqlx::FromRow)]
pub struct Note {
    id: i32,         //integer of 32 bits, -2**31, 2**31
    message: String, // string
    time: i32,
}
impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let time = Local.timestamp(self.time.into(), 0);
        // let time = DateTime::<Local>::from_utc(NaiveDateTime::from_timestamp(self.time.into(), 1800), );

        write!(
            f,
            "{:>3}) {:<40}  - {}",
            self.id,
            self.message,
            time.format("%T %D")
        )
    }
}
pub async fn add_new(msg: String, db: &mut sqlx::SqliteConnection) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO notes
                (Message, Time)
                values ( ? , ? )",
    )
    .bind(msg)
    .bind(Utc::now().timestamp())
    .execute(db)
    .await?;
    Ok(())
}

pub async fn get_all(db: &mut sqlx::SqliteConnection) -> Result<Vec<Note>, sqlx::Error> {
    let mut notes_cursor =
        sqlx::query_as::<_, Note>("SELECT Id, Message, Time FROM notes").fetch(db);
    let mut notes = Vec::new();

    // breaks if there's any error
    while let Some(result) = notes_cursor.next().await {
        notes.push(result.unwrap());
    }
    Ok(notes)
}

pub async fn delete(id: i32, db: &mut sqlx::SqliteConnection) -> Result<(), sqlx::Error> {
    sqlx::query(
        "DELETE FROM notes
                WHERE Id = ?",
    )
    .bind(id)
    .execute(db)
    .await?;
    Ok(())
}
