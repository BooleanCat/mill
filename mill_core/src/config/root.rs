use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Root {
    pub path: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly: Option<bool>,
}

impl Root {
    pub fn new(path: &str) -> Self {
        Self {
            path: String::from(path),
            readonly: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Root;
    use serde_json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({"path": "/foo/bar"}),
            serde_json::to_value(Root::new("/foo/bar")).unwrap()
        );
    }

    #[test]
    fn serialize_optional_fields() {
        assert_eq!(
            serde_json::json!({
                "path": "/foo/bar",
                "readonly": true
            }),
            serde_json::to_value(Root {
                readonly: Some(true),
                ..Root::new("/foo/bar")
            })
            .unwrap()
        );
    }
}
