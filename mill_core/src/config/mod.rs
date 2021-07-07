mod mount;
pub mod process;
mod root;

pub use mount::Mount;
pub use process::Process;
pub use root::Root;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub oci_version: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<Root>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mounts: Vec<Mount>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub process: Option<Process>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    // // pub hooks: Option<Hook>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub annotations: HashMap<String, String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub linux: Option<Linux>,

    // // pub windows: Option<Windows>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub solaris: Option<Solaris>,

    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub vm: Option<Vm>,
}

#[cfg(test)]
mod tests {
    use super::{Config, Mount, Process, Root};
    use serde_json;
    use std::collections::HashMap;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({
                "ociVersion": "0.1.0",
                "root": {"path": "rootfs"},
                "mounts": [{"destination": "/tmpfs"}],
                "process": {"cwd": "/root"},
                "hostname": "neptune",
                "annotations": {"com.example.gpu-cores": "2"}
            }),
            serde_json::to_value(Config {
                oci_version: "0.1.0".into(),
                root: Some(Root {
                    path: "rootfs".into(),
                    ..Default::default()
                }),
                mounts: vec![Mount {
                    destination: "/tmpfs".into(),
                    ..Default::default()
                }],
                process: Some(Process {
                    cwd: "/root".into(),
                    ..Default::default()
                }),
                hostname: Some("neptune".into()),
                annotations: [("com.example.gpu-cores".into(), "2".into())]
                    .iter()
                    .cloned()
                    .collect(),
            })
            .unwrap()
        );
    }

    #[test]
    fn serialize_defaults() {
        assert_eq!(
            serde_json::json!({"ociVersion": "0.1.0"}),
            serde_json::to_value(Config {
                oci_version: "0.1.0".into(),
                ..Default::default()
            })
            .unwrap()
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            serde_json::from_value::<Config>(serde_json::json!({
                "ociVersion": "0.1.0",
                "root": {"path": "rootfs"},
                "mounts": [{"destination": "/tmpfs"}],
                "process": {"cwd": "/root"},
                "hostname": "neptune",
                "annotations": {"com.example.gpu-cores": "2"}
            }))
            .unwrap(),
            Config {
                oci_version: "0.1.0".into(),
                root: Some(Root {
                    path: "rootfs".into(),
                    ..Default::default()
                }),
                mounts: vec![Mount {
                    destination: "/tmpfs".into(),
                    ..Default::default()
                }],
                process: Some(Process {
                    cwd: "/root".into(),
                    ..Default::default()
                }),
                hostname: Some("neptune".into()),
                annotations: [("com.example.gpu-cores".into(), "2".into())]
                    .iter()
                    .cloned()
                    .collect(),
            }
        );
    }

    #[test]
    fn deserialize_defaults() {
        assert_eq!(
            serde_json::from_value::<Config>(serde_json::json!({"ociVersion": "0.1.0"})).unwrap(),
            Config {
                oci_version: "0.1.0".into(),
                root: None,
                mounts: vec![],
                process: None,
                hostname: None,
                annotations: HashMap::new(),
            }
        );
    }
}
