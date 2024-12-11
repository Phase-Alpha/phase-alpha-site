use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {

    use leptos::*;
    use redis::{aio::MultiplexedConnection, Client, RedisError};
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;
    use dotenv::dotenv;
    use std::env;
    use axum::{response::{Redirect, IntoResponse}, extract::Json, http::{HeaderMap, StatusCode}};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct UrlRequest {
        pub url: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct UrlResponse {
        pub uuid: String,
    }

    pub async fn store_url_in_redis(
        url: &str,
        con: &mut MultiplexedConnection,
    ) -> Result<String, RedisError> {
        let uuid = Uuid::new_v4().to_string();
        redis::cmd("SET")
            .arg(&uuid)
            .arg(url)
            .arg("EX")
            // Expire in 1 week
            .arg(604800)
            .query_async(con)
            .await?;

        Ok(uuid)
    }

    pub async fn fetch_url_from_redis(
        uuid: &str,
        con: &mut MultiplexedConnection,
    ) -> Result<String, RedisError> {
        let url: Option<String> = redis::cmd("GET")
            .arg(uuid)
            .query_async(con)
            .await?;

            url.ok_or_else(|| RedisError::from((redis::ErrorKind::ResponseError, "URL not found")))
    }

    async fn get_redis_client() -> Result<Client, RedisError> {
        Client::open("redis://redis:6379/")
    }

    pub async fn shorten_url(
        headers: HeaderMap,
        Json(req): Json<UrlRequest>,
    ) -> impl IntoResponse {

        dotenv().ok();

        let expected_api_key = match env::var("URL_KEY") {
            Ok(key) => key,
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Server misconfiguration: Missing API key".to_string()),
        };

        let provided_api_key = headers
            .get("x-api-key")
            .and_then(|value| value.to_str().ok());

        if provided_api_key != Some(expected_api_key.as_str()) {
            return (StatusCode::UNAUTHORIZED, "Invalid or missing API key".to_string());
        }

        let redis_client = match get_redis_client().await {
            Ok(client) => client,
            Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("Redis client error: {}", e)),
        };

        let mut con = match redis_client.get_multiplexed_async_connection().await {
            Ok(con) => con,
            Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("Redis connection error: {}", e)),
        };

        let uuid = match store_url_in_redis(&req.url, &mut con).await {
            Ok(uuid) => uuid,
            Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("Redis store error: {}", e)),
        };

        (StatusCode::OK, uuid)
    }

    pub async fn redirect(
        uuid: String,
    ) -> Result<Redirect, StatusCode> {
        let redis_client = get_redis_client().await.unwrap();
        let mut con = redis_client.get_multiplexed_async_connection().await.unwrap();

        let url = fetch_url_from_redis(&uuid, &mut con).await;

        match url {
            Ok(long_url) => Ok(Redirect::to(&long_url)),
            Err(_) => Err(StatusCode::NOT_FOUND),
        }
    }
}}

#[cfg(feature = "ssr")]
#[cfg(test)]
mod tests {
    use super::*;
    use redis::Client;

    async fn setup_redis() -> MultiplexedConnection {
        let client = Client::open("redis://127.0.0.1/").unwrap();
        client.get_multiplexed_async_connection().await.unwrap()
    }

    #[tokio::test]
    async fn test_store_url_in_redis() {
        let mut con = setup_redis().await;

        let url = "https://example.com";
        let uuid = store_url_in_redis(url, &mut con).await.unwrap();

        let stored_url: String = redis::cmd("GET")
            .arg(&uuid)
            .query_async(&mut con)
            .await
            .unwrap();
        assert_eq!(stored_url, url);
    }

    #[tokio::test]
    async fn test_fetch_url_from_redis() {
        let mut con = setup_redis().await;

        let uuid = "test-uuid";
        let url = "https://example.com";

        redis::cmd("SET")
            .arg(uuid)
            .arg(url)
            .query_async::<()>(&mut con)
            .await
            .unwrap();

        let fetched_url = fetch_url_from_redis(uuid, &mut con).await.unwrap();
        assert_eq!(fetched_url, url);
    }

    #[tokio::test]
    async fn test_fetch_url_not_found() {
        let mut con = setup_redis().await;

        let result = fetch_url_from_redis("non-existent-uuid", &mut con).await;
        assert!(result.is_err());
    }
}
