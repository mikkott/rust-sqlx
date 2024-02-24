use self::models::User;

use sqlx::sqlite::SqlitePool;
use std::env;
pub mod controllers;
pub mod models;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    let foo =
        controllers::add_user(&pool, &User::new("foo".to_string(), "foo@bar".to_string())).await?;
    println!("Added user {}", foo);

    let bar = controllers::list_users::<User>(&pool).await?;

    for e in &bar {
        println!("Deleting: {:?}", e);
        controllers::delete_user(&pool, e).await?;
    }

    let bar = controllers::list_users::<User>(&pool).await?;

    for e in bar {
        println!("Users:");
        println!("{:?}", e);
    }

    Ok(())
}
