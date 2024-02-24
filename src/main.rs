use self::models::models::User;
use sqlx::sqlite::SqlitePool;
use std::env;
pub mod models;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    let foo = add_user(&pool, &User::new("foo".to_string(), "foo@bar".to_string())).await?;
    println!("Added user {}", foo);

    let bar = list_users::<User>(&pool).await?;

    for e in &bar {
        println!("Deleting: {:?}", e);
        delete_user(&pool, e).await?;
    }

    let bar = list_users::<User>(&pool).await?;

    for e in bar {
        println!("Users:");
        println!("{:?}", e);
    }

    Ok(())
}

async fn add_user(pool: &SqlitePool, user: &User) -> anyhow::Result<i64> {
    let mut conn = pool.acquire().await?;
    let id = sqlx::query!(
        r#"
            INSERT INTO users ( email, username )
            VALUES ( ?1, ?2 )
        "#,
        user.email,
        user.username
    )
    .execute(&mut *conn)
    .await?
    .last_insert_rowid();

    Ok(id)
}

async fn list_users<T>(pool: &SqlitePool) -> anyhow::Result<Vec<User>> {
    let users = sqlx::query_as::<_, User>(r#"SELECT * FROM users"#)
        .fetch_all(pool)
        .await?;

    Ok(users)
}

async fn delete_user(pool: &SqlitePool, user: &User) -> anyhow::Result<u64> {
    let mut conn = pool.acquire().await?;
    let id = sqlx::query!(r#"DELETE FROM users WHERE id = ?1"#, user.id)
        .execute(&mut *conn)
        .await?
        .rows_affected();

    Ok(id)
}
