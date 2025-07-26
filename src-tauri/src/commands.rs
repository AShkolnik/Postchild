// async fn get_request(
//     url: &str,
//     headers: Option<HashMap<String, String>>,
//     query_params: Option<HashMap<String, String>>,
// ) -> Result<String, reqwest::Error>

#[tauri::command(rename_all = "snake_case")]
pub fn request(http_method: &str, url: &str) {
    println!("GET REQUEST: {} with method: {}", url, http_method);
}
