use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Image {
    pub path: String,
    pub format: String,
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
            serde_json::to_value(Image {
                path: "/foo/bar".into(),
                format: "raw".into()
            })
            .unwrap()
        );
    }
}
