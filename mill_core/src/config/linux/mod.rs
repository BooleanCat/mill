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

impl Linux {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::{Device, IdMapping, Linux, Namespace};
    use serde_json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({}),
            serde_json::to_value(Linux::new()).unwrap()
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
                namespaces: Some(vec![Namespace::new("pid")]),
                uid_mappings: Some(vec![
                    IdMapping::new(0, 1_000, 32_000),
                    IdMapping::new(64_000, 60, 10)
                ]),
                gid_mappings: Some(vec![
                    IdMapping::new(64_000, 60, 10),
                    IdMapping::new(0, 1_000, 32_001)
                ]),
                devices: Some(vec![Device::new("c", "/dev/fuse", 10, 229)]),
                cgroups_path: Some(String::from("/myRuntime/myContainer")),
            })
            .unwrap()
        );
    }
}
