mod hypervisor;
mod image;
mod kernel;

pub use hypervisor::Hypervisor;
pub use image::Image;
pub use kernel::Kernel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Vm {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hypervisor: Option<Hypervisor>,

    pub kernel: Kernel,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
}

#[cfg(test)]
mod tests {
    use super::{Hypervisor, Image, Kernel, Vm};
    use serde_json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::json!({
                "kernel": {"path": "/foo/bar"}
            }),
            serde_json::to_value(Vm {
                kernel: Kernel {
                    path: "/foo/bar".into(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .unwrap()
        );
    }

    #[test]
    fn serialize_optional_fields() {
        assert_eq!(
            serde_json::json!({
                "hypervisor": {"path": "/bar/baz"},
                "kernel": {"path": "/foo/bar"},
                "image": {
                    "path": "/bar/foo",
                    "format": "raw"
                }
            }),
            serde_json::to_value(Vm {
                kernel: Kernel {
                    path: "/foo/bar".into(),
                    ..Default::default()
                },
                hypervisor: Some(Hypervisor {
                    path: "/bar/baz".into(),
                    ..Default::default()
                }),
                image: Some(Image {
                    path: "/bar/foo".into(),
                    format: "raw".into()
                }),
            })
            .unwrap()
        );
    }
}
