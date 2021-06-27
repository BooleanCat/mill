use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IdMapping {
    #[serde(rename = "containerID")]
    pub container_id: u32,

    #[serde(rename = "hostID")]
    pub host_id: u32,

    pub size: u32,
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
            serde_json::to_value(IdMapping {
                container_id: 0,
                host_id: 1_000,
                size: 32_000
            })
            .unwrap()
        );
    }
}
