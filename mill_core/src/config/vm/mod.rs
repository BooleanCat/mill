mod hypervisor;
mod image;
mod kernel;

pub use hypervisor::Hypervisor;
pub use image::Image;
pub use kernel::Kernel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Vm {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hypervisor: Option<Hypervisor>,

    pub kernel: Kernel,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
}

impl Vm {
    pub fn new(kernel: Kernel) -> Self {
        Self {
            kernel,
            hypervisor: None,
            image: None,
        }
    }
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
            serde_json::to_value(Vm::new(Kernel::new("/foo/bar"))).unwrap()
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
                kernel: Kernel::new("/foo/bar"),
                hypervisor: Some(Hypervisor::new("/bar/baz")),
                image: Some(Image::new("/bar/foo", "raw")),
            })
            .unwrap()
        );
    }
}
