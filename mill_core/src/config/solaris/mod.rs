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

impl Solaris {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::{CappedCpu, CappedMemory, Solaris};
    use serde_json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({}),
            serde_json::to_value(Solaris::new()).unwrap()
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
                milestone: Some(String::from("svc:/milestone/container:default")),
                limitpriv: Some(String::from("default")),
                max_shm_memory: Some(String::from("512m")),
                capped_cpu: Some(CappedCpu::new()),
                capped_memory: Some(CappedMemory::new()),
                anet: Some(vec![]),
            })
            .unwrap()
        );
    }
}
