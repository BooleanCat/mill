mod anet;
mod capped_cpu;
mod capped_memory;

pub use anet::Anet;
pub use capped_cpu::CappedCpu;
pub use capped_memory::CappedMemory;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Solaris {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limitpriv: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_shm_memory: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cappedCPU")]
    pub capped_cpu: Option<CappedCpu>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub capped_memory: Option<CappedMemory>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub anet: Option<Vec<Anet>>,
}

#[cfg(test)]
mod tests {
    use super::Solaris;
    use serde_json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({}),
            serde_json::to_value::<Solaris>(Default::default()).unwrap()
        );
    }

    #[test]
    fn serialize_optional_fields() {
        assert_eq!(
            serde_json::json!({
                "milestone": "svc:/milestone/container:default",
                "limitpriv": "default",
                "maxShmMemory": "512m",
                "cappedCPU": {},
                "cappedMemory": {},
                "anet": []
            }),
            serde_json::to_value(Solaris {
                milestone: Some("svc:/milestone/container:default".into()),
                limitpriv: Some("default".into()),
                max_shm_memory: Some("512m".into()),
                capped_cpu: Some(Default::default()),
                capped_memory: Some(Default::default()),
                anet: Some(vec![]),
            })
            .unwrap()
        );
    }
}
