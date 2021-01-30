use crate::base::Id;
use objc::rc::StrongPtr;

pub trait VZSocketDeviceConfiguration {
    unsafe fn id(&self) -> Id;
}

pub struct VZSocketDeviceConf(StrongPtr);

impl VZSocketDeviceConfiguration for VZSocketDeviceConf {
    unsafe fn id(&self) -> Id {
        *self.0
    }
}
