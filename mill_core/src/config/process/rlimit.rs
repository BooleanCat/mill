use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
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
                "type": "RLIMIT_NOFILE",
                "soft": 1024,
                "hard": 2048
            }),
            serde_json::to_value(Rlimit {
                rlimit_type: "RLIMIT_NOFILE".into(),
                soft: 1024,
                hard: 2048
            })
            .unwrap()
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            serde_json::from_value::<Rlimit>(serde_json::json!({
                "type": "RLIMIT_NOFILE",
                "soft": 1024,
                "hard": 2048
            }))
            .unwrap(),
            Rlimit {
                rlimit_type: "RLIMIT_NOFILE".into(),
                soft: 1024,
                hard: 2048
            }
        );
    }
}
