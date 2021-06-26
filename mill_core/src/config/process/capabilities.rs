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

impl Capabilities {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::Capabilities;
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({});

        let got = serde_json::to_value(Capabilities::new()).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "effective": [
                "CAP_CHOWN"
            ],
            "bounding": [
                "CAP_AUDIT_CONTROL",
                "CAP_AUDIT_READ"
            ],
            "inheritable": [
                "CAP_AUDIT_WRITE"
            ],
            "permitted": [
                "CAP_BPF"
            ],
            "ambient": [
                "CAP_CHECKPOINT_RESTORE"
            ]
        });

        let got = serde_json::to_value(Capabilities {
            effective: Some(vec![String::from("CAP_CHOWN")]),
            bounding: Some(vec![
                String::from("CAP_AUDIT_CONTROL"),
                String::from("CAP_AUDIT_READ"),
            ]),
            inheritable: Some(vec![String::from("CAP_AUDIT_WRITE")]),
            permitted: Some(vec![String::from("CAP_BPF")]),
            ambient: Some(vec![String::from("CAP_CHECKPOINT_RESTORE")]),
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
