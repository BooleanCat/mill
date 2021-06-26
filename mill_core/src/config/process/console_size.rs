use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsoleSize {
    pub height: u64,
    pub width: u64,
}

impl ConsoleSize {
    pub fn new(height: u64, width: u64) -> Self {
        Self { height, width }
    }
}

#[cfg(test)]
mod tests {
    use super::ConsoleSize;
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({
            "height": 256,
            "width": 512
        });

        let got = serde_json::to_value(ConsoleSize::new(256, 512)).unwrap();

        assert_eq!(want, got);
    }
}
