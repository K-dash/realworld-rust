use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
#[allow(non_snake_case)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub user: UserInfo,
}

// impl User {
//     pub async fn create(pool: &PgPool, new_user: User) -> sqlx::Result<User> {
//         sqlx::query_as!(
//             User,
//             r#"
//             INSERT INTO users (id, username, email, bio, image, token, hashed_password)
//             VALUES ($1, $2, $3, $4, $5, $6, $7)
//             RETURNING id, username, email, bio, image, token, hashed_password
//             "#,
//             Uuid::new_v4(),
//             new_user.username,
//             new_user.email,
//             new_user.bio,
//             new_user.image,
//             new_user.token,
//             new_user.hashed_password
//         )
//         .fetch_one(pool)
//         .await
//     }
// }
