use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Namespace {
    #[serde(rename = "type")]
    pub namespace_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

impl Namespace {
    pub fn new(namespace_type: &str) -> Self {
        Self {
            namespace_type: String::from(namespace_type),
            path: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Namespace;
    use serde_json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({"type": "pid"}),
            serde_json::to_value(Namespace::new("pid")).unwrap()
        );
    }

    #[test]
    fn serialize_optional_fields() {
        assert_eq!(
            serde_json::json!({
                "type": "pid",
                "path": "/proc/1234/ns/pid"
            }),
            serde_json::to_value(Namespace {
                path: Some(String::from("/proc/1234/ns/pid")),
                ..Namespace::new("pid")
            })
            .unwrap()
        );
    }
}
