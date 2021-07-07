use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Mount {
    pub destination: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<String>,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub mount_type: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::Mount;
    use serde_json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({
                "destination": "/tmp",
                "type": "tmpfs",
                "source": "tmpfs",
                "options": ["nosuid", "strictatime", "mode=755", "size=65536k"]
            }),
            serde_json::to_value(Mount {
                destination: "/tmp".into(),
                mount_type: Some("tmpfs".into()),
                source: Some("tmpfs".into()),
                options: vec![
                    "nosuid".into(),
                    "strictatime".into(),
                    "mode=755".into(),
                    "size=65536k".into()
                ]
            })
            .unwrap()
        );
    }

    #[test]
    fn serialize_defaults() {
        assert_eq!(
            serde_json::json!({"destination": "/tmpfs"}),
            serde_json::to_value(Mount {
                destination: "/tmpfs".into(),
                ..Default::default()
            })
            .unwrap()
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            serde_json::from_value::<Mount>(serde_json::json!({
                "destination": "/tmp",
                "type": "tmpfs",
                "source": "tmpfs",
                "options": ["nosuid", "strictatime", "mode=755", "size=65536k"]
            }))
            .unwrap(),
            Mount {
                destination: "/tmp".into(),
                mount_type: Some("tmpfs".into()),
                source: Some("tmpfs".into()),
                options: vec![
                    "nosuid".into(),
                    "strictatime".into(),
                    "mode=755".into(),
                    "size=65536k".into()
                ],
            }
        );
    }

    #[test]
    fn deserialize_defaults() {
        assert_eq!(
            serde_json::from_value::<Mount>(serde_json::json!({"destination": "/tmpfs"})).unwrap(),
            Mount {
                destination: "/tmpfs".into(),
                mount_type: None,
                source: None,
                options: vec![],
            }
        );
    }
}
