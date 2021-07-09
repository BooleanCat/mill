use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct ConsoleSize {
    pub height: u32,
    pub width: u32,
}

#[cfg(test)]
mod tests {
    use super::ConsoleSize;
    use serde_json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({
                "height": 1024,
                "width": 2048
            }),
            serde_json::to_value(ConsoleSize {
                height: 1024,
                width: 2048
            })
            .unwrap()
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            serde_json::from_value::<ConsoleSize>(serde_json::json!({
                "height": 1024,
                "width": 2048
            }))
            .unwrap(),
            ConsoleSize {
                height: 1024,
                width: 2048
            }
        );
    }
}
