//! serial port module

use crate::base::{Id, NSFileHandle};

use objc::rc::StrongPtr;
use objc::{class, msg_send, sel, sel_impl};

/// common configure for serial port attachment
pub trait VZSerialPortAttachment {
    fn id(&self) -> Id;
}

/// builder for VZFileHandleSerialPortAttachment
/// # Examples
/// ```rust
/// let attachement = VZFileHandleSerialPortAttachmentBuilder::new()
///     .file_handle_for_reading(file_handle_for_reading)
///     .file_handle_for_writing(file_handle_for_writing)
///     .build();
/// ```
pub struct VZFileHandleSerialPortAttachmentBuilder<R, W> {
    file_handle_for_reading: R,
    file_handle_for_writing: W,
}

impl VZFileHandleSerialPortAttachmentBuilder<(), ()> {
    pub fn new() -> Self {
        VZFileHandleSerialPortAttachmentBuilder {
            file_handle_for_reading: (),
            file_handle_for_writing: (),
        }
    }
}

impl<R, W> VZFileHandleSerialPortAttachmentBuilder<R, W> {
    pub fn file_handle_for_reading(
        self,
        file_handle_for_reading: NSFileHandle,
    ) -> VZFileHandleSerialPortAttachmentBuilder<NSFileHandle, W> {
        VZFileHandleSerialPortAttachmentBuilder {
            file_handle_for_reading: file_handle_for_reading,
            file_handle_for_writing: self.file_handle_for_writing,
        }
    }

    pub fn file_handle_for_writing(
        self,
        file_handle_for_writing: NSFileHandle,
    ) -> VZFileHandleSerialPortAttachmentBuilder<R, NSFileHandle> {
        VZFileHandleSerialPortAttachmentBuilder {
            file_handle_for_reading: self.file_handle_for_reading,
            file_handle_for_writing: file_handle_for_writing,
        }
    }
}

impl VZFileHandleSerialPortAttachmentBuilder<NSFileHandle, NSFileHandle> {
    pub fn build(self) -> VZFileHandleSerialPortAttachment {
        unsafe {
            VZFileHandleSerialPortAttachment::new(
                self.file_handle_for_reading,
                self.file_handle_for_writing,
            )
        }
    }
}

/// thie struct configure a serial port
pub struct VZFileHandleSerialPortAttachment(StrongPtr);

impl VZFileHandleSerialPortAttachment {
    unsafe fn new(
        file_handle_for_reading: NSFileHandle,
        file_handle_for_writing: NSFileHandle,
    ) -> VZFileHandleSerialPortAttachment {
        let i: Id = msg_send![class!(VZFileHandleSerialPortAttachment), new];
        let p = StrongPtr::new(
            msg_send![i, initWithFileHandleForReading:*file_handle_for_reading.0 fileHandleForWriting:*file_handle_for_writing.0],
        );
        VZFileHandleSerialPortAttachment(p)
    }
}

impl VZSerialPortAttachment for VZFileHandleSerialPortAttachment {
    fn id(&self) -> Id {
        *self.0
    }
}

/// configure of serial port
pub trait VZSerialPortConfiguration {
    fn id(&self) -> Id;
}

/// configure of serial port through the Virtio interface
pub struct VZVirtioConsoleDeviceSerialPortConfiguration(StrongPtr);

impl VZVirtioConsoleDeviceSerialPortConfiguration {
    pub fn new<T: VZSerialPortAttachment>(
        attachement: T,
    ) -> VZVirtioConsoleDeviceSerialPortConfiguration {
        unsafe {
            let p = StrongPtr::new(msg_send![
                class!(VZVirtioConsoleDeviceSerialPortConfiguration),
                new
            ]);
            let _: Id = msg_send![*p, setAttachment: attachement.id()];
            VZVirtioConsoleDeviceSerialPortConfiguration(p)
        }
    }
}

impl VZSerialPortConfiguration for VZVirtioConsoleDeviceSerialPortConfiguration {
    fn id(&self) -> Id {
        *self.0
    }
}
