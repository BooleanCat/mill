mod capabilities;
mod console_size;
mod rlimit;
mod user;

pub use capabilities::Capabilities;
pub use console_size::ConsoleSize;
pub use rlimit::Rlimit;
use serde::{Deserialize, Serialize};
pub use user::User;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Process {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_size: Option<ConsoleSize>,

    pub cwd: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_line: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rlimits: Option<Vec<Rlimit>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub apparmor_profile: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Capabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_new_privileges: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_score_adj: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub selinux_label: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[cfg(test)]
mod tests {
    use super::{ConsoleSize, Process, Rlimit, User};
    use serde_json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({"cwd": "/foo/bar"}),
            serde_json::to_value(Process {
                cwd: "/foo/bar".to_string(),
                ..Default::default()
            })
            .unwrap()
        );
    }

    #[test]
    fn serialize_optional_fields() {
        assert_eq!(
            serde_json::json!({
                "terminal": true,
                "consoleSize": {
                    "height": 256,
                    "width": 512
                },
                "cwd": "/foo/bar",
                "env": ["FOO=BAR", "BAR=BAZ"],
                "args": ["ls", "./"],
                "commandLine": "ls ./",
                "rlimits": [
                    {
                        "type": "RLIMIT_CORE",
                        "soft": 256,
                        "hard": 512
                    }
                ],
                "apparmorProfile": "profile",
                "capabilities": {},
                "noNewPrivileges": true,
                "oomScoreAdj": 200,
                "selinuxLabel": "foo",
                "user": {
                    "uid": 10,
                    "gid": 20
                }
            }),
            serde_json::to_value(Process {
                cwd: "/foo/bar".into(),
                terminal: Some(true),
                console_size: Some(ConsoleSize {
                    height: 256,
                    width: 512
                }),
                env: Some(vec!["FOO=BAR".into(), "BAR=BAZ".into()]),
                args: Some(vec!["ls".into(), "./".into()]),
                command_line: Some("ls ./".into()),
                rlimits: Some(vec![Rlimit {
                    rlimit_type: "RLIMIT_CORE".into(),
                    soft: 256,
                    hard: 512
                }]),
                apparmor_profile: Some("profile".into()),
                capabilities: Some(Default::default()),
                no_new_privileges: Some(true),
                oom_score_adj: Some(200),
                selinux_label: Some("foo".into()),
                user: Some(User::Posix {
                    uid: 10,
                    gid: 20,
                    umask: None,
                    additional_gids: None,
                }),
            })
            .unwrap()
        );
    }
}
