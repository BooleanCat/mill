use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    #[serde(rename = "type")]
    pub device_type: String,

    pub path: String,
    pub major: i64,
    pub minor: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_mode: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid: Option<u32>,
}

impl Device {
    pub fn new(device_type: &str, path: &str, major: i64, minor: i64) -> Self {
        Self {
            device_type: String::from(device_type),
            path: String::from(path),
            major,
            minor,
            file_mode: None,
            uid: None,
            gid: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Device;
    use serde_json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({
                "type": "c",
                "path": "/dev/fuse",
                "major": 10,
                "minor": 229
            }),
            serde_json::to_value(Device::new("c", "/dev/fuse", 10, 229)).unwrap()
        );
    }

    #[test]
    fn serialize_optional_fields() {
        assert_eq!(
            serde_json::json!({
                "type": "c",
                "path": "/dev/fuse",
                "major": 10,
                "minor": 229,
                "fileMode": 438,
                "uid": 0,
                "gid": 0
            }),
            serde_json::to_value(Device {
                file_mode: Some(438),
                uid: Some(0),
                gid: Some(0),
                ..Device::new("c", "/dev/fuse", 10, 229)
            })
            .unwrap()
        );
    }
}
