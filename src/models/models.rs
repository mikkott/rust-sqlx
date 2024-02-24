#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub enabled: bool,
}

impl User {
    pub fn new(username: String, email: String) -> Self {
        User {
            id: 0,
            username,
            email,
            enabled: false,
        }
    }
}
