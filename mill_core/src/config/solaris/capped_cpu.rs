use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CappedCpu {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ncpus: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::CappedCpu;
    use serde_json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({}),
            serde_json::to_value::<CappedCpu>(Default::default()).unwrap()
        );
    }

    #[test]
    fn serialize_optional_fields() {
        assert_eq!(
            serde_json::json!({"ncpus": "8"}),
            serde_json::to_value(CappedCpu {
                ncpus: Some("8".into()),
            })
            .unwrap(),
        );
    }
}
