use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};

// struct for receiving user input
#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub user_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub id_token: String,
}

// struct to represent database record
#[derive(Serialize, FromRow)]
pub struct User {
    pub user_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub session_token: String,
}

// Implementation for User struct, functions for read/write/update/delete
// from db
//
// Functions are used in routes.rs
impl User {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<User>> {
        let mut users = vec![];
        let recs = sqlx::query!(
            r#"
                SELECT user_id, first_name, last_name, session_token
                    FROM users
                ORDER BY user_id
            "#
        )
        .fetch_all(pool)
        .await?;

        for rec in recs {
            users.push(User {
                user_id: rec.user_id,
                first_name: rec.first_name,
                last_name: rec.last_name,
                session_token: rec.session_token,
            });
        }

        Ok(users)
    }

    pub async fn find_by_id(id: i32, pool: &PgPool) -> Result<User> {
        let rec = sqlx::query!(
            r#"
                    SELECT * FROM users WHERE user_id = $1
                "#,
            id
        )
        .fetch_one(&*pool)
        .await?;

        Ok(User {
            user_id: rec.user_id,
            first_name: rec.first_name,
            last_name: rec.last_name,
            session_token: rec.session_token,
        })
    }

    pub async fn create(user: UserRequest, pool: &PgPool) -> Result<User> {
        let random_bytes: Vec<u8> = (0..64).map(|_| rand::random::<u8>()).collect();
        let encoded_bytes = hex::encode(random_bytes.to_vec());
        let mut tx = pool.begin().await?;
        let user = sqlx::query(
            "INSERT INTO users (user_id, first_name, last_name, session_token)
        VALUES ($1, $2, $3, $4) RETURNING user_id, first_name, last_name, session_token",
        )
        .bind(user.user_id)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(encoded_bytes)
        .map(|row: PgRow| User {
            user_id: row.get(0),
            first_name: row.get(1),
            last_name: row.get(2),
            session_token: row.get(3),
        })
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(user)
    }

    pub async fn update(id: i32, user: UserRequest, pool: &PgPool) -> Result<User> {
        let mut tx = pool.begin().await.unwrap();
        let user = sqlx::query(
            "UPDATE users SET user_id = $1, first_name = $2,
        last_name = $3 WHERE user_id = $1 RETURNING user_id, first_name, last_name, session_token",
        )
        .bind(id)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .map(|row: PgRow| User {
            user_id: row.get(0),
            first_name: row.get(1),
            last_name: row.get(2),
            session_token: row.get(3),
        })
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await.unwrap();
        Ok(user)
    }

    pub async fn delete(id: i32, pool: &PgPool) -> Result<u64> {
        let mut tx = pool.begin().await?;
        let deleted = sqlx::query("DELETE FROM users WHERE user_id = $1")
            .bind(id)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(deleted.rows_affected())
    }
}
