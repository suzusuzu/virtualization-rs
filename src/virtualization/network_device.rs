use crate::base::{Id, NSString};

use objc::rc::StrongPtr;
use objc::{class, msg_send, sel, sel_impl};

pub trait VZNetworkDeviceAttachment {
    unsafe fn id(&self) -> Id;
}

pub struct VZNATNetworkDeviceAttachment(StrongPtr);

impl VZNATNetworkDeviceAttachment {
    pub unsafe fn new() -> VZNATNetworkDeviceAttachment {
        let p = StrongPtr::new(msg_send![class!(VZNATNetworkDeviceAttachment), new]);
        VZNATNetworkDeviceAttachment(p)
    }
}

impl VZNetworkDeviceAttachment for VZNATNetworkDeviceAttachment {
    unsafe fn id(&self) -> Id {
        *self.0
    }
}

pub trait VZBridgedNetworkInterface {
    unsafe fn id(&self) -> Id;
    unsafe fn localized_display_name(&self) -> NSString {
        let _obj = self.id();
        let p = StrongPtr::retain(msg_send![class!(_obj), localizedDisplayName]);
        NSString(p)
    }
    unsafe fn identifier(&self) -> NSString {
        let _obj = self.id();
        let p = StrongPtr::retain(msg_send![class!(_obj), identifier]);
        NSString(p)
    }
}

pub struct VZBridgedNetworkDeviceAttachment(StrongPtr);

impl VZBridgedNetworkDeviceAttachment {
    pub unsafe fn new<T: VZBridgedNetworkInterface>(
        interface: T,
    ) -> VZBridgedNetworkDeviceAttachment {
        let obj: Id = msg_send![class!(VZBridgedNetworkDeviceAttachment), alloc];
        let p = StrongPtr::new(msg_send![obj, initWithInterface:interface.id()]);
        VZBridgedNetworkDeviceAttachment(p)
    }
}

impl VZNetworkDeviceAttachment for VZBridgedNetworkDeviceAttachment {
    unsafe fn id(&self) -> Id {
        *self.0
    }
}

pub struct VZMACAddress(pub StrongPtr);

impl VZMACAddress {
    pub unsafe fn new() -> VZMACAddress {
        let p = StrongPtr::new(msg_send![class!(VZMACAddress), new]);
        VZMACAddress(p)
    }
    pub unsafe fn random_locally_administered_address() -> VZMACAddress {
        let p = StrongPtr::new(msg_send![
            class!(VZMACAddress),
            randomLocallyAdministeredAddress
        ]);
        VZMACAddress(p)
    }

    pub unsafe fn init_with_string(s: &str) -> VZMACAddress {
        let string = NSString::new(s);
        let p = StrongPtr::new(msg_send![class!(VZMACAddress), initWithString:*string.0]);
        VZMACAddress(p)
    }
}

pub trait VZNetworkDeviceConfiguration {
    unsafe fn id(&self) -> Id;
}

pub struct VZVirtioNetworkDeviceConfiguration(StrongPtr);

impl VZVirtioNetworkDeviceConfiguration {
    pub unsafe fn new<T: VZNetworkDeviceAttachment>(
        attachment: T,
    ) -> VZVirtioNetworkDeviceConfiguration {
        let p = StrongPtr::new(msg_send![class!(VZVirtioNetworkDeviceConfiguration), new]);
        let _: Id = msg_send![*p, setAttachment:attachment.id()];
        VZVirtioNetworkDeviceConfiguration(p)
    }

    pub unsafe fn set_attachment<T: VZNetworkDeviceAttachment>(&mut self, attachment: T) {
        let _: Id = msg_send![*self.0, setAttachment:attachment.id()];
    }

    pub unsafe fn set_mac_address(&mut self, mac: VZMACAddress) {
        let _: Id = msg_send![*self.0, setMACAddress:*mac.0];
    }
}

impl VZNetworkDeviceConfiguration for VZVirtioNetworkDeviceConfiguration {
    unsafe fn id(&self) -> Id {
        *self.0
    }
}
