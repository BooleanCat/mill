use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Mount {
    pub destination: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub mount_type: Option<String>,
}

impl Mount {
    pub fn new(destination: &str) -> Self {
        Self {
            destination: String::from(destination),
            source: None,
            options: None,
            mount_type: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Mount;
    use serde_json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({"destination": "/foo/bar"}),
            serde_json::to_value(Mount::new("/foo/bar")).unwrap()
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
                destination: String::from("/foo/bar"),
                source: Some(String::from("/bar/baz")),
                options: Some(vec![String::from("foo"), String::from("bar")]),
                mount_type: Some(String::from("tmpfs")),
            })
            .unwrap()
        );
    }
}
