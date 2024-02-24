use sqlx::sqlite::SqlitePool;
use std::env;

pub mod models;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {

    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    let foo = add_user(&pool, self::models::models::User::new("foo".to_string(), "foo@bar".to_string())).await?;
    println!("Added user {}", foo);

    Ok(())
}

async fn add_user(pool: &SqlitePool, user: self::models::models::User) -> anyhow::Result<i64> {

    let mut conn = pool.acquire().await?;

    let id = sqlx::query!(
        r#"
INSERT INTO users ( email, username )
VALUES ( ?1, ?2 )
        "#,
        user.email, user.username
    )
    .execute(&mut *conn)
    .await?
    .last_insert_rowid();

    Ok(id)
}