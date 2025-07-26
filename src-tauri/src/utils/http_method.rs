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
    pub fn handle(&self) {
        match self {
            HttpMethod::Get => handle_get(),
            HttpMethod::Post => handle_post(),
            HttpMethod::Put => handle_put(),
            HttpMethod::Delete => handle_delete(),
        }
    }
}

fn handle_get() { /* logic for GET */
}
fn handle_post() { /* logic for POST */
}
fn handle_put() { /* logic for PUT */
}
fn handle_delete() { /* logic for DELETE */
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
