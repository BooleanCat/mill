use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Device {
    pub allow: bool,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::Device;
    use serde_json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({"allow": true}),
            serde_json::to_value(Device {
                allow: true,
                ..Default::default()
            })
            .unwrap()
        );
    }

    #[test]
    fn serialize_optional_fields() {
        assert_eq!(
            serde_json::json!({
                "allow": true,
                "type": "c",
                "major": 10,
                "minor": 229,
                "access": "rw"
            }),
            serde_json::to_value(Device {
                allow: true,
                device_type: Some("c".into()),
                major: Some(10),
                minor: Some(229),
                access: Some("rw".into()),
            })
            .unwrap()
        );
    }
}
