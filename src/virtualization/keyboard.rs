//! keyboard module

use crate::base::Id;

use objc::rc::StrongPtr;
use objc::{class, msg_send, sel, sel_impl};

/// The base class for a configuring a keyboard.
pub trait VZKeyboardConfiguration {
    fn id(&self) -> Id;
}

/// A device that defines the configuration for a USB keyboard.
pub struct VZUSBKeyboardConfiguration(StrongPtr);

impl VZUSBKeyboardConfiguration {
    pub fn new() -> Self {
        Self(unsafe { StrongPtr::new(msg_send![class!(VZUSBKeyboardConfiguration), new]) })
    }
}

impl VZKeyboardConfiguration for VZUSBKeyboardConfiguration {
    fn id(&self) -> Id {
        *self.0
    }
}
