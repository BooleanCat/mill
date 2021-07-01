mod device;

pub use device::Device;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Resources {
    #[serde(skip_serializing_if = "Option::is_none")]
    devices: Option<Vec<Device>>,
}

#[cfg(test)]
mod tests {
    use super::{Device, Resources};
    use serde_json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({}),
            serde_json::to_value::<Resources>(Default::default()).unwrap()
        );
    }

    #[test]
    fn serialize_optional_fields() {
        assert_eq!(
            serde_json::json!({
                "devices": [{"allow": true}]
            }),
            serde_json::to_value(Resources {
                devices: Some(vec![Device {
                    allow: true,
                    ..Default::default()
                }]),
            })
            .unwrap()
        );
    }
}
