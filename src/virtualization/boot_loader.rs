//! boot loader module
use crate::base::{Id, NSError, NSString, NSUInteger, NSURL};

use objc::rc::StrongPtr;
use objc::{class, msg_send, sel, sel_impl};

/// common behaviors for booting
pub trait VZBootLoader {
    fn id(&self) -> Id;
}

/// builder for VZLinuxBootLoader
/// # Examples
/// ```rust
/// let boot_loader = VZLinuxBootLoaderBuilder::new()
///     .kernel_url(kernel_url)
///     .initial_ramdisk_url(initial_ramdisk_url)
///     .command_line(command_line)
///     .build();
/// ```
pub struct VZLinuxBootLoaderBuilder<KernelURL, InitialRamdiskURL, CommandLine> {
    kernel_url: KernelURL,
    initial_ramdisk_url: InitialRamdiskURL,
    command_line: CommandLine,
}

impl VZLinuxBootLoaderBuilder<(), (), ()> {
    pub fn new() -> Self {
        VZLinuxBootLoaderBuilder {
            kernel_url: (),
            initial_ramdisk_url: (),
            command_line: (),
        }
    }
}

impl<KernelURL, InitialRamdiskURL, CommandLine>
    VZLinuxBootLoaderBuilder<KernelURL, InitialRamdiskURL, CommandLine>
{
    pub fn kernel_url<T: Into<String>>(
        self,
        kernel_url: T,
    ) -> VZLinuxBootLoaderBuilder<String, InitialRamdiskURL, CommandLine> {
        VZLinuxBootLoaderBuilder {
            kernel_url: kernel_url.into(),
            initial_ramdisk_url: self.initial_ramdisk_url,
            command_line: self.command_line,
        }
    }

    pub fn initial_ramdisk_url<T: Into<String>>(
        self,
        initial_ramdisk_url: T,
    ) -> VZLinuxBootLoaderBuilder<KernelURL, String, CommandLine> {
        VZLinuxBootLoaderBuilder {
            kernel_url: self.kernel_url,
            initial_ramdisk_url: initial_ramdisk_url.into(),
            command_line: self.command_line,
        }
    }

    pub fn command_line<T: Into<String>>(
        self,
        command_line: T,
    ) -> VZLinuxBootLoaderBuilder<KernelURL, InitialRamdiskURL, String> {
        VZLinuxBootLoaderBuilder {
            kernel_url: self.kernel_url,
            initial_ramdisk_url: self.initial_ramdisk_url,
            command_line: command_line.into(),
        }
    }
}

impl VZLinuxBootLoaderBuilder<String, String, String> {
    pub fn build(self) -> VZLinuxBootLoader {
        unsafe {
            VZLinuxBootLoader::new(
                self.kernel_url.as_str(),
                self.initial_ramdisk_url.as_str(),
                self.command_line.as_str(),
            )
        }
    }
}

///  bootLoader for Linux kernel
pub struct VZLinuxBootLoader(StrongPtr);

impl VZLinuxBootLoader {
    unsafe fn new(
        kernel_url: &str,
        initial_ramdisk_url: &str,
        command_line: &str,
    ) -> VZLinuxBootLoader {
        let kernel_url_nsurl = NSURL::file_url_with_path(kernel_url, false).absolute_url();
        let initial_ramdisk_url_nsurl =
            NSURL::file_url_with_path(initial_ramdisk_url, false).absolute_url();
        let command_line_nsstring = NSString::new(command_line);
        let p = StrongPtr::new(msg_send![class!(VZLinuxBootLoader), new]);
        let _: Id = msg_send![*p, setKernelURL: *kernel_url_nsurl.0];
        let _: Id = msg_send![*p, setInitialRamdiskURL: *initial_ramdisk_url_nsurl.0];
        let _: Id = msg_send![*p, setCommandLine: *command_line_nsstring.0];
        VZLinuxBootLoader(p)
    }
}

impl VZBootLoader for VZLinuxBootLoader {
    fn id(&self) -> Id {
        *self.0
    }
}

pub struct VZEFIVariableStoreInitializationOption(NSUInteger);

impl VZEFIVariableStoreInitializationOption {
    /// A Boolean value that indicates whether the framework can overwrite the EFI variable store.
    pub fn allow_overwrite() -> Self {
        Self(1 << 0)
    }
}

/// Constants that describe the options available when creating a new Extensible Firmware Interface
/// (EFI) variable store.
#[derive(Default)]
pub struct VZEFIVariableStoreInitializationOptions {
    options: Vec<VZEFIVariableStoreInitializationOption>,
}

impl VZEFIVariableStoreInitializationOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with(mut self, option: VZEFIVariableStoreInitializationOption) -> Self {
        self.options.push(option);
        self
    }

    fn into_raw(self) -> NSUInteger {
        self.options
            .iter()
            .fold(NSUInteger::default(), |mut acc, v| {
                acc |= v.0;
                acc
            })
    }
}

/// An object that represents the Extensible Firmware Interface (EFI) variable store that contains
/// NVRAM variables the EFI exposes.
pub struct VZEFIVariableStore(StrongPtr);

impl VZEFIVariableStore {
    /// Creates a new EFI variable store at specified the URL on the filesystem, initialization
    /// options, and error-return variable.
    ///
    /// ```
    /// # use virtualization_rs::virtualization::boot_loader::*;
    /// let efi_variables = match VZEFIVariableStore::create(
    ///     "/path/to/efi/variables",
    ///     VZEFIVariableStoreInitializationOptions::new()
    ///         .with(VZEFIVariableStoreInitializationOption::allow_overwrite()),
    /// ) {
    ///     Ok(v) => v,
    ///     Err(e) => {
    ///         e.dump();
    ///         panic!("Failed to create an EFI variable store")
    ///     }
    /// };
    /// ```
    pub fn create<T: Into<String>>(
        file_url: T,
        options: VZEFIVariableStoreInitializationOptions,
    ) -> Result<Self, NSError> {
        let file_url = NSURL::url_with_string(file_url.into().as_str());
        let options = options.into_raw();
        let error = NSError::nil();
        let i: Id = unsafe { msg_send![class!(VZEFIVariableStore), alloc] };
        let p = unsafe {
            StrongPtr::new(msg_send![
                i,
                initCreatingVariableStoreAtURL: file_url
                options: options
                error: &(*error.0)
            ])
        };

        if error.code() != 0 {
            Err(error)
        } else {
            Ok(Self(p))
        }
    }

    /// Initialize the variable store from the URL of an existing file.
    pub fn open<T: Into<String>>(file_url: T) -> Self {
        let file_url = NSURL::url_with_string(file_url.into().as_str());
        let i: Id = unsafe { msg_send![class!(VZEFIVariableStore), alloc] };
        Self(unsafe { StrongPtr::new(msg_send![i, initWithURL: file_url]) })
    }
}

/// Type-safe builder for [`VZEFIBootLoader`].
#[derive(Default)]
pub struct VZEFIBootLoaderBuilder {
    variable_store: Option<VZEFIVariableStore>,
}

impl VZEFIBootLoaderBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_variable_store(mut self, variable_store: VZEFIVariableStore) -> Self {
        self.variable_store = Some(variable_store);
        self
    }

    pub fn build(self) -> VZEFIBootLoader {
        unsafe { VZEFIBootLoader::new(self.variable_store) }
    }
}

/// The boot loader configuration the system uses to boot guest-operating systems that expect an
/// Extensible Firmware Interface (EFI) ROM.
pub struct VZEFIBootLoader(StrongPtr);

impl VZEFIBootLoader {
    unsafe fn new(variable_store: Option<VZEFIVariableStore>) -> Self {
        let p = StrongPtr::new(msg_send![class!(VZEFIBootLoader), new]);
        if let Some(v) = variable_store {
            let _: Id = msg_send![*p, setVariableStore: *v.0];
        }
        Self(p)
    }
}

impl VZBootLoader for VZEFIBootLoader {
    fn id(&self) -> Id {
        *self.0
    }
}
