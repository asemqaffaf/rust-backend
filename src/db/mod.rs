// src/db/mod.rs
use crate::models::user::{CreateUserRequest, UpdateUserRequest, User};
use bcrypt::{hash, DEFAULT_COST};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use uuid::Uuid; // Added Utc import

pub async fn create_user(
    pool: &Pool<Postgres>,
    user_data: CreateUserRequest,
) -> Result<User, sqlx::Error> {
    let password_hash =
        hash(user_data.password.as_bytes(), DEFAULT_COST).expect("Failed to hash password");
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (email, username, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id as "id!: Uuid", 
                email as "email!: String", 
                username as "username!: String",
                password_hash as "password_hash!: String",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
        "#,
        user_data.email,
        user_data.username,
        password_hash,
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn get_user(pool: &Pool<Postgres>, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id as "id!: Uuid",
               email as "email!: String",
               username as "username!: String",
               password_hash as "password_hash!: String",
               created_at as "created_at!: DateTime<Utc>",
               updated_at as "updated_at!: DateTime<Utc>"
        FROM users 
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn update_user(
    pool: &Pool<Postgres>,
    user_id: Uuid,
    update_data: UpdateUserRequest,
) -> Result<Option<User>, sqlx::Error> {
    let current_user = get_user(pool, user_id)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;

    let password_hash = if let Some(password) = update_data.password {
        hash(password.as_bytes(), DEFAULT_COST).expect("Failed to hash password")
    } else {
        current_user.password_hash
    };

    let user = sqlx::query_as!(
        User,
        r#"
        UPDATE users
        SET 
            email = COALESCE($1, email),
            username = COALESCE($2, username),
            password_hash = $3,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $4
        RETURNING id as "id!: Uuid",
                 email as "email!: String",
                 username as "username!: String",
                 password_hash as "password_hash!: String",
                 created_at as "created_at!: DateTime<Utc>",
                 updated_at as "updated_at!: DateTime<Utc>"
        "#,
        update_data.email,
        update_data.username,
        password_hash,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn delete_user(pool: &Pool<Postgres>, user_id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM users
        WHERE id = $1
        "#,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}
