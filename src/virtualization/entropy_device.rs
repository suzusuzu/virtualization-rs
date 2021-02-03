//! entropy device module

use crate::base::Id;

use objc::rc::StrongPtr;
use objc::{class, msg_send, sel, sel_impl};

/// common configure of entropy device
pub trait VZEntropyDeviceConfiguration {
    fn id(&self) -> Id;
}

/// configure of entropy device
pub struct VZVirtioEntropyDeviceConfiguration(StrongPtr);

impl VZVirtioEntropyDeviceConfiguration {
    pub fn new() -> VZVirtioEntropyDeviceConfiguration {
        unsafe {
            let p = StrongPtr::new(msg_send![class!(VZVirtioEntropyDeviceConfiguration), new]);
            VZVirtioEntropyDeviceConfiguration(p)
        }
    }
}

impl VZEntropyDeviceConfiguration for VZVirtioEntropyDeviceConfiguration {
    fn id(&self) -> Id {
        *self.0
    }
}
