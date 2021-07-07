use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Cpu {
    #[serde(skip_serializing_if = "Option::is_none")]
    shares: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    quota: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    period: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    realtime_runtime: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    realtime_period: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cpus: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mems: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::Cpu;
    use serde_json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({}),
            serde_json::to_value::<Cpu>(Default::default()).unwrap()
        );
    }

    #[test]
    fn serialize_optional_fields() {
        assert_eq!(
            serde_json::json!({
                "shares": 1024,
                "quota": 1000000,
                "period": 500000,
                "realtimeRuntime": 950000,
                "realtimePeriod": 1000000,
                "cpus": "2-3",
                "mems": "0-7"
            }),
            serde_json::to_value(Cpu {
                shares: Some(1024),
                quota: Some(1000000),
                period: Some(500000),
                realtime_runtime: Some(950000),
                realtime_period: Some(1000000),
                cpus: Some("2-3".into()),
                mems: Some("0-7".into()),
            })
            .unwrap()
        );
    }
}
