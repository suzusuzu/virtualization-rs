//! graphics device module

use crate::base::{Id, NSArray, NSInteger};

use objc::rc::StrongPtr;
use objc::{class, msg_send, sel, sel_impl};

/// The base class for a graphics device configuration.
pub trait VZGraphicsDeviceConfiguration {
    fn id(&self) -> Id;
}

/// The configuration for a Mac graphics device.
pub struct VZMacGraphicsDisplayConfiguration(StrongPtr);

impl VZMacGraphicsDisplayConfiguration {
    /// Create a display configuration suitable for showing on the specified screen.
    ///
    /// # Safety
    /// The type `NSSize` must be a valid struct type of `NSSize` [^1].
    ///
    /// [^1]: https://developer.apple.com/documentation/foundation/nssize?language=objc
    pub unsafe fn new_for<NSSize>(screen: Id, size_in_points: NSSize) -> Self {
        let i: Id = unsafe { msg_send![class!(VZMacGraphicsDisplayConfiguration), alloc] };
        Self(unsafe {
            StrongPtr::new(msg_send![
                i,
                initWithScreen: screen
                sizeInPoints: size_in_points
            ])
        })
    }

    /// Create a display configuration with the specified pixel dimensions and pixel density.
    pub fn new_with(
        width_in_pixels: NSInteger,
        height_in_pixels: NSInteger,
        pixels_per_inch: NSInteger,
    ) -> Self {
        let i: Id = unsafe { msg_send![class!(VZMacGraphicsDisplayConfiguration), alloc] };
        Self(unsafe {
            StrongPtr::new(msg_send![
                i,
                initWithWidthInPixels: width_in_pixels
                heightInPixels: height_in_pixels
                pixelsPerInch: pixels_per_inch
            ])
        })
    }
}

/// Configuration for a display attached to a Mac graphics device.
pub struct VZMacGraphicsDeviceConfiguration(StrongPtr);

impl VZMacGraphicsDeviceConfiguration {
    /// Creates a new Mac graphics device configuration.
    pub fn new(displays: Vec<VZMacGraphicsDisplayConfiguration>) -> Self {
        let displays = displays.iter().map(|x| *x.0).collect();
        let arr: NSArray<VZMacGraphicsDisplayConfiguration> = NSArray::array_with_objects(displays);
        unsafe {
            let p = StrongPtr::new(msg_send![class!(VZMacGraphicsDeviceConfiguration), new]);
            let _: () = msg_send![*p, setDisplays: *arr.p];
            Self(p)
        }
    }
}

impl VZGraphicsDeviceConfiguration for VZMacGraphicsDeviceConfiguration {
    fn id(&self) -> Id {
        *self.0
    }
}

/// The configuration for a Virtio graphics device that configures the dimensions of the graphics
/// device for a Linux VM.
pub struct VZVirtioGraphicsScanoutConfiguration(StrongPtr);

impl VZVirtioGraphicsScanoutConfiguration {
    /// Creates a Virtio graphics device with the specified dimensions.
    pub fn new(width_in_pixels: NSInteger, height_in_pixels: NSInteger) -> Self {
        let i: Id = unsafe { msg_send![class!(VZVirtioGraphicsScanoutConfiguration), alloc] };
        Self(unsafe {
            StrongPtr::new(msg_send![
                i,
                initWithWidthInPixels: width_in_pixels
                heightInPixels: height_in_pixels
            ])
        })
    }
}

/// Configuration that represents the configuration of a Virtio graphics device for a Linux VM.
pub struct VZVirtioGraphicsDeviceConfiguration(StrongPtr);

impl VZVirtioGraphicsDeviceConfiguration {
    /// Creates a new Virtio graphics device.
    pub fn new(scanouts: Vec<VZVirtioGraphicsScanoutConfiguration>) -> Self {
        let scanouts = scanouts.iter().map(|x| *x.0).collect();
        let arr: NSArray<VZMacGraphicsDisplayConfiguration> = NSArray::array_with_objects(scanouts);
        unsafe {
            let p = StrongPtr::new(msg_send![class!(VZVirtioGraphicsDeviceConfiguration), new]);
            let _: () = msg_send![*p, setScanouts: *arr.p];
            Self(p)
        }
    }
}

impl VZGraphicsDeviceConfiguration for VZVirtioGraphicsDeviceConfiguration {
    fn id(&self) -> Id {
        *self.0
    }
}
