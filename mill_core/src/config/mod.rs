mod mount;
pub mod process;
mod root;
pub mod solaris;
pub mod vm;

pub use mount::Mount;
pub use process::Process;
pub use root::Root;
use serde::{Deserialize, Serialize};
pub use solaris::Solaris;
use std::collections::HashMap;
pub use vm::Vm;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub oci_version: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<Root>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<Mount>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub process: Option<Process>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    // pub hooks: Option<Hook>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<HashMap<String, String>>,
    // pub linux: Option<Linux>,
    // pub windows: Option<Windows>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solaris: Option<Solaris>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm: Option<Vm>,
}

impl Config {
    pub fn new(oci_version: &str) -> Self {
        Self {
            oci_version: String::from(oci_version),
            root: None,
            mounts: None,
            process: None,
            hostname: None,
            annotations: None,
            vm: None,
            solaris: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{vm, Config, Mount, Process, Root, Solaris, Vm};
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
            "root": {
                "path": "/foo/bar"
            },
            "mounts": [
                {
                    "destination": "/foo/bar"
                }
            ],
            "process": {
                "cwd": "/foo/bar"
            },
            "hostname": "baz",
            "annotations": {
                "com.example.gpu-cores": "2"
            },
            "vm": {
                "kernel": {
                    "path": "/bar/foo"
                }
            },
            "solaris": {}
        });

        let got = serde_json::to_value(Config {
            root: Some(Root::new("/foo/bar")),
            mounts: Some(vec![Mount::new("/foo/bar")]),
            process: Some(Process::new("/foo/bar")),
            hostname: Some(String::from("baz")),
            annotations: Some(
                [(String::from("com.example.gpu-cores"), String::from("2"))]
                    .iter()
                    .cloned()
                    .collect(),
            ),
            vm: Some(Vm::new(vm::Kernel::new("/bar/foo"))),
            solaris: Some(Solaris::new()),
            ..Config::new("0.1.0")
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
