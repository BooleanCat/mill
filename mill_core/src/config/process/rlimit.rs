use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Rlimit {
    #[serde(rename = "type")]
    pub rlimit_type: String,

    pub soft: u64,
    pub hard: u64,
}

impl Rlimit {
    pub fn new(rlimit_type: &str, soft: u64, hard: u64) -> Self {
        Self {
            rlimit_type: String::from(rlimit_type),
            soft,
            hard,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Rlimit;
    use serde_json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({
                "type": "RLIMIT_CORE",
                "soft": 256,
                "hard": 512
            }),
            serde_json::to_value(Rlimit::new("RLIMIT_CORE", 256, 512)).unwrap()
        );
    }
}
