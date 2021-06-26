use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub path: String,
    pub format: String,
}

impl Image {
    pub fn new(path: &str, format: &str) -> Self {
        Self {
            path: String::from(path),
            format: String::from(format),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Image;
    use serde_json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({
                "path": "/foo/bar",
                "format": "raw"
            }),
            serde_json::to_value(Image::new("/foo/bar", "raw")).unwrap()
        );
    }
}
