use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum User {
    #[serde(rename_all = "camelCase")]
    Posix {
        uid: i64,
        gid: i64,

        #[serde(skip_serializing_if = "Option::is_none")]
        umask: Option<i64>,

        #[serde(skip_serializing_if = "Option::is_none")]
        additional_gids: Option<Vec<i64>>,
    },
    Windows {
        #[serde(skip_serializing_if = "Option::is_none")]
        username: Option<String>,
    },
}

impl User {
    pub fn posix(uid: i64, gid: i64) -> Self {
        Self::Posix {
            uid,
            gid,
            additional_gids: None,
            umask: None,
        }
    }

    pub fn windows() -> Self {
        Self::Windows { username: None }
    }
}

#[cfg(test)]
mod tests {
    use super::User;
    use serde_json;

    #[test]
    fn serialize_posix() {
        let want = serde_json::json!({
            "uid": 10,
            "gid": 20
        });

        let got = serde_json::to_value(User::posix(10, 20)).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_posix_optional_fields() {
        let want = serde_json::json!({
            "uid": 10,
            "gid": 20,
            "umask": 0300,
            "additionalGids": [
                30,
                40
            ]
        });

        let got = serde_json::to_value(User::Posix {
            umask: Some(0300),
            additional_gids: Some(vec![30, 40]),
            uid: 10,
            gid: 20,
        })
        .unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_windows() {
        let want = serde_json::json!({});

        let got = serde_json::to_value(User::windows()).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_windows_optional_fields() {
        let want = serde_json::json!({
            "username": "alan"
        });

        let got = serde_json::to_value(User::Windows {
            username: Some(String::from("alan")),
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
