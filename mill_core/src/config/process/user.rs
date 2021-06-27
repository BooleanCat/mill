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

#[cfg(test)]
mod tests {
    use super::User;
    use serde_json;

    #[test]
    fn serialize_posix() {
        assert_eq!(
            serde_json::json!({
                "uid": 10,
                "gid": 20
            }),
            serde_json::to_value(User::Posix {
                uid: 10,
                gid: 20,
                umask: None,
                additional_gids: None,
            })
            .unwrap()
        );
    }

    #[test]
    fn serialize_posix_optional_fields() {
        assert_eq!(
            serde_json::json!({
                "uid": 10,
                "gid": 20,
                "umask": 0300,
                "additionalGids": [30, 40]
            }),
            serde_json::to_value(User::Posix {
                uid: 10,
                gid: 20,
                umask: Some(0300),
                additional_gids: Some(vec![30, 40]),
            })
            .unwrap()
        );
    }

    #[test]
    fn serialize_windows() {
        assert_eq!(
            serde_json::json!({}),
            serde_json::to_value(User::Windows { username: None }).unwrap()
        );
    }

    #[test]
    fn serialize_windows_optional_fields() {
        assert_eq!(
            serde_json::json!({"username": "alan"}),
            serde_json::to_value(User::Windows {
                username: Some("alan".into()),
            })
            .unwrap()
        );
    }
}
