use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IdMapping {
    #[serde(rename = "containerID")]
    pub container_id: u32,

    #[serde(rename = "hostID")]
    pub host_id: u32,

    pub size: u32,
}

impl IdMapping {
    pub fn new(container_id: u32, host_id: u32, size: u32) -> Self {
        Self {
            container_id,
            host_id,
            size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IdMapping;
    use serde_json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({
                "containerID": 0,
                "hostID": 1_000,
                "size": 32_000
            }),
            serde_json::to_value(IdMapping::new(0, 1_000, 32_000)).unwrap()
        );
    }
}
