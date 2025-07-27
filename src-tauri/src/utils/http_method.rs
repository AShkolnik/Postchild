use serde_json::json;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

impl HttpMethod {
    pub async fn handle(&self, url: reqwest::Url, body: Option<String>) -> Result<serde_json::Value, String> {
        match self {
            HttpMethod::Get => handle_get(url).await,
            HttpMethod::Post => handle_post(url, body.clone().unwrap_or_default()).await,
            HttpMethod::Put => handle_put(url, body.clone().unwrap_or_default()).await,
            HttpMethod::Delete => handle_delete(url).await,
        }
    }
}

async fn handle_get(url: reqwest::Url) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::new();

    let response = match client.get(url).send().await {
        Ok(res) => res,
        Err(e) => return Err(format!("Request error: {}", e)),
    };

    let status = response.status().as_u16();
    let headers = response
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect::<std::collections::HashMap<_, _>>();

    let body = match response.text().await {
        Ok(body) => body,
        Err(e) => return Err(format!("Failed to read response: {}", e)),
    };

    Ok(json!({"status": status, "headers": headers, "body": body}))
}

async fn handle_post(url: reqwest::Url, body: String) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::new();

    let response = match client.post(url).body(body).send().await {
        Ok(res) => res,
        Err(e) => return Err(format!("Request error: {}", e)),
    };

    let status = response.status().as_u16();
    let headers = response
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect::<std::collections::HashMap<_, _>>();

    let body = match response.text().await {
        Ok(body) => body,
        Err(e) => return Err(format!("Failed to read response: {}", e)),
    };


    Ok(json!({"status": status, "headers": headers, "body": body}))
}

async fn handle_put(url: reqwest::Url, body: String) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::new();

    let response = match client.put(url).body(body).send().await {
        Ok(res) => res,
        Err(e) => return Err(format!("Request error: {}", e)),
    };

    let status = response.status().as_u16();
    let headers = response
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect::<std::collections::HashMap<_, _>>();

    let body = match response.text().await {
        Ok(body) => body,
        Err(e) => return Err(format!("Failed to read response: {}", e)),
    };


    Ok(json!({"status": status, "headers": headers, "body": body}))
}

async fn handle_delete(url: reqwest::Url) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::new();

    let response = match client.delete(url).send().await {
        Ok(res) => res,
        Err(e) => return Err(format!("Request error: {}", e)),
    };

    let status = response.status().as_u16();
    let headers = response
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect::<std::collections::HashMap<_, _>>();

    let body = match response.text().await {
        Ok(body) => body,
        Err(e) => return Err(format!("Failed to read response: {}", e)),
    };


    Ok(json!({"status": status, "headers": headers, "body": body}))
}

impl FromStr for HttpMethod {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "GET" => Ok(HttpMethod::Get),
            "POST" => Ok(HttpMethod::Post),
            "PUT" => Ok(HttpMethod::Put),
            "DELETE" => Ok(HttpMethod::Delete),
            _ => Err(()),
        }
    }
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
        };
        write!(f, "{}", s)
    }
}
