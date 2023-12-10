use axum::{extract::Json, response::{IntoResponse, Response}, http::{StatusCode, header::{CONTENT_TYPE, SET_COOKIE}}, body::Body};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use std::env::var;
use uuid::Uuid;
use crate::global::APP_CLIENTS;
use redis::AsyncCommands;


#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}


#[derive(Serialize)]
struct Error {
    err: String,
}

pub async fn login(Json(LoginRequest { username, password }): Json<LoginRequest>) -> impl IntoResponse {
    let database_url = var("DATABASE_URL").expect("DATABASE_URL is not set");
    let pool = MySqlPool::connect(&database_url).await.unwrap();

    let user = sqlx::query!(
        r#"
            SELECT id, name, password FROM User WHERE name = ?;
        "#,
        username
    ).fetch_one(&pool)
    .await;

    let user = match user {
        Ok(user) => user,
        Err(_) => return (StatusCode::UNAUTHORIZED, serde_json::to_string(&Error { err: "Invalid username".to_string() }).unwrap()).into_response(),
    };

    if password != user.password {
        return (StatusCode::UNAUTHORIZED, serde_json::to_string(&Error { err: "Invalid password".to_string() }).unwrap()).into_response();
    }

    let expiration_time = 60 * 60 * 24;
    let token = Uuid::new_v4().to_string();

    let client = APP_CLIENTS.get().unwrap().redis_client();
    let mut con = client.get_async_connection().await.unwrap();


    let _: redis::RedisResult<()> = con.set_ex(&token, user.id.to_string(), expiration_time).await;
    let _: redis::RedisResult<()> = con.set_ex(&user.id.to_string(), &token, expiration_time).await;

    let cookie = format!("token={}; Max-Age={}; Path=/; HttpOnly", token, expiration_time);

    let mut response = Response::new(Body::from(serde_json::to_string(&LoginResponse { token }).unwrap()));
    response.headers_mut().insert(SET_COOKIE, cookie.parse().unwrap());
    response.headers_mut().insert(CONTENT_TYPE, "application/json".parse().unwrap());
    response.into_response()


}
