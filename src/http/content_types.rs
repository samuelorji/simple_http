#[derive(Debug)]
pub enum ContentType {
    json,
    html,
    text
}

impl ContentType {
    pub fn as_mime_type(&self) -> &str {
        match self {
            ContentType::json => "application/json",
            ContentType::html => "text/html",
            ContentType::text => "text/plain"
        }
    }
}