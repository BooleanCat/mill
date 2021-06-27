mod linux;
mod mount;
mod process;
mod root;
mod solaris;
mod vm;

pub use linux::{IdMapping as LinuxIdMapping, Linux, Namespace as LinuxNamespace};
pub use mount::Mount;
pub use process::{
    Capabilities as ProcessCapabilities, ConsoleSize as ProcessConsoleSize, Process,
    Rlimit as ProcessRlimit, User as ProcessUser,
};
pub use root::Root;
use serde::{Deserialize, Serialize};
pub use solaris::{
    Anet as SolarisAnet, CappedCpu as SolarisCappedCpu, CappedMemory as SolarisCappedMemory,
    Solaris,
};
use std::collections::HashMap;
pub use vm::{Hypervisor as VmHypervisor, Image as VmImage, Kernel as VmKernel, Vm};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub oci_version: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<Root>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<Mount>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub process: Option<Process>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    // pub hooks: Option<Hook>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub linux: Option<Linux>,

    // pub windows: Option<Windows>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solaris: Option<Solaris>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm: Option<Vm>,
}

impl Config {
    pub fn new(oci_version: &str) -> Self {
        Self {
            oci_version: String::from(oci_version),
            root: None,
            mounts: None,
            process: None,
            hostname: None,
            annotations: None,
            vm: None,
            solaris: None,
            linux: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, Mount, Process, Root, Vm, VmKernel};
    use serde_json;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({"ociVersion": "0.1.0"}),
            serde_json::to_value(Config::new("0.1.0")).unwrap()
        );
    }

    #[test]
    fn serialize_optional_fields() {
        assert_eq!(
            serde_json::json!({
                "ociVersion": "0.1.0",
                "root": {"path": "/foo/bar"},
                "mounts": [
                    {"destination": "/foo/bar"}
                ],
                "process": {"cwd": "/foo/bar"},
                "hostname": "baz",
                "annotations": {"com.example.gpu-cores": "2"},
                "vm": {
                    "kernel": {"path": "/bar/foo"}
                },
                "solaris": {},
                "linux": {}
            }),
            serde_json::to_value(Config {
                root: Some(Root::new("/foo/bar")),
                mounts: Some(vec![Mount::new("/foo/bar")]),
                process: Some(Process::new("/foo/bar")),
                hostname: Some(String::from("baz")),
                annotations: Some(
                    [(String::from("com.example.gpu-cores"), String::from("2"))]
                        .iter()
                        .cloned()
                        .collect(),
                ),
                vm: Some(Vm::new(VmKernel::new("/bar/foo"))),
                solaris: Some(Default::default()),
                linux: Some(Default::default()),
                ..Config::new("0.1.0")
            })
            .unwrap()
        );
    }

    #[test]
    fn serialize_example_spec() {
        assert_eq!(
            serde_json::to_value(
                serde_json::from_reader::<_, Config>(BufReader::new(
                    File::open("./src/config/fixtures/config.json").unwrap(),
                ))
                .unwrap()
            )
            .unwrap(),
            serde_json::json!({
                "ociVersion": "1.0.1",
                "process": {
                    "terminal": true,
                    "user": {
                        "uid": 1,
                        "gid": 1,
                        "additionalGids": [
                            5,
                            6
                        ]
                    },
                    "args": [
                        "sh"
                    ],
                    "env": [
                        "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
                        "TERM=xterm"
                    ],
                    "cwd": "/",
                    "capabilities": {
                        "bounding": [
                            "CAP_AUDIT_WRITE",
                            "CAP_KILL",
                            "CAP_NET_BIND_SERVICE"
                        ],
                        "permitted": [
                            "CAP_AUDIT_WRITE",
                            "CAP_KILL",
                            "CAP_NET_BIND_SERVICE"
                        ],
                        "inheritable": [
                            "CAP_AUDIT_WRITE",
                            "CAP_KILL",
                            "CAP_NET_BIND_SERVICE"
                        ],
                        "effective": [
                            "CAP_AUDIT_WRITE",
                            "CAP_KILL"
                        ],
                        "ambient": [
                            "CAP_NET_BIND_SERVICE"
                        ]
                    },
                    "rlimits": [
                        {
                            "type": "RLIMIT_CORE",
                            "hard": 1024,
                            "soft": 1024
                        },
                        {
                            "type": "RLIMIT_NOFILE",
                            "hard": 1024,
                            "soft": 1024
                        }
                    ],
                    "apparmorProfile": "acme_secure_profile",
                    "oomScoreAdj": 100,
                    "selinuxLabel": "system_u:system_r:svirt_lxc_net_t:s0:c124,c675",
                    "noNewPrivileges": true
                },
                "root": {
                    "path": "rootfs",
                    "readonly": true
                },
                "hostname": "slartibartfast",
                "mounts": [
                    {
                        "destination": "/proc",
                        "type": "proc",
                        "source": "proc"
                    },
                    {
                        "destination": "/dev",
                        "type": "tmpfs",
                        "source": "tmpfs",
                        "options": [
                            "nosuid",
                            "strictatime",
                            "mode=755",
                            "size=65536k"
                        ]
                    },
                    {
                        "destination": "/dev/pts",
                        "type": "devpts",
                        "source": "devpts",
                        "options": [
                            "nosuid",
                            "noexec",
                            "newinstance",
                            "ptmxmode=0666",
                            "mode=0620",
                            "gid=5"
                        ]
                    },
                    {
                        "destination": "/dev/shm",
                        "type": "tmpfs",
                        "source": "shm",
                        "options": [
                            "nosuid",
                            "noexec",
                            "nodev",
                            "mode=1777",
                            "size=65536k"
                        ]
                    },
                    {
                        "destination": "/dev/mqueue",
                        "type": "mqueue",
                        "source": "mqueue",
                        "options": [
                            "nosuid",
                            "noexec",
                            "nodev"
                        ]
                    },
                    {
                        "destination": "/sys",
                        "type": "sysfs",
                        "source": "sysfs",
                        "options": [
                            "nosuid",
                            "noexec",
                            "nodev"
                        ]
                    },
                    {
                        "destination": "/sys/fs/cgroup",
                        "type": "cgroup",
                        "source": "cgroup",
                        "options": [
                            "nosuid",
                            "noexec",
                            "nodev",
                            "relatime",
                            "ro"
                        ]
                    }
                ],
                "annotations": {
                    "com.example.key1": "value1",
                    "com.example.key2": "value2"
                }
            }),
        );
    }
}
