use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Linux {}

impl Linux {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::Linux;
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
            serde_json::json!({}),
            serde_json::to_value(Linux {}).unwrap()
        );
    }
}
