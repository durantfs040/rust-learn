use axum::{response::{IntoResponse, Response}, http::{StatusCode, header::{HeaderMap, COOKIE, SET_COOKIE, CONTENT_TYPE},}, body::Body};
use redis::AsyncCommands;
use serde::Serialize;
use crate::global::APP_CLIENTS;

#[derive(Serialize)]
struct Ok {
    success: bool,
}

#[derive(Serialize)]
struct Error {
    err: String,
}

pub async fn logout(headers: HeaderMap) -> impl IntoResponse {
    
    let cookie = match headers.get(COOKIE) {
        Some(cookie) => cookie,
        None => return (StatusCode::UNAUTHORIZED, serde_json::to_string(&Error { err: "Unauthorized".to_string() }).unwrap()).into_response(),
    };

    let token = cookie.to_str().unwrap().split("=").collect::<Vec<&str>>()[1];

    let client = APP_CLIENTS.get().unwrap().redis_client();
    let mut con = client.get_async_connection().await.unwrap();

    let user_id : redis::RedisResult<String> = con.get(token).await;


    if let Ok(user_id) = user_id {
        let _: redis::RedisResult<()> = con.del(user_id).await;
        let _: redis::RedisResult<()> = con.del(token).await;
        // set header to application/json
        let mut response = Response::new(Body::from(serde_json::to_string(&Ok { success: true }).unwrap()));
        response.headers_mut().insert(CONTENT_TYPE, "application/json".parse().unwrap());
        response.into_response()

    }
    else {
        (StatusCode::UNAUTHORIZED, serde_json::to_string(&Error { err: "Unauthorized".to_string() }).unwrap()).into_response()
    }

}