use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConsoleSize {
    pub height: u64,
    pub width: u64,
}

#[cfg(test)]
mod tests {
    use super::ConsoleSize;
    use serde_json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({
                "height": 256,
                "width": 512
            }),
            serde_json::to_value(ConsoleSize {
                height: 256,
                width: 512
            })
            .unwrap()
        );
    }
}
