use crate::models::User;
use sqlx::sqlite::SqlitePool;

pub async fn add_user(pool: &SqlitePool, user: &User) -> anyhow::Result<i64> {
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

pub async fn list_users<T>(pool: &SqlitePool) -> anyhow::Result<Vec<User>> {
    let users = sqlx::query_as::<_, User>(r#"SELECT * FROM users"#)
        .fetch_all(pool)
        .await?;

    Ok(users)
}

pub async fn delete_user(pool: &SqlitePool, user: &User) -> anyhow::Result<u64> {
    let mut conn = pool.acquire().await?;
    let id = sqlx::query!(r#"DELETE FROM users WHERE id = ?1"#, user.id)
        .execute(&mut *conn)
        .await?
        .rows_affected();

    Ok(id)
}
