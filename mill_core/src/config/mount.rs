use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Mount {
    pub destination: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,

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
            serde_json::json!({"destination": "/foo/bar"}),
            serde_json::to_value(Mount {
                destination: "/foo/bar".into(),
                ..Default::default()
            })
            .unwrap()
        );
    }

    #[test]
    fn serialize_optional_fields() {
        assert_eq!(
            serde_json::json!({
                "destination": "/foo/bar",
                "source": "/bar/baz",
                "options": ["foo", "bar"],
                "type": "tmpfs"
            }),
            serde_json::to_value(Mount {
                destination: "/foo/bar".into(),
                source: Some("/bar/baz".into()),
                options: Some(vec!["foo".into(), "bar".into()]),
                mount_type: Some("tmpfs".into()),
            })
            .unwrap()
        );
    }
}
