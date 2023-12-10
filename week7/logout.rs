use axum::{response::IntoResponse, http::{StatusCode, header::{HeaderMap, COOKIE},}};
use redis::AsyncCommands;
use crate::global::APP_CLIENTS;

pub async fn logout(headers: HeaderMap) -> impl IntoResponse {
    
    let cookie = match headers.get(COOKIE) {
        Some(cookie) => cookie,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let token = cookie.to_str().unwrap().split("=").collect::<Vec<&str>>()[1];

    let client = APP_CLIENTS.get().unwrap().redis_client();
    let mut con = client.get_async_connection().await.unwrap();

    let user_id : redis::RedisResult<String> = con.get(token).await;


    if let Ok(user_id) = user_id {
        let _: redis::RedisResult<()> = con.del(user_id).await;
        let _: redis::RedisResult<()> = con.del(token).await;
        (StatusCode::OK, "OK").into_response()
    }
    else {
        (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
    }

}