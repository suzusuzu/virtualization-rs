//! pointing device module

use crate::base::Id;

use objc::rc::StrongPtr;
use objc::{class, msg_send, sel, sel_impl};

/// The base class for a pointing device configuration.
pub trait VZPointingDeviceConfiguration {
    fn id(&self) -> Id;
}

/// The class that represents the configuration for a Mac trackpad.
///
/// # Note
/// The framework recognizes this device in virtual machines running macOS 13 and later. To support
/// both macOS 13.0 and earlier guests, set pointingDevices to an array that contains both a
/// [`VZMacTrackpadConfiguration`] and a [`VZUSBScreenCoordinatePointingDeviceConfiguration`] object.
pub struct VZMacTrackpadConfiguration(StrongPtr);

impl VZMacTrackpadConfiguration {
    /// Creates a new Mac trackpad configuration.
    pub fn new() -> Self {
        Self(unsafe { StrongPtr::new(msg_send![class!(VZMacTrackpadConfiguration), new]) })
    }
}

impl VZPointingDeviceConfiguration for VZMacTrackpadConfiguration {
    fn id(&self) -> Id {
        *self.0
    }
}

/// An object that defines the configuration for a USB pointing device that reports absolute coordinates.
pub struct VZUSBScreenCoordinatePointingDeviceConfiguration(StrongPtr);

impl VZUSBScreenCoordinatePointingDeviceConfiguration {
    /// Creates a new pointing device.
    pub fn new() -> Self {
        Self(unsafe {
            StrongPtr::new(msg_send![
                class!(VZUSBScreenCoordinatePointingDeviceConfiguration),
                new
            ])
        })
    }
}

impl VZPointingDeviceConfiguration for VZUSBScreenCoordinatePointingDeviceConfiguration {
    fn id(&self) -> Id {
        *self.0
    }
}
