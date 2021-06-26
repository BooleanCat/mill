mod capabilities;
mod console_size;
mod rlimit;
mod user;

pub use capabilities::Capabilities;
pub use console_size::ConsoleSize;
pub use rlimit::Rlimit;
use serde::{Deserialize, Serialize};
pub use user::User;

#[derive(Debug, Serialize, Deserialize)]
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

impl Process {
    pub fn new(cwd: &str) -> Self {
        Self {
            terminal: None,
            console_size: None,
            cwd: String::from(cwd),
            env: None,
            args: None,
            command_line: None,
            rlimits: None,
            apparmor_profile: None,
            capabilities: None,
            no_new_privileges: None,
            oom_score_adj: None,
            selinux_label: None,
            user: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Capabilities, ConsoleSize, Process, Rlimit, User};
    use serde_json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({"cwd": "/foo/bar"}),
            serde_json::to_value(Process::new("/foo/bar")).unwrap()
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
                terminal: Some(true),
                console_size: Some(ConsoleSize::new(256, 512)),
                env: Some(vec![String::from("FOO=BAR"), String::from("BAR=BAZ")]),
                args: Some(vec![String::from("ls"), String::from("./")]),
                command_line: Some(String::from("ls ./")),
                rlimits: Some(vec![Rlimit::new("RLIMIT_CORE", 256, 512)]),
                apparmor_profile: Some(String::from("profile")),
                capabilities: Some(Capabilities::new()),
                no_new_privileges: Some(true),
                oom_score_adj: Some(200),
                selinux_label: Some(String::from("foo")),
                user: Some(User::posix(10, 20)),
                ..Process::new("/foo/bar")
            })
            .unwrap()
        );
    }
}
