use crate::utils::http_method::HttpMethod;

use tauri::command;

#[command(rename_all = "snake_case")]
pub async fn request(http_method: &str, url: &str, body: Option<String>) -> Result<serde_json::Value, String> {
    let url = match reqwest::Url::parse(url) {
        Ok(u) => u,
        Err(e) => return Err(format!("Invalid URL: {}", e)),
    };

    let method = match http_method.parse::<HttpMethod>() {
        Ok(m) => m,
        Err(e) => return Err(format!("Invalid HTTP method: {:?}", e)),
    };


    match method {
        HttpMethod::Get => method.handle(url, None).await,
        HttpMethod::Post => method.handle(url, body).await,
        HttpMethod::Put => method.handle(url, body).await,
        HttpMethod::Delete => method.handle(url, None).await,
    }
}
