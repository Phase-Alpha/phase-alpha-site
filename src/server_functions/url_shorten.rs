use leptos::*;
use redis::{aio::MultiplexedConnection, Client};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
) -> Result<String, String> {
    let uuid = Uuid::new_v4().to_string();
    redis::cmd("SET")
        .arg(&uuid)
        .arg(url)
        .arg("EX")
        // Expire in 1 week
        .arg(604800)
        .query_async(con)
        .await
        .map_err(|e| format!("Redis query error: {e}"))?;

    Ok(uuid)
}

pub async fn fetch_url_from_redis(
    uuid: &str,
    con: &mut MultiplexedConnection,
) -> Result<String, String> {
    let url: Option<String> = redis::cmd("GET")
        .arg(uuid)
        .query_async(con)
        .await
        .map_err(|e| format!("Redis query error: {e}"))?;

    url.ok_or_else(|| "URL not found".to_string())
}

async fn get_redis_client() -> Result<Client, String> {
    Client::open("redis://127.0.0.1/").map_err(|e| format!("Failed to connect to Redis: {e}"))
}

#[server(ShortenUrl, "/api")]
pub async fn shorten_url(req: UrlRequest) -> Result<UrlResponse, ServerFnError> {
    use dotenv::dotenv;

    dotenv.ok();

    let URL_API: String = env::var("URL_KEY").expect("URL API Key should be set");
    let headers = request_headers();
    let api_key = headers
        .get("x-api-key")
        .map(|h| h.to_str().unwrap_or(""))
        .unwrap_or("");

    if api_key != expected_api_key {
        log::error!("Invalid API key: {}", api_key);
        return Err(ServerFnError::Unauthorized("Invalid API key.".into()));
    }

    let redis_client = get_redis_client()
        .await
        .map_err(ServerFnError::ServerError)?;
    let mut con = redis_client
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| {
            log::error!("Redis connection error: {e}");
            ServerFnError::ServerError("Database connection failed.".into())
        })?;

    let uuid = store_url_in_redis(&req.url, &mut con)
        .await
        .map_err(|e| ServerFnError::ServerError(e.into()))?;

    Ok(UrlResponse { uuid })
}

#[server(ResolveUrl, "/api")]
pub async fn resolve_url(uuid: String) -> Result<String, ServerFnError> {
    let redis_client = get_redis_client()
        .await
        .map_err(ServerFnError::ServerError)?;
    let mut con = redis_client
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| {
            log::error!("Redis connection error: {e}");
            ServerFnError::ServerError("Database connection failed.".into())
        })?;

    fetch_url_from_redis(&uuid, &mut con)
        .await
        .map_err(|e| ServerFnError::ServerError(e.into()))
}

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
        assert_eq!(result.unwrap_err(), "URL not found");
    }
}
