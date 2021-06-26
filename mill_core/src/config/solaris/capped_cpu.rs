use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CappedCpu {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ncpus: Option<String>,
}

impl CappedCpu {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::CappedCpu;
    use serde_json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({}),
            serde_json::to_value(CappedCpu::new()).unwrap()
        );
    }

    #[test]
    fn serialize_optional_fields() {
        assert_eq!(
            serde_json::json!({"ncpus": "8"}),
            serde_json::to_value(CappedCpu {
                ncpus: Some(String::from("8")),
            })
            .unwrap(),
        );
    }
}
