mod device;
mod id_mapping;
mod namespace;

pub use device::Device;
pub use id_mapping::IdMapping;
pub use namespace::Namespace;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Linux {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<Namespace>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid_mappings: Option<Vec<IdMapping>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid_mappings: Option<Vec<IdMapping>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Device>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroups_path: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{Device, IdMapping, Linux, Namespace};
    use serde_json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({}),
            serde_json::to_value::<Linux>(Default::default()).unwrap()
        );
    }

    #[test]
    fn serialize_optional_fields() {
        assert_eq!(
            serde_json::json!({
                "namespaces": [
                    {"type": "pid"}
                ],
                "uidMappings": [
                    {
                        "containerID": 0,
                        "hostID": 1_000,
                        "size": 32_000
                    },
                    {
                        "containerID": 64_000,
                        "hostID": 60,
                        "size": 10
                    }
                ],
                "gidMappings": [
                    {
                        "containerID": 64_000,
                        "hostID": 60,
                        "size": 10
                    },
                    {
                        "containerID": 0,
                        "hostID": 1_000,
                        "size": 32_001
                    }
                ],
                "devices": [
                    {
                        "type": "c",
                        "path": "/dev/fuse",
                        "major": 10,
                        "minor": 229
                    }
                ],
                "cgroupsPath": "/myRuntime/myContainer"
            }),
            serde_json::to_value(Linux {
                namespaces: Some(vec![Namespace {
                    namespace_type: "pid".into(),
                    ..Default::default()
                }]),
                uid_mappings: Some(vec![
                    IdMapping {
                        container_id: 0,
                        host_id: 1_000,
                        size: 32_000
                    },
                    IdMapping {
                        container_id: 64_000,
                        host_id: 60,
                        size: 10
                    },
                ]),
                gid_mappings: Some(vec![
                    IdMapping {
                        container_id: 64_000,
                        host_id: 60,
                        size: 10
                    },
                    IdMapping {
                        container_id: 0,
                        host_id: 1_000,
                        size: 32_001
                    },
                ]),
                devices: Some(vec![Device {
                    device_type: "c".into(),
                    path: "/dev/fuse".into(),
                    major: 10,
                    minor: 229,
                    ..Default::default()
                }]),
                cgroups_path: Some("/myRuntime/myContainer".into()),
            })
            .unwrap()
        );
    }
}
