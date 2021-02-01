//! storage device module

use crate::base::{Id, NSError, NSURL};

use objc::runtime::BOOL;
use objc::{class, msg_send, sel, sel_impl};
use objc::{rc::StrongPtr, runtime::NO, runtime::YES};

/// common configure of storage device attachment
pub trait VZStorageDeviceAttachment {
    unsafe fn id(&self) -> Id;
}

/// builder for VZDiskImageStorageDeviceAttachment
/// # Examples
/// ```rust
/// let block_attachment = match VZDiskImageStorageDeviceAttachmentBuilder::new()
///     .path(canonicalize(&disk).unwrap().into_os_string().into_string().unwrap())
///     .build()
/// {
///     Ok(x) => x,
///     Err(err) => {
///         err.dump();
///         return;
///     }
/// };
/// ```
pub struct VZDiskImageStorageDeviceAttachmentBuilder<Path, ReadOnly> {
    path: Path,
    read_only: ReadOnly,
}

impl VZDiskImageStorageDeviceAttachmentBuilder<(), bool> {
    pub fn new() -> Self {
        VZDiskImageStorageDeviceAttachmentBuilder {
            path: (),
            read_only: true,
        }
    }
}

impl<Path, ReadOnly> VZDiskImageStorageDeviceAttachmentBuilder<Path, ReadOnly> {
    pub fn path<T: Into<String>>(
        self,
        path: T,
    ) -> VZDiskImageStorageDeviceAttachmentBuilder<String, ReadOnly> {
        VZDiskImageStorageDeviceAttachmentBuilder {
            path: path.into(),
            read_only: self.read_only,
        }
    }

    pub fn read_only(
        self,
        read_only: bool,
    ) -> VZDiskImageStorageDeviceAttachmentBuilder<Path, bool> {
        VZDiskImageStorageDeviceAttachmentBuilder {
            path: self.path,
            read_only: read_only,
        }
    }
}

impl VZDiskImageStorageDeviceAttachmentBuilder<String, bool> {
    pub unsafe fn build(self) -> Result<VZDiskImageStorageDeviceAttachment, NSError> {
        let read_only = if self.read_only { YES } else { NO };
        VZDiskImageStorageDeviceAttachment::new(self.path.as_str(), read_only)
    }
}

/// configure of disk image storage device attachment
pub struct VZDiskImageStorageDeviceAttachment(StrongPtr);

impl VZDiskImageStorageDeviceAttachment {
    unsafe fn new(
        path: &str,
        read_only: BOOL,
    ) -> Result<VZDiskImageStorageDeviceAttachment, NSError> {
        let i: Id = msg_send![class!(VZDiskImageStorageDeviceAttachment), alloc];
        let path_nsurl = NSURL::file_url_with_path(path, objc::runtime::NO);
        let error = NSError::nil();
        let p = StrongPtr::new(
            msg_send![i, initWithURL:*path_nsurl.0 readOnly:read_only error:&(*error.0)],
        );
        if error.code() != 0 {
            Err(error)
        } else {
            Ok(VZDiskImageStorageDeviceAttachment(p))
        }
    }
}

impl VZStorageDeviceAttachment for VZDiskImageStorageDeviceAttachment {
    unsafe fn id(&self) -> Id {
        *self.0
    }
}

/// configure of storage device
pub trait VZStorageDeviceConfiguration {
    unsafe fn id(&self) -> Id;
}

/// configure of storage device through the Virtio interface
pub struct VZVirtioBlockDeviceConfiguration(StrongPtr);

impl VZVirtioBlockDeviceConfiguration {
    pub unsafe fn new<T: VZStorageDeviceAttachment>(
        attachment: T,
    ) -> VZVirtioBlockDeviceConfiguration {
        let i: Id = msg_send![class!(VZVirtioBlockDeviceConfiguration), alloc];
        let p = StrongPtr::new(msg_send![i, initWithAttachment:attachment.id()]);
        VZVirtioBlockDeviceConfiguration(p)
    }
}

impl VZStorageDeviceConfiguration for VZVirtioBlockDeviceConfiguration {
    unsafe fn id(&self) -> Id {
        *self.0
    }
}
