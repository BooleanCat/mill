use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub path: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

impl Root {
    pub fn new(path: &str) -> Self {
        Self {
            path: String::from(path),
            read_only: None,
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
                "readOnly": true
            }),
            serde_json::to_value(Root {
                read_only: Some(true),
                ..Root::new("/foo/bar")
            })
            .unwrap()
        );
    }
}
