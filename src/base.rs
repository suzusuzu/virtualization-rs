//! base module

use std::marker::PhantomData;
use std::slice;
use std::str;

use block::Block;
use objc::rc::StrongPtr;
use objc::runtime::{Object, BOOL};
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

pub struct NSArray<T> {
    pub _phantom: PhantomData<T>,
    pub p: StrongPtr,
}

impl<T> NSArray<T> {
    pub unsafe fn array_with_objects(objects: Vec<Id>) -> NSArray<T> {
        let p = StrongPtr::new(
            msg_send![class!(NSArray), arrayWithObjects:objects.as_slice().as_ptr() count:objects.len()],
        );
        NSArray {
            p: p,
            _phantom: PhantomData,
        }
    }

    pub unsafe fn count(&self) -> usize {
        msg_send![*self.p, count]
    }
}

impl<T: From<StrongPtr>> NSArray<T> {
    pub unsafe fn object_at_index(&self, index: usize) -> T {
        T::from(StrongPtr::retain(msg_send![*self.p, objectAtIndex: index]))
    }
}

const UTF8_ENCODING: usize = 4;
pub struct NSString(pub StrongPtr);

impl NSString {
    pub unsafe fn new(string: &str) -> NSString {
        let alloc: Id = msg_send![class!(NSString), alloc];
        let p = StrongPtr::new(
            msg_send![alloc, initWithBytes:string.as_ptr() length:string.len() encoding:UTF8_ENCODING as Id],
        );
        NSString(p)
    }

    pub unsafe fn len(&self) -> usize {
        msg_send![*self.0, lengthOfBytesUsingEncoding: UTF8_ENCODING]
    }

    pub unsafe fn as_str(&self) -> &str {
        let bytes = {
            let bytes: *const libc::c_char = msg_send![*self.0, UTF8String];
            bytes as *const u8
        };
        let len = self.len();
        let bytes = slice::from_raw_parts(bytes, len);
        str::from_utf8(bytes).unwrap()
    }
}

impl From<StrongPtr> for NSString {
    fn from(p: StrongPtr) -> Self {
        NSString(p)
    }
}

pub struct NSURL(pub StrongPtr);

impl NSURL {
    pub unsafe fn url_with_string(url: &str) -> NSURL {
        let url_nsstring = NSString::new(url);
        let p = StrongPtr::retain(msg_send![class!(NSURL), URLWithString: url_nsstring]);
        NSURL(p)
    }

    pub unsafe fn file_url_with_path(path: &str, is_directory: BOOL) -> NSURL {
        let path_nsstring = NSString::new(path);
        let p = StrongPtr::retain(
            msg_send![class!(NSURL), fileURLWithPath:path_nsstring isDirectory:is_directory],
        );
        NSURL(p)
    }

    pub unsafe fn check_resource_is_reachable_and_return_error(&self) -> BOOL {
        let obj: Id = msg_send![*self.0, checkResourceIsReachableAndReturnError: NIL];
        obj as BOOL
    }

    pub unsafe fn absolute_url(&self) -> NSURL {
        let p = StrongPtr::retain(msg_send![*self.0, absoluteURL]);
        NSURL(p)
    }
}

pub struct NSFileHandle(pub StrongPtr);

impl NSFileHandle {
    pub unsafe fn new() -> NSFileHandle {
        let p = StrongPtr::new(msg_send![class!(NSFileHandle), new]);
        NSFileHandle(p)
    }

    pub unsafe fn file_handle_with_standard_input() -> NSFileHandle {
        let p = StrongPtr::retain(msg_send![class!(NSFileHandle), fileHandleWithStandardInput]);
        NSFileHandle(p)
    }

    pub unsafe fn file_handle_with_standard_output() -> NSFileHandle {
        let p = StrongPtr::retain(msg_send![
            class!(NSFileHandle),
            fileHandleWithStandardOutput
        ]);
        NSFileHandle(p)
    }
}

pub struct NSDictionary(pub StrongPtr);

impl NSDictionary {
    pub unsafe fn all_keys<T>(&self) -> NSArray<T> {
        NSArray {
            p: StrongPtr::retain(msg_send![*self.0, allKeys]),
            _phantom: PhantomData,
        }
    }

    pub unsafe fn all_values<T>(&self) -> NSArray<T> {
        NSArray {
            p: StrongPtr::retain(msg_send![*self.0, allValues]),
            _phantom: PhantomData,
        }
    }
}

pub struct NSError(pub StrongPtr);

impl NSError {
    pub unsafe fn nil() -> NSError {
        let p = StrongPtr::new(NIL);
        NSError(p)
    }

    pub unsafe fn code(&self) -> isize {
        msg_send![*self.0, code]
    }

    pub unsafe fn localized_description(&self) -> NSString {
        NSString(StrongPtr::retain(msg_send![*self.0, localizedDescription]))
    }

    pub unsafe fn localized_failure_reason(&self) -> NSString {
        NSString(StrongPtr::retain(msg_send![
            *self.0,
            localizedFailureReason
        ]))
    }

    pub unsafe fn localized_recovery_suggestion(&self) -> NSString {
        NSString(StrongPtr::retain(msg_send![
            *self.0,
            localizedRecoverySuggestion
        ]))
    }

    pub unsafe fn help_anchor(&self) -> NSString {
        NSString(StrongPtr::retain(msg_send![*self.0, helpAnchor]))
    }

    pub unsafe fn user_info(&self) -> NSDictionary {
        NSDictionary(StrongPtr::retain(msg_send![*self.0, userInfo]))
    }

    pub unsafe fn dump(&self) {
        let code: i64 = msg_send![*self.0, code];
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
