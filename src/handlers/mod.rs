// src/handlers/mod.rs
use crate::models::user::{CreateUserRequest, UpdateUserRequest};
use actix_web::{web, HttpResponse, Responder};
use sqlx::Error;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/users")
            .route("", web::post().to(create_user))
            .route("{id}", web::get().to(get_user))
            .route("/{id}", web::put().to(update_user))
            .route("/{id}", web::delete().to(delete_user)),
    );
}

async fn create_user(
    pool: web::Data<Pool<Postgres>>,
    user_data: web::Json<CreateUserRequest>,
) -> impl Responder {
    match crate::db::create_user(&pool, user_data.into_inner()).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(Error::Database(db_err)) if db_err.code().map_or(false, |code| code == "23505") => {
           return  HttpResponse::BadRequest().json(db_err.to_string())
        }
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

async fn get_user(pool: web::Data<Pool<Postgres>>, id: web::Path<Uuid>) -> impl Responder {
    match crate::db::get_user(&pool, id.clone()).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn update_user(
    pool: web::Data<Pool<Postgres>>,
    id: web::Path<Uuid>,
    user_data: web::Json<UpdateUserRequest>,
) -> impl Responder {
    match crate::db::update_user(&pool, id.into_inner(), user_data.into_inner()).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn delete_user(pool: web::Data<Pool<Postgres>>, id: web::Path<Uuid>) -> impl Responder {
    match crate::db::delete_user(&pool, id.into_inner()).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
