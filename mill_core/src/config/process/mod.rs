mod capabilities;
mod console_size;
mod rlimit;

pub use capabilities::Capabilities;
pub use console_size::ConsoleSize;
pub use rlimit::Rlimit;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Process {
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub terminal: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_size: Option<ConsoleSize>,

    pub cwd: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env: Vec<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_line: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rlimits: Vec<Rlimit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub apparmor_profile: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Capabilities>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub no_new_privileges: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_score_adj: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub selinux_label: Option<String>,
    // user
}

#[cfg(test)]
mod tests {
    use super::{ConsoleSize, Process, Rlimit};
    use serde_json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({
                "terminal": true,
                "consoleSize": {
                    "height": 1024,
                    "width": 2048
                },
                "cwd": "/root",
                "env": [
                    "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
                    "TERM=xterm"
                ],
                "args": ["sh"],
                "commandLine": "ls /",
                "rlimits": [
                    {
                        "type": "RLIMIT_NOFILE",
                        "hard": 1024,
                        "soft": 1024
                    }
                ],
                "apparmorProfile": "acme_secure_profile",
                "capabilities": {},
                "noNewPrivileges": true,
                "oomScoreAdj": 200,
                "selinuxLabel": "system_u:system_r:svirt_lxc_net_t:s0:c124,c675"
            }),
            serde_json::to_value(Process {
                terminal: true,
                console_size: Some(ConsoleSize {
                    height: 1024,
                    width: 2048
                }),
                cwd: "/root".into(),
                env: vec![
                    "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin".into(),
                    "TERM=xterm".into()
                ],
                args: vec!["sh".into()],
                command_line: Some("ls /".into()),
                rlimits: vec![Rlimit {
                    rlimit_type: "RLIMIT_NOFILE".into(),
                    soft: 1024,
                    hard: 1024
                }],
                apparmor_profile: Some("acme_secure_profile".into()),
                capabilities: Some(Default::default()),
                no_new_privileges: true,
                oom_score_adj: Some(200),
                selinux_label: Some("system_u:system_r:svirt_lxc_net_t:s0:c124,c675".into()),
            })
            .unwrap()
        );
    }

    #[test]
    fn serialize_defaults() {
        assert_eq!(
            serde_json::json!({"cwd": "/root"}),
            serde_json::to_value(Process {
                cwd: "/root".into(),
                ..Default::default()
            })
            .unwrap()
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            serde_json::from_value::<Process>(serde_json::json!({
                "terminal": true,
                "consoleSize": {
                    "height": 1024,
                    "width": 2048
                },
                "cwd": "/root",
                "env": [
                    "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
                    "TERM=xterm"
                ],
                "args": ["sh"],
                "commandLine": "ls /",
                "rlimits": [
                    {
                        "type": "RLIMIT_NOFILE",
                        "hard": 1024,
                        "soft": 1024
                    }
                ],
                "apparmorProfile": "acme_secure_profile",
                "capabilities": {},
                "noNewPrivileges": true,
                "oomScoreAdj": 200,
                "selinuxLabel": "system_u:system_r:svirt_lxc_net_t:s0:c124,c675",
            }))
            .unwrap(),
            Process {
                terminal: true,
                console_size: Some(ConsoleSize {
                    height: 1024,
                    width: 2048
                }),
                cwd: "/root".into(),
                env: vec![
                    "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin".into(),
                    "TERM=xterm".into()
                ],
                args: vec!["sh".into()],
                command_line: Some("ls /".into()),
                rlimits: vec![Rlimit {
                    rlimit_type: "RLIMIT_NOFILE".into(),
                    soft: 1024,
                    hard: 1024
                }],
                apparmor_profile: Some("acme_secure_profile".into()),
                capabilities: Some(Default::default()),
                no_new_privileges: true,
                oom_score_adj: Some(200),
                selinux_label: Some("system_u:system_r:svirt_lxc_net_t:s0:c124,c675".into()),
            }
        );
    }

    #[test]
    fn deserialize_defaults() {
        assert_eq!(
            serde_json::from_value::<Process>(serde_json::json!({"cwd": "/root"})).unwrap(),
            Process {
                terminal: false,
                console_size: None,
                cwd: "/root".into(),
                env: vec![],
                args: vec![],
                command_line: None,
                rlimits: vec![],
                apparmor_profile: None,
                capabilities: None,
                no_new_privileges: false,
                oom_score_adj: None,
                selinux_label: None,
            }
        );
    }
}
