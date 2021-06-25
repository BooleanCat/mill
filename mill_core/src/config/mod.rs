use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub oci_version: String,
    // pub root: Option<Root>,
    // pub mounts: Option<Vec<Mount>>,
    // pub process: Option<Process>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    // pub hooks: Option<Hook>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<HashMap<String, String>>,
    // pub linux: Option<Linux>,
    // pub windows: Option<Windows>,
    // pub solaris: Option<Solaris>,
    // pub vm: Option<Vm>,
}

impl Config {
    pub fn new(oci_version: &str) -> Self {
        Self {
            oci_version: String::from(oci_version),
            hostname: None,
            annotations: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({
            "ociVersion": "0.1.0"
        });

        let got = serde_json::to_value(Config::new("0.1.0")).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "ociVersion": "0.1.0",
            "hostname": "baz",
            "annotations": {
                "com.example.gpu-cores": "2"
            }
        });

        let got = serde_json::to_value(Config {
            hostname: Some(String::from("baz")),
            annotations: Some(
                [(String::from("com.example.gpu-cores"), String::from("2"))]
                    .iter()
                    .cloned()
                    .collect(),
            ),
            ..Config::new("0.1.0")
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
