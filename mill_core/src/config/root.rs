use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub path: String,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub readonly: bool,
}

#[cfg(test)]
mod tests {
    use super::Root;
    use serde_json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({
                "path": "rootfs",
                "readonly": true
            }),
            serde_json::to_value(Root {
                path: "rootfs".into(),
                readonly: true,
            })
            .unwrap()
        );
    }

    #[test]
    fn serialize_defaults() {
        assert_eq!(
            serde_json::json!({"path": "rootfs"}),
            serde_json::to_value(Root {
                path: "rootfs".into(),
                ..Default::default()
            })
            .unwrap()
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            serde_json::from_value::<Root>(serde_json::json!({
                "path": "rootfs",
                "readonly": true,
            }))
            .unwrap(),
            Root {
                path: "rootfs".into(),
                readonly: true,
            }
        );
    }

    #[test]
    fn deserialize_defaults() {
        assert_eq!(
            serde_json::from_value::<Root>(serde_json::json!({"path": "rootfs"})).unwrap(),
            Root {
                path: "rootfs".into(),
                readonly: false,
            }
        );
    }
}
