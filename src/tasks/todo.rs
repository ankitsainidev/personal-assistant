use sqlx::{self, sqlite::SqliteQueryAs};
use std::fmt;
use tokio::stream::StreamExt;

#[derive(Debug, sqlx::FromRow)]
pub struct Todo {
    id: i32,
    description: String,
    done: bool,
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status = if self.done { "Completed" } else { "Pending" };
        write!(
            f,
            "{id:>3}) {description:<30} {status}",
            id = self.id,
            description = self.description,
            status = status
        )
    }
}

pub async fn add_new(
    description: String,
    db: &mut sqlx::SqliteConnection,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO todos
                (description)
                values ( ?)",
    )
    .bind(description)
    .execute(db)
    .await?;
    Ok(())
}

pub async fn get_all(db: &mut sqlx::SqliteConnection) -> Result<Vec<Todo>, sqlx::Error> {
    let mut todos_cursor = sqlx::query_as::<_, Todo>("SELECT * FROM todos ORDER BY done").fetch(db);
    let mut todos = Vec::new();

    while let Some(result) = todos_cursor.next().await {
        todos.push(result.unwrap());
    }
    Ok(todos)
}

pub async fn mark_done(id: i32, db: &mut sqlx::SqliteConnection) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE todos SET done = ? WHERE id=?")
        .bind(true)
        .bind(id)
        .execute(db)
        .await?;
    Ok(())
}

pub async fn clean(db: &mut sqlx::SqliteConnection) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM todos WHERE done=?")
        .bind(true)
        .execute(db)
        .await?;
    Ok(())
}
