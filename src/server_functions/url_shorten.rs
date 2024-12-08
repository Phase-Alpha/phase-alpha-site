use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {

    use leptos::*;
    use redis::{aio::MultiplexedConnection, Client, RedisError};
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;
    use std::env;

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
            // .map_err(|e| format!("Redis query error: {e}"))?;

            url.ok_or_else(|| RedisError::from((redis::ErrorKind::ResponseError, "URL not found")))
    }

    async fn get_redis_client() -> Result<Client, RedisError> {
        Client::open("redis://127.0.0.1/")
            // .map_err(|e| RedisError::from((redis::ErrorKind::ClientError, &e.to_string())))
    }

    #[server(ShortenUrl, "/api")]
    pub async fn shorten_url(req: UrlRequest) -> Result<UrlResponse, ServerFnError> {
        use dotenv::dotenv;
        use axum::{extract::FromRequestParts, http::{Method, HeaderMap}};
        use leptos_axum::extract;

        dotenv().ok();

        let url_api: String = env::var("URL_KEY").expect("URL API Key should be set");
        let api_key = extract(|headers: HeaderMap| async move {
                headers
                    .get("x-api-key")
                    .and_then(|value| value.to_str().ok())
                    .map(|key| key.to_string())
                    .ok_or_else(|| ServerFnError::ServerError("Missing API key".to_string()))
            })
            .await.unwrap();

        if api_key != Ok(url_api) {
            log::error!("Invalid API key: {}", api_key.unwrap());
            return Err(ServerFnError::ServerError("Invalid API key.".into()));
        }

        let redis_client = get_redis_client()
            .await
            .map_err(|e| ServerFnError::ServerError(format!("{}", e.to_string())))?;
        let mut con = redis_client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| {
                log::error!("Redis connection error: {e}");
                ServerFnError::ServerError("Database connection failed.".into())
            })?;

        let uuid = store_url_in_redis(&req.url, &mut con)
            .await
            .map_err(|e| ServerFnError::ServerError(format!("{}", e.to_string())))?;

        Ok(UrlResponse { uuid })
    }

    #[server(ResolveUrl, "/api")]
    pub async fn resolve_url(uuid: String) -> Result<String, ServerFnError> {
        let redis_client = get_redis_client()
            .await?;
            // .map_err(ServerFnError::ServerError)?;
        let mut con = redis_client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| {
                log::error!("Redis connection error: {e}");
                ServerFnError::ServerError(format!("Database connection failed: {}", e.to_string()))
            })?;

        fetch_url_from_redis(&uuid, &mut con)
            .await
            .map_err(|e| ServerFnError::ServerError(format!("{}", e.to_string())))
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
