use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Kernel {
    pub path: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub initrd: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::Kernel;
    use serde_json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({"path": "/foo/bar"}),
            serde_json::to_value(Kernel {
                path: "/foo/bar".into(),
                ..Default::default()
            })
            .unwrap()
        );
    }

    #[test]
    fn serialize_optional_fields() {
        assert_eq!(
            serde_json::json!({
                "path": "/foo/bar",
                "parameters": ["bar", "baz"],
                "initrd": "/baz/bar.img"
            }),
            serde_json::to_value(Kernel {
                path: "/foo/bar".into(),
                parameters: Some(vec!["bar".into(), "baz".into()]),
                initrd: Some("/baz/bar.img".into()),
            })
            .unwrap()
        );
    }
}
