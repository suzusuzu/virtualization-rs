use crate::base::Id;

use objc::rc::StrongPtr;
use objc::{class, msg_send, sel, sel_impl};

pub trait VZMemoryBalloonDeviceConfiguration {
    unsafe fn id(&self) -> Id;
}

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
