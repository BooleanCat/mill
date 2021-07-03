use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Memory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel: Option<i64>,

    #[serde(rename = "kernelTCP", skip_serializing_if = "Option::is_none")]
    pub kernel_tcp: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub swappiness: Option<u64>,

    #[serde(rename = "disableOOMKiller", skip_serializing_if = "Option::is_none")]
    pub disable_oom_killer: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_hierarchy: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::Memory;
    use serde_json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({}),
            serde_json::to_value::<Memory>(Default::default()).unwrap()
        );
    }

    #[test]
    fn serialize_optional_fields() {
        assert_eq!(
            serde_json::json!({
                "limit": 536870912,
                "reservation": 536870913,
                "swap": 536870914,
                "kernel": -1,
                "kernelTCP": -1,
                "swappiness": 0,
                "disableOOMKiller": true,
                "useHierarchy": true
            }),
            serde_json::to_value(Memory {
                limit: Some(536870912),
                reservation: Some(536870913),
                swap: Some(536870914),
                kernel: Some(-1),
                kernel_tcp: Some(-1),
                swappiness: Some(0),
                disable_oom_killer: Some(true),
                use_hierarchy: Some(true),
            })
            .unwrap()
        );
    }
}
