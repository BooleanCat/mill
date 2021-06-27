mod namespace;

pub use namespace::Namespace;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Linux {
    #[serde(skip_serializing_if = "Option::is_none")]
    namespaces: Option<Vec<Namespace>>,
}

impl Linux {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::{Linux, Namespace};
    use serde_json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({}),
            serde_json::to_value(Linux::new()).unwrap()
        );
    }

    #[test]
    fn serialize_optional_fields() {
        assert_eq!(
            serde_json::json!({
                "namespaces": [
                    {"type": "pid"}
                ]
            }),
            serde_json::to_value(Linux {
                namespaces: Some(vec![Namespace::new("pid")]),
            })
            .unwrap()
        );
    }
}
