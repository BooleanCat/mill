use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Root {
    pub path: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::Root;
    use serde_json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({"path": "/foo/bar"}),
            serde_json::to_value(Root {
                path: "/foo/bar".into(),
                ..Default::default()
            })
            .unwrap()
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
                path: "/foo/bar".into(),
                readonly: Some(true),
            })
            .unwrap()
        );
    }
}
