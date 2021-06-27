use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Rlimit {
    #[serde(rename = "type")]
    pub rlimit_type: String,

    pub soft: u64,
    pub hard: u64,
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
            serde_json::to_value(Rlimit {
                rlimit_type: "RLIMIT_CORE".into(),
                soft: 256,
                hard: 512
            })
            .unwrap()
        );
    }
}
