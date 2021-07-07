use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Process {
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub terminal: bool,
    // console_size
    pub cwd: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env: Vec<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_line: Option<String>,
    // rlimits
    // apparmorProfile
    // capabilities
    // noNewPrivileges
    // oomScoreAdj
    // selinuxLabel
    // user
}

#[cfg(test)]
mod tests {
    use super::Process;
    use serde_json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({
                "terminal": true,
                "cwd": "/root",
                "env": [
                    "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
                    "TERM=xterm"
                ],
                "args": ["sh"],
                "commandLine": "ls /"
            }),
            serde_json::to_value(Process {
                terminal: true,
                cwd: "/root".into(),
                env: vec![
                    "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin".into(),
                    "TERM=xterm".into()
                ],
                args: vec!["sh".into()],
                command_line: Some("ls /".into()),
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
                "cwd": "/root",
                "env": [
                    "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
                    "TERM=xterm"
                ],
                "args": ["sh"],
                "commandLine": "ls /"
            }))
            .unwrap(),
            Process {
                terminal: true,
                cwd: "/root".into(),
                env: vec![
                    "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin".into(),
                    "TERM=xterm".into()
                ],
                args: vec!["sh".into()],
                command_line: Some("ls /".into()),
            }
        );
    }

    #[test]
    fn deserialize_defaults() {
        assert_eq!(
            serde_json::from_value::<Process>(serde_json::json!({"cwd": "/root"})).unwrap(),
            Process {
                terminal: false,
                cwd: "/root".into(),
                env: vec![],
                args: vec![],
                command_line: None,
            }
        );
    }
}
