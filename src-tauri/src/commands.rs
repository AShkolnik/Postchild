// async fn get_request(
//     url: &str,
//     headers: Option<HashMap<String, String>>,
//     query_params: Option<HashMap<String, String>>,
// ) -> Result<String, reqwest::Error>

use crate::utils::http_method::HttpMethod;

use serde_json::json;
use tauri::command;

#[command(rename_all = "snake_case")]
pub async fn request(http_method: &str, url: &str) -> Result<serde_json::Value, String> {
    let url = match reqwest::Url::parse(url) {
        Ok(u) => u,
        Err(e) => return Err(format!("Invalid URL: {}", e)),
    };

    let method = match http_method.parse::<HttpMethod>() {
        Ok(m) => m,
        Err(e) => return Err(format!("Invalid HTTP method: {:?}", e)),
    };

    let client = reqwest::Client::new();

    match method {
        HttpMethod::Get => {
            let response = match client.get(url).send().await {
                Ok(res) => res,
                Err(e) => return Err(format!("Request error: {}", e)),
            };

            let headers = response.headers().iter().map(|(k, v)| {
                ( k.to_string(), v.to_str().unwrap_or("").to_string())
}).collect::<Vec<_>>();

            let body = match response.text().await {
                Ok(body) => body,
                Err(e) => return Err(format!("Failed to read response: {}", e)),
            };

            Ok(json!({"body": body, "headers": headers}))
        },

        unsupported => Err(format!("Unsupported method: {:?}", unsupported)),
    }
}
