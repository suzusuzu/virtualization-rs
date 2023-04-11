//! base module

use std::marker::PhantomData;
use std::slice;
use std::str;

use block::Block;
use objc::rc::StrongPtr;
use objc::runtime::{Object, BOOL, NO, YES};
use objc::{class, msg_send, sel, sel_impl};

#[link(name = "Virtualization", kind = "framework")]
extern "C" {}

#[link(name = "Foundation", kind = "framework")]
extern "C" {
    pub fn dispatch_queue_create(label: *const libc::c_char, attr: Id) -> Id;
    pub fn dispatch_sync(queue: Id, block: &Block<(), ()>);
    pub fn dispatch_async(queue: Id, block: &Block<(), ()>);
}

pub type Id = *mut Object;
pub const NIL: Id = 0 as Id;

pub type NSInteger = libc::c_long;
pub type NSUInteger = libc::c_ulong;

pub struct NSArray<T> {
    pub _phantom: PhantomData<T>,
    pub p: StrongPtr,
}

impl<T> NSArray<T> {
    pub fn array_with_objects(objects: Vec<Id>) -> NSArray<T> {
        unsafe {
            let p = StrongPtr::retain(
                msg_send![class!(NSArray), arrayWithObjects:objects.as_slice().as_ptr() count:objects.len()],
            );
            NSArray {
                p: p,
                _phantom: PhantomData,
            }
        }
    }

    pub fn count(&self) -> usize {
        unsafe { msg_send![*self.p, count] }
    }
}

impl<T: From<StrongPtr>> NSArray<T> {
    pub fn object_at_index(&self, index: usize) -> T {
        debug_assert!(index < self.count());
        unsafe { T::from(StrongPtr::retain(msg_send![*self.p, objectAtIndex: index])) }
    }
}

const UTF8_ENCODING: usize = 4;
pub struct NSString(pub StrongPtr);

impl NSString {
    pub fn new(string: &str) -> NSString {
        unsafe {
            let alloc: Id = msg_send![class!(NSString), alloc];
            let p = StrongPtr::new(
                msg_send![alloc, initWithBytes:string.as_ptr() length:string.len() encoding:UTF8_ENCODING as Id],
            );
            NSString(p)
        }
    }

    pub fn len(&self) -> usize {
        unsafe { msg_send![*self.0, lengthOfBytesUsingEncoding: UTF8_ENCODING] }
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            let bytes = {
                let bytes: *const libc::c_char = msg_send![*self.0, UTF8String];
                bytes as *const u8
            };
            let len = self.len();
            let bytes = slice::from_raw_parts(bytes, len);
            str::from_utf8(bytes).unwrap()
        }
    }
}

impl From<StrongPtr> for NSString {
    fn from(p: StrongPtr) -> Self {
        NSString(p)
    }
}

pub struct NSURL(pub StrongPtr);

impl NSURL {
    pub fn url_with_string(url: &str) -> NSURL {
        unsafe {
            let url_nsstring = NSString::new(url);
            let p = StrongPtr::retain(msg_send![class!(NSURL), URLWithString: url_nsstring]);
            NSURL(p)
        }
    }

    pub fn file_url_with_path(path: &str, is_directory: bool) -> NSURL {
        unsafe {
            let path_nsstring = NSString::new(path);
            let is_directory_ = if is_directory { YES } else { NO };
            let p = StrongPtr::retain(
                msg_send![class!(NSURL), fileURLWithPath:path_nsstring isDirectory:is_directory_],
            );
            NSURL(p)
        }
    }

    pub fn check_resource_is_reachable_and_return_error(&self) -> bool {
        let b: BOOL = unsafe { msg_send![*self.0, checkResourceIsReachableAndReturnError: NIL] };
        b == YES
    }

    pub fn absolute_url(&self) -> NSURL {
        unsafe {
            let p = StrongPtr::retain(msg_send![*self.0, absoluteURL]);
            NSURL(p)
        }
    }
}

pub struct NSFileHandle(pub StrongPtr);

impl NSFileHandle {
    pub fn new() -> NSFileHandle {
        unsafe {
            let p = StrongPtr::new(msg_send![class!(NSFileHandle), new]);
            NSFileHandle(p)
        }
    }

    pub fn file_handle_with_standard_input() -> NSFileHandle {
        unsafe {
            let p = StrongPtr::retain(msg_send![class!(NSFileHandle), fileHandleWithStandardInput]);
            NSFileHandle(p)
        }
    }

    pub fn file_handle_with_standard_output() -> NSFileHandle {
        unsafe {
            let p = StrongPtr::retain(msg_send![
                class!(NSFileHandle),
                fileHandleWithStandardOutput
            ]);
            NSFileHandle(p)
        }
    }
}

pub struct NSDictionary(pub StrongPtr);

impl NSDictionary {
    pub fn all_keys<T>(&self) -> NSArray<T> {
        unsafe {
            NSArray {
                p: StrongPtr::retain(msg_send![*self.0, allKeys]),
                _phantom: PhantomData,
            }
        }
    }

    pub fn all_values<T>(&self) -> NSArray<T> {
        unsafe {
            NSArray {
                p: StrongPtr::retain(msg_send![*self.0, allValues]),
                _phantom: PhantomData,
            }
        }
    }
}

pub struct NSError(pub StrongPtr);

impl NSError {
    pub fn nil() -> NSError {
        unsafe {
            let p = StrongPtr::new(NIL);
            NSError(p)
        }
    }

    pub fn code(&self) -> isize {
        unsafe { msg_send![*self.0, code] }
    }

    pub fn localized_description(&self) -> NSString {
        unsafe { NSString(StrongPtr::retain(msg_send![*self.0, localizedDescription])) }
    }

    pub fn localized_failure_reason(&self) -> NSString {
        unsafe {
            NSString(StrongPtr::retain(msg_send![
                *self.0,
                localizedFailureReason
            ]))
        }
    }

    pub fn localized_recovery_suggestion(&self) -> NSString {
        unsafe {
            NSString(StrongPtr::retain(msg_send![
                *self.0,
                localizedRecoverySuggestion
            ]))
        }
    }

    pub fn help_anchor(&self) -> NSString {
        unsafe { NSString(StrongPtr::retain(msg_send![*self.0, helpAnchor])) }
    }

    pub fn user_info(&self) -> NSDictionary {
        unsafe { NSDictionary(StrongPtr::retain(msg_send![*self.0, userInfo])) }
    }

    pub fn dump(&self) {
        let code = self.code();
        println!("code: {}", code);
        let localized_description = self.localized_description();
        println!("localizedDescription : {}", localized_description.as_str());
        let localized_failure_reason = self.localized_failure_reason();
        println!(
            "localizedFailureReason : {}",
            localized_failure_reason.as_str()
        );
        let localized_recovery_suggestion = self.localized_recovery_suggestion();
        println!(
            "localizedRecoverySuggestion : {}",
            localized_recovery_suggestion.as_str()
        );
        let help_anchor = self.help_anchor();
        println!("helpAnchor : {}", help_anchor.as_str());
        let user_info = self.user_info();
        println!("userInfo :");
        let keys: NSArray<NSString> = user_info.all_keys();
        let values: NSArray<NSString> = user_info.all_values();
        let cnt = keys.count();
        for i in 0..cnt {
            let k = keys.object_at_index(i);
            let o = values.object_at_index(i);
            println!("    key: {}, value: {}", k.as_str(), o.as_str());
        }
    }
}
