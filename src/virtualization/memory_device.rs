//! memory device module

use crate::base::Id;

use objc::rc::StrongPtr;
use objc::{class, msg_send, sel, sel_impl};

/// common configure of memory balloon device
pub trait VZMemoryBalloonDeviceConfiguration {
    unsafe fn id(&self) -> Id;
}

/// configure of memory balloon device through the Virtio interface
pub struct VZVirtioTraditionalMemoryBalloonDeviceConfiguration(StrongPtr);

impl VZVirtioTraditionalMemoryBalloonDeviceConfiguration {
    pub unsafe fn new() -> VZVirtioTraditionalMemoryBalloonDeviceConfiguration {
        let p = StrongPtr::new(msg_send![
            class!(VZVirtioTraditionalMemoryBalloonDeviceConfiguration),
            new
        ]);
        VZVirtioTraditionalMemoryBalloonDeviceConfiguration(p)
    }
}

impl VZMemoryBalloonDeviceConfiguration for VZVirtioTraditionalMemoryBalloonDeviceConfiguration {
    unsafe fn id(&self) -> Id {
        *self.0
    }
}
