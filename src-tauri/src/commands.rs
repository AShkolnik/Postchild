// async fn get_request(
//     url: &str,
//     headers: Option<HashMap<String, String>>,
//     query_params: Option<HashMap<String, String>>,
// ) -> Result<String, reqwest::Error>

use crate::utils::http_method::HttpMethod;

#[tauri::command(rename_all = "snake_case")]
pub fn request(http_method: &str, url: &str) -> Result<String, String> {
    match http_method.parse::<HttpMethod>() {
        Ok(method) => Ok(format!("{}, {}", method, url)),
        Err(e) => Err(format!("{:?}", e)),
    }
}
