use serde::Serialize;

#[derive(Serialize)]
pub struct HttpResponse {
    pub body: String,
    pub headers: Vec<(String, String)>,
}
