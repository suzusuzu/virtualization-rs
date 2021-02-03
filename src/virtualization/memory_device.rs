//! memory device module

use crate::base::Id;

use objc::rc::StrongPtr;
use objc::{class, msg_send, sel, sel_impl};

/// common configure of memory balloon device
pub trait VZMemoryBalloonDeviceConfiguration {
    fn id(&self) -> Id;
}

/// configure of memory balloon device through the Virtio interface
pub struct VZVirtioTraditionalMemoryBalloonDeviceConfiguration(StrongPtr);

impl VZVirtioTraditionalMemoryBalloonDeviceConfiguration {
    pub fn new() -> VZVirtioTraditionalMemoryBalloonDeviceConfiguration {
        unsafe {
            let p = StrongPtr::new(msg_send![
                class!(VZVirtioTraditionalMemoryBalloonDeviceConfiguration),
                new
            ]);
            VZVirtioTraditionalMemoryBalloonDeviceConfiguration(p)
        }
    }
}

impl VZMemoryBalloonDeviceConfiguration for VZVirtioTraditionalMemoryBalloonDeviceConfiguration {
    fn id(&self) -> Id {
        *self.0
    }
}
