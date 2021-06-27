use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Hypervisor {
    pub path: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::Hypervisor;
    use serde_json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({"path": "/foo/bar"}),
            serde_json::to_value(Hypervisor {
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
                "parameters": ["bar", "baz"],
            }),
            serde_json::to_value(Hypervisor {
                path: "/foo/bar".into(),
                parameters: Some(vec!["bar".into(), "baz".into()]),
            })
            .unwrap()
        );
    }
}
