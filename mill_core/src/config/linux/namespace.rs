use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Namespace {
    #[serde(rename = "type")]
    pub namespace_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::Namespace;
    use serde_json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({"type": "pid"}),
            serde_json::to_value(Namespace {
                namespace_type: "pid".into(),
                ..Default::default()
            })
            .unwrap()
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
                namespace_type: "pid".into(),
                path: Some("/proc/1234/ns/pid".into()),
            })
            .unwrap()
        );
    }
}
