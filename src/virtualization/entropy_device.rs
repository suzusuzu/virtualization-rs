//! entropy device module

use crate::base::Id;

use objc::rc::StrongPtr;
use objc::{class, msg_send, sel, sel_impl};

/// common configure of entropy device
pub trait VZEntropyDeviceConfiguration {
    unsafe fn id(&self) -> Id;
}

/// configure of entropy device
pub struct VZVirtioEntropyDeviceConfiguration(StrongPtr);

impl VZVirtioEntropyDeviceConfiguration {
    pub unsafe fn new() -> VZVirtioEntropyDeviceConfiguration {
        let p = StrongPtr::new(msg_send![class!(VZVirtioEntropyDeviceConfiguration), new]);
        VZVirtioEntropyDeviceConfiguration(p)
    }
}

impl VZEntropyDeviceConfiguration for VZVirtioEntropyDeviceConfiguration {
    unsafe fn id(&self) -> Id {
        *self.0
    }
}
