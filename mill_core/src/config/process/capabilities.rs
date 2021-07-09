use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Capabilities {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub effective: Vec<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bounding: Vec<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inheritable: Vec<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permitted: Vec<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ambient: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::Capabilities;
    use serde_json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({
                "bounding": [
                    "CAP_AUDIT_WRITE",
                    "CAP_KILL",
                    "CAP_NET_BIND_SERVICE"
                ],
                "permitted": [
                    "CAP_AUDIT_WRITE",
                    "CAP_KILL",
                    "CAP_NET_BIND_SERVICE"
                ],
                "inheritable": [
                    "CAP_AUDIT_WRITE",
                    "CAP_KILL",
                    "CAP_NET_BIND_SERVICE"
                ],
                "effective": [
                    "CAP_AUDIT_WRITE",
                    "CAP_KILL"
                ],
                "ambient": ["CAP_NET_BIND_SERVICE"]
            }),
            serde_json::to_value(Capabilities {
                bounding: vec![
                    "CAP_AUDIT_WRITE".into(),
                    "CAP_KILL".into(),
                    "CAP_NET_BIND_SERVICE".into()
                ],
                permitted: vec![
                    "CAP_AUDIT_WRITE".into(),
                    "CAP_KILL".into(),
                    "CAP_NET_BIND_SERVICE".into()
                ],
                inheritable: vec![
                    "CAP_AUDIT_WRITE".into(),
                    "CAP_KILL".into(),
                    "CAP_NET_BIND_SERVICE".into()
                ],
                effective: vec!["CAP_AUDIT_WRITE".into(), "CAP_KILL".into(),],
                ambient: vec!["CAP_NET_BIND_SERVICE".into()],
            })
            .unwrap()
        );
    }

    #[test]
    fn serialize_defaults() {
        assert_eq!(
            serde_json::json!({}),
            serde_json::to_value::<Capabilities>(Default::default()).unwrap()
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            serde_json::from_value::<Capabilities>(serde_json::json!({
                "bounding": [
                    "CAP_AUDIT_WRITE",
                    "CAP_KILL",
                    "CAP_NET_BIND_SERVICE"
                ],
                "permitted": [
                    "CAP_AUDIT_WRITE",
                    "CAP_KILL",
                    "CAP_NET_BIND_SERVICE"
                ],
                "inheritable": [
                    "CAP_AUDIT_WRITE",
                    "CAP_KILL",
                    "CAP_NET_BIND_SERVICE"
                ],
                "effective": [
                    "CAP_AUDIT_WRITE",
                    "CAP_KILL"
                ],
                "ambient": ["CAP_NET_BIND_SERVICE"]
            }))
            .unwrap(),
            Capabilities {
                bounding: vec![
                    "CAP_AUDIT_WRITE".into(),
                    "CAP_KILL".into(),
                    "CAP_NET_BIND_SERVICE".into()
                ],
                permitted: vec![
                    "CAP_AUDIT_WRITE".into(),
                    "CAP_KILL".into(),
                    "CAP_NET_BIND_SERVICE".into()
                ],
                inheritable: vec![
                    "CAP_AUDIT_WRITE".into(),
                    "CAP_KILL".into(),
                    "CAP_NET_BIND_SERVICE".into()
                ],
                effective: vec!["CAP_AUDIT_WRITE".into(), "CAP_KILL".into(),],
                ambient: vec!["CAP_NET_BIND_SERVICE".into()],
            }
        );
    }

    #[test]
    fn deserialize_defaults() {
        assert_eq!(
            serde_json::from_value::<Capabilities>(serde_json::json!({})).unwrap(),
            Capabilities {
                bounding: vec![],
                permitted: vec![],
                inheritable: vec![],
                effective: vec![],
                ambient: vec![],
            }
        );
    }
}
