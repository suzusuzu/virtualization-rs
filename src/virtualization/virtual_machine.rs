use crate::{
    base::{Id, NSArray, NSError},
    virtualization::boot_loader::VZBootLoader,
    virtualization::entropy_device::VZEntropyDeviceConfiguration,
    virtualization::memory_device::VZMemoryBalloonDeviceConfiguration,
    virtualization::network_device::VZNetworkDeviceConfiguration,
    virtualization::serial_port::VZSerialPortConfiguration,
    virtualization::socket_device::VZSocketDeviceConfiguration,
    virtualization::storage_device::VZStorageDeviceConfiguration,
};

use block::Block;
use objc::runtime::BOOL;
use objc::{class, msg_send, sel, sel_impl};
use objc::{rc::StrongPtr, runtime::YES};

pub struct VZVirtualMachineConfigurationBuilder {
    conf: VZVirtualMachineConfiguration,
}

impl VZVirtualMachineConfigurationBuilder {
    pub unsafe fn new() -> Self {
        VZVirtualMachineConfigurationBuilder {
            conf: VZVirtualMachineConfiguration::new(),
        }
    }

    pub unsafe fn boot_loader<T: VZBootLoader>(mut self, boot_loader: T) -> Self {
        self.conf.set_boot_loader(boot_loader);
        self
    }

    pub unsafe fn cpu_count(mut self, cpu_count: usize) -> Self {
        self.conf.set_cpu_count(cpu_count);
        self
    }

    pub unsafe fn memory_size(mut self, memory_size: usize) -> Self {
        self.conf.set_memory_size(memory_size);
        self
    }

    pub unsafe fn entropy_devices<T: VZEntropyDeviceConfiguration>(
        mut self,
        entropy_devices: Vec<T>,
    ) -> Self {
        self.conf.set_entropy_devices(entropy_devices);
        self
    }

    pub unsafe fn memory_balloon_devices<T: VZMemoryBalloonDeviceConfiguration>(
        mut self,
        memory_balloon_devices: Vec<T>,
    ) -> Self {
        self.conf.set_memory_balloon_devices(memory_balloon_devices);
        self
    }

    pub unsafe fn network_devices<T: VZNetworkDeviceConfiguration>(
        mut self,
        network_devices: Vec<T>,
    ) -> Self {
        self.conf.set_network_devices(network_devices);
        self
    }

    pub unsafe fn serial_ports<T: VZSerialPortConfiguration>(
        mut self,
        serial_ports: Vec<T>,
    ) -> Self {
        self.conf.set_serial_ports(serial_ports);
        self
    }

    pub unsafe fn socket_devices<T: VZSocketDeviceConfiguration>(
        mut self,
        socket_devices: Vec<T>,
    ) -> Self {
        self.conf.set_socket_devices(socket_devices);
        self
    }

    pub unsafe fn storage_devices<T: VZStorageDeviceConfiguration>(
        mut self,
        storage_devices: Vec<T>,
    ) -> Self {
        self.conf.set_storage_devices(storage_devices);
        self
    }

    pub fn build(self) -> VZVirtualMachineConfiguration {
        self.conf
    }
}

pub struct VZVirtualMachineConfiguration(StrongPtr);

impl VZVirtualMachineConfiguration {
    unsafe fn new() -> VZVirtualMachineConfiguration {
        let obj = StrongPtr::new(msg_send![class!(VZVirtualMachineConfiguration), new]);
        VZVirtualMachineConfiguration(obj)
    }

    unsafe fn set_boot_loader<T: VZBootLoader>(&mut self, boot_loader: T) {
        let _: () = msg_send![*self.0, setBootLoader: boot_loader.id()];
    }

    unsafe fn set_cpu_count(&mut self, cnt: usize) {
        let _: () = msg_send![*self.0, setCPUCount: cnt];
    }

    unsafe fn set_memory_size(&mut self, size: usize) {
        let _: () = msg_send![*self.0, setMemorySize: size];
    }

    unsafe fn set_entropy_devices<T: VZEntropyDeviceConfiguration>(&mut self, devices: Vec<T>) {
        let device_ids = devices.iter().map(|x| x.id()).collect();
        let arr: NSArray<T> = NSArray::array_with_objects(device_ids);
        let _: () = msg_send![*self.0, setEntropyDevices:*arr.p];
    }

    unsafe fn set_memory_balloon_devices<T: VZMemoryBalloonDeviceConfiguration>(
        &mut self,
        devices: Vec<T>,
    ) {
        let device_ids = devices.iter().map(|x| x.id()).collect();
        let arr: NSArray<T> = NSArray::array_with_objects(device_ids);
        let _: () = msg_send![*self.0, setMemoryBalloonDevices:*arr.p];
    }

    unsafe fn set_network_devices<T: VZNetworkDeviceConfiguration>(&mut self, devices: Vec<T>) {
        let device_ids = devices.iter().map(|x| x.id()).collect();
        let arr: NSArray<T> = NSArray::array_with_objects(device_ids);
        let _: () = msg_send![*self.0, setNetworkDevices:*arr.p];
    }

    unsafe fn set_serial_ports<T: VZSerialPortConfiguration>(&mut self, devices: Vec<T>) {
        let device_ids = devices.iter().map(|x| x.id()).collect();
        let arr: NSArray<T> = NSArray::array_with_objects(device_ids);
        let _: () = msg_send![*self.0, setSerialPorts:*arr.p];
    }

    unsafe fn set_socket_devices<T: VZSocketDeviceConfiguration>(&mut self, devices: Vec<T>) {
        let device_ids = devices.iter().map(|x| x.id()).collect();
        let arr: NSArray<T> = NSArray::array_with_objects(device_ids);
        let _: () = msg_send![*self.0, setSocketDevices:*arr.p];
    }

    unsafe fn set_storage_devices<T: VZStorageDeviceConfiguration>(&mut self, devices: Vec<T>) {
        let device_ids = devices.iter().map(|x| x.id()).collect();
        let arr: NSArray<T> = NSArray::array_with_objects(device_ids);
        let _: () = msg_send![*self.0, setStorageDevices:*arr.p];
    }

    pub unsafe fn validate_with_error(&self) -> Result<BOOL, NSError> {
        let error = NSError(StrongPtr::new(0 as Id));
        let obj: Id = msg_send![*self.0, validateWithError: &(*error.0)];
        if error.code() != 0 {
            Err(error)
        } else {
            Ok(obj as BOOL)
        }
    }
}

#[derive(Clone)]
pub struct VZVirtualMachine(StrongPtr);

#[derive(Debug)]
pub enum VZVirtualMachineState {
    // Initial state before the virtual machine is started.
    VZVirtualMachineStateStopped,
    // Running virtual machine.
    VZVirtualMachineStateRunning,
    // A started virtual machine is paused. This state can only be transitioned from VZVirtualMachineStatePausing.
    VZVirtualMachineStatePaused,
    // The virtual machine has encountered an internal error.
    VZVirtualMachineStateError,
    // The virtual machine is configuring the hardware and starting.
    VZVirtualMachineStateStarting,
    // The virtual machine is being paused. This is the intermediate state between VZVirtualMachineStateRunning and VZVirtualMachineStatePaused.
    VZVirtualMachineStatePausing,
    // The virtual machine is being resumed. This is the intermediate state between VZVirtualMachineStatePaused and VZVirtualMachineStateRunning. */
    VZVirtualMachineStateResuming,
    // Other
    Other,
}

impl VZVirtualMachine {
    pub unsafe fn new(conf: VZVirtualMachineConfiguration, queue: Id) -> VZVirtualMachine {
        let i: Id = msg_send![class!(VZVirtualMachine), alloc];
        let p = StrongPtr::new(msg_send![i, initWithConfiguration:*conf.0 queue:queue]);
        VZVirtualMachine(p)
    }

    pub unsafe fn start_with_completion_handler(&self, completion_handler: &Block<(Id,), ()>) {
        let _: Id = msg_send![*self.0, startWithCompletionHandler: completion_handler];
    }

    pub unsafe fn request_stop_with_error(&mut self) -> Result<bool, NSError> {
        let error = NSError(StrongPtr::new(0 as Id));
        let ret: BOOL = msg_send![*self.0, requestStopWithError:*error.0];
        if error.code() != 0 {
            Err(error)
        } else {
            Ok(ret == 1i8)
        }
    }

    pub unsafe fn supported() -> bool {
        let b: BOOL = msg_send![class!(VZVirtualMachine), isSupported];
        b == YES
    }

    pub unsafe fn state(&self) -> VZVirtualMachineState {
        let n: isize = msg_send![*self.0, state];
        match n {
            0 => VZVirtualMachineState::VZVirtualMachineStateStopped,
            1 => VZVirtualMachineState::VZVirtualMachineStateRunning,
            2 => VZVirtualMachineState::VZVirtualMachineStatePaused,
            3 => VZVirtualMachineState::VZVirtualMachineStateError,
            4 => VZVirtualMachineState::VZVirtualMachineStateStarting,
            5 => VZVirtualMachineState::VZVirtualMachineStatePausing,
            6 => VZVirtualMachineState::VZVirtualMachineStateResuming,
            _ => VZVirtualMachineState::Other,
        }
    }
}
