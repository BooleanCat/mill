use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Kernel {
    pub path: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub initrd: Option<String>,
}

impl Kernel {
    pub fn new(path: &str) -> Self {
        Self {
            path: String::from(path),
            parameters: None,
            initrd: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Kernel;
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({
            "path": "/foo/bar"
        });

        let got = serde_json::to_value(Kernel::new("/foo/bar")).unwrap();

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
            "initrd": "/baz/bar.img"
        });

        let got = serde_json::to_value(Kernel {
            parameters: Some(vec![String::from("bar"), String::from("baz")]),
            initrd: Some(String::from("/baz/bar.img")),
            ..Kernel::new("/foo/bar")
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
