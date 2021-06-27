use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Capabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inheritable: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub permitted: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ambient: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::Capabilities;
    use serde_json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({}),
            serde_json::to_value::<Capabilities>(Default::default()).unwrap()
        );
    }

    #[test]
    fn serialize_optional_fields() {
        assert_eq!(
            serde_json::json!({
                "effective": ["CAP_CHOWN"],
                "bounding": ["CAP_AUDIT_CONTROL", "CAP_AUDIT_READ"],
                "inheritable": ["CAP_AUDIT_WRITE"],
                "permitted": ["CAP_BPF"],
                "ambient": ["CAP_CHECKPOINT_RESTORE"]
            }),
            serde_json::to_value(Capabilities {
                effective: Some(vec!["CAP_CHOWN".into()]),
                bounding: Some(vec!["CAP_AUDIT_CONTROL".into(), "CAP_AUDIT_READ".into()]),
                inheritable: Some(vec!["CAP_AUDIT_WRITE".into()]),
                permitted: Some(vec!["CAP_BPF".into()]),
                ambient: Some(vec!["CAP_CHECKPOINT_RESTORE".into()]),
            })
            .unwrap()
        );
    }
}
