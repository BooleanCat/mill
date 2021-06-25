use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Hypervisor {
    pub path: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<String>>,
}

impl Hypervisor {
    pub fn new(path: &str) -> Self {
        Self {
            path: String::from(path),
            parameters: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Hypervisor;
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({
            "path": "/foo/bar"
        });

        let got = serde_json::to_value(Hypervisor::new("/foo/bar")).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "path": "/foo/bar",
            "parameters": [
                "bar",
                "baz"
            ],
        });

        let got = serde_json::to_value(Hypervisor {
            parameters: Some(vec![String::from("bar"), String::from("baz")]),
            ..Hypervisor::new("/foo/bar")
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
