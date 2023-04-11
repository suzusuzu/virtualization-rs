//! storage device module

use crate::base::{Id, NSError, NSInteger, NSURL};

use objc::runtime::BOOL;
use objc::{class, msg_send, sel, sel_impl};
use objc::{rc::StrongPtr, runtime::NO, runtime::YES};

/// common configure of storage device attachment
pub trait VZStorageDeviceAttachment {
    fn id(&self) -> Id;
}

/// An integer that describes the disk image caching mode.
pub struct VZDiskImageCachingMode(NSInteger);

impl VZDiskImageCachingMode {
    /// Allows the virtualization framework to automatically determine whether to enable data caching.
    pub fn automatic() -> Self {
        Self(0)
    }

    /// Disables data caching.
    pub fn uncached() -> Self {
        Self(1)
    }

    /// Enables data caching.
    pub fn cached() -> Self {
        Self(2)
    }
}

/// An integer that describes the disk image synchronization mode.
pub struct VZDiskImageSynchronizationMode(NSInteger);

impl VZDiskImageSynchronizationMode {
    /// Synchronizes data to the permanent storage holding the disk image.
    ///
    /// # Discussion
    /// This mode synchronizes the data with the permanent storage holding the disk image and
    /// ensures the data moves from the disk's internal cache to permanent storage. This ensures
    /// there's no loss of already synchronized data in the case of panic or loss of power.
    pub fn full() -> Self {
        Self(1)
    }

    /// Synchronizes data to the drive using the system's best-effort synchronization mode.
    ///
    /// # Discussion
    /// This mode synchronizes the data with the drive, but doesn't ensure the data moves from the
    /// disk’s internal cache to permanent storage.
    ///
    /// This is a best-effort mode with the same guarantees as the `fsync(_:)` system call.
    pub fn fsync() -> Self {
        Self(2)
    }

    /// Disables data synchronization with the permanent storage.
    ///
    /// # Discussion
    /// This option doesn't guarantee data integrity if any error condition occurs such as disk full
    /// on the host, panic, power loss, and so on.
    //
    /// This mode is useful when a VM is only run once to perform a task to completion or failure.
    /// In that case, the framework can't safely reuse the disk image on failure.
    ///
    /// Using this mode may result in improved performance since no synchronization with the
    /// underlying storage is necessary.
    pub fn none() -> Self {
        Self(3)
    }
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
pub struct VZDiskImageStorageDeviceAttachmentBuilder<
    Path,
    ReadOnly,
    CachingMode,
    SynchronizationMode,
> {
    path: Path,
    read_only: ReadOnly,
    caching_mode: CachingMode,
    synchronization_mode: SynchronizationMode,
}

impl VZDiskImageStorageDeviceAttachmentBuilder<(), bool, (), ()> {
    pub fn new() -> Self {
        VZDiskImageStorageDeviceAttachmentBuilder {
            path: (),
            read_only: true,
            caching_mode: (),
            synchronization_mode: (),
        }
    }
}

impl<Path, ReadOnly, CachingMode, SynchronizationMode>
    VZDiskImageStorageDeviceAttachmentBuilder<Path, ReadOnly, CachingMode, SynchronizationMode>
{
    pub fn path<T: Into<String>>(
        self,
        path: T,
    ) -> VZDiskImageStorageDeviceAttachmentBuilder<String, ReadOnly, CachingMode, SynchronizationMode>
    {
        VZDiskImageStorageDeviceAttachmentBuilder {
            path: path.into(),
            read_only: self.read_only,
            caching_mode: self.caching_mode,
            synchronization_mode: self.synchronization_mode,
        }
    }

    pub fn read_only(
        self,
        read_only: bool,
    ) -> VZDiskImageStorageDeviceAttachmentBuilder<Path, bool, CachingMode, SynchronizationMode>
    {
        VZDiskImageStorageDeviceAttachmentBuilder {
            path: self.path,
            read_only,
            caching_mode: self.caching_mode,
            synchronization_mode: self.synchronization_mode,
        }
    }

    pub fn caching_mode(
        self,
        caching_mode: VZDiskImageCachingMode,
    ) -> VZDiskImageStorageDeviceAttachmentBuilder<
        Path,
        ReadOnly,
        VZDiskImageCachingMode,
        SynchronizationMode,
    > {
        VZDiskImageStorageDeviceAttachmentBuilder {
            path: self.path,
            read_only: self.read_only,
            caching_mode,
            synchronization_mode: self.synchronization_mode,
        }
    }

    pub fn synchronization_mode(
        self,
        synchronization_mode: VZDiskImageSynchronizationMode,
    ) -> VZDiskImageStorageDeviceAttachmentBuilder<
        Path,
        ReadOnly,
        CachingMode,
        VZDiskImageSynchronizationMode,
    > {
        VZDiskImageStorageDeviceAttachmentBuilder {
            path: self.path,
            read_only: self.read_only,
            caching_mode: self.caching_mode,
            synchronization_mode,
        }
    }
}

impl VZDiskImageStorageDeviceAttachmentBuilder<String, bool, (), ()> {
    pub fn build(self) -> Result<VZDiskImageStorageDeviceAttachment, NSError> {
        let read_only = if self.read_only { YES } else { NO };
        unsafe { VZDiskImageStorageDeviceAttachment::new(self.path.as_str(), read_only) }
    }
}

impl
    VZDiskImageStorageDeviceAttachmentBuilder<
        String,
        bool,
        VZDiskImageCachingMode,
        VZDiskImageSynchronizationMode,
    >
{
    pub fn build(self) -> Result<VZDiskImageStorageDeviceAttachment, NSError> {
        let read_only = if self.read_only { YES } else { NO };
        unsafe {
            VZDiskImageStorageDeviceAttachment::new_with_mode(
                self.path.as_str(),
                read_only,
                self.caching_mode.0,
                self.synchronization_mode.0,
            )
        }
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
        let path_nsurl = NSURL::file_url_with_path(path, false);
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

    /// Initialize the attachment from a local file URL.
    unsafe fn new_with_mode(
        path: &str,
        read_only: BOOL,
        caching_mode: NSInteger,
        synchronization_mode: NSInteger,
    ) -> Result<VZDiskImageStorageDeviceAttachment, NSError> {
        let i: Id = msg_send![class!(VZDiskImageStorageDeviceAttachment), alloc];
        let path_nsurl = NSURL::file_url_with_path(path, false);
        let error = NSError::nil();
        let p = StrongPtr::new(msg_send![
            i,
            initWithURL: *path_nsurl.0
            readOnly: read_only
            cachingMode: caching_mode
            synchronizationMode: synchronization_mode
            error: &(*error.0)
        ]);

        if error.code() != 0 {
            Err(error)
        } else {
            Ok(VZDiskImageStorageDeviceAttachment(p))
        }
    }
}

impl VZStorageDeviceAttachment for VZDiskImageStorageDeviceAttachment {
    fn id(&self) -> Id {
        *self.0
    }
}

/// configure of storage device
pub trait VZStorageDeviceConfiguration {
    fn id(&self) -> Id;
}

/// configure of storage device through the Virtio interface
pub struct VZVirtioBlockDeviceConfiguration(StrongPtr);

impl VZVirtioBlockDeviceConfiguration {
    pub fn new<T: VZStorageDeviceAttachment>(attachment: T) -> VZVirtioBlockDeviceConfiguration {
        unsafe {
            let i: Id = msg_send![class!(VZVirtioBlockDeviceConfiguration), alloc];
            let p = StrongPtr::new(msg_send![i, initWithAttachment:attachment.id()]);
            VZVirtioBlockDeviceConfiguration(p)
        }
    }
}

impl VZStorageDeviceConfiguration for VZVirtioBlockDeviceConfiguration {
    fn id(&self) -> Id {
        *self.0
    }
}

/// The configuration object that represents a USB Mass storage device.
pub struct VZUSBMassStorageDeviceConfiguration(StrongPtr);

impl VZUSBMassStorageDeviceConfiguration {
    /// Creates a new storage device configuration with the specified attachment.
    pub fn new<T: VZStorageDeviceAttachment>(attachment: T) -> Self {
        unsafe {
            let i: Id = msg_send![class!(VZUSBMassStorageDeviceConfiguration), alloc];
            let p = StrongPtr::new(msg_send![i, initWithAttachment:attachment.id()]);
            Self(p)
        }
    }
}

impl VZStorageDeviceConfiguration for VZUSBMassStorageDeviceConfiguration {
    fn id(&self) -> Id {
        *self.0
    }
}
