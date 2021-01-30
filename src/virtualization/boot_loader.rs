use crate::base::{Id, NSString, NSURL};

use objc::rc::StrongPtr;
use objc::{class, msg_send, sel, sel_impl};

pub trait VZBootLoader {
    unsafe fn id(&self) -> Id;
}

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
    pub unsafe fn build(self) -> VZLinuxBootLoader {
        VZLinuxBootLoader::new(
            self.kernel_url.as_str(),
            self.initial_ramdisk_url.as_str(),
            self.command_line.as_str(),
        )
    }
}

pub struct VZLinuxBootLoader(StrongPtr);

impl VZLinuxBootLoader {
    unsafe fn new(
        kernel_url: &str,
        initial_ramdisk_url: &str,
        command_line: &str,
    ) -> VZLinuxBootLoader {
        let kernel_url_nsurl =
            NSURL::file_url_with_path(kernel_url, objc::runtime::NO).absolute_url();
        let initial_ramdisk_url_nsurl =
            NSURL::file_url_with_path(initial_ramdisk_url, objc::runtime::NO).absolute_url();
        let command_line_nsstring = NSString::new(command_line);
        let p = StrongPtr::new(msg_send![class!(VZLinuxBootLoader), new]);
        let _: Id = msg_send![*p, setKernelURL: *kernel_url_nsurl.0];
        let _: Id = msg_send![*p, setInitialRamdiskURL: *initial_ramdisk_url_nsurl.0];
        let _: Id = msg_send![*p, setCommandLine: *command_line_nsstring.0];
        VZLinuxBootLoader(p)
    }
}

impl VZBootLoader for VZLinuxBootLoader {
    unsafe fn id(&self) -> Id {
        *self.0
    }
}
