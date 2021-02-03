use block::{Block, ConcreteBlock};
use libc::sleep;
use objc::rc::StrongPtr;
use std::fs::canonicalize;
use virtualization_rs::{
    base::{dispatch_async, dispatch_queue_create, Id, NSError, NSFileHandle, NIL},
    virtualization::{
        boot_loader::VZLinuxBootLoaderBuilder,
        entropy_device::VZVirtioEntropyDeviceConfiguration,
        memory_device::VZVirtioTraditionalMemoryBalloonDeviceConfiguration,
        network_device::{
            VZMACAddress, VZNATNetworkDeviceAttachment, VZVirtioNetworkDeviceConfiguration,
        },
        serial_port::{
            VZFileHandleSerialPortAttachmentBuilder, VZVirtioConsoleDeviceSerialPortConfiguration,
        },
        storage_device::{
            VZDiskImageStorageDeviceAttachmentBuilder, VZVirtioBlockDeviceConfiguration,
        },
        virtual_machine::{VZVirtualMachine, VZVirtualMachineConfigurationBuilder},
    },
};

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "simplevm")]
struct Opt {
    #[structopt(long, parse(from_os_str))]
    kernel: PathBuf,

    #[structopt(short, long, parse(from_os_str))]
    initrd: PathBuf,

    #[structopt(short, long, default_value = "console=hvc0")]
    command_line: String,

    #[structopt(short, long, parse(from_os_str))]
    disk: PathBuf,

    #[structopt(short, long, default_value = "4")]
    cpu: usize,

    #[structopt(short, long, default_value = "2147483648")]
    memory_size: usize,
}

fn main() {
    let opt = Opt::from_args();

    let cpu_count = opt.cpu;
    let memory_size = opt.memory_size;
    let command_line = opt.command_line;
    let kernel = opt.kernel;
    let disk = opt.disk;
    let initrd = opt.initrd;

    if !VZVirtualMachine::supported() {
        println!("not supported");
        return;
    }

    let boot_loader = VZLinuxBootLoaderBuilder::new()
        .kernel_url(
            canonicalize(&kernel)
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap(),
        )
        .initial_ramdisk_url(
            canonicalize(&initrd)
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap(),
        )
        .command_line(command_line)
        .build();
    let file_handle_for_reading = NSFileHandle::file_handle_with_standard_input();
    let file_handle_for_writing = NSFileHandle::file_handle_with_standard_output();
    let attachement = VZFileHandleSerialPortAttachmentBuilder::new()
        .file_handle_for_reading(file_handle_for_reading)
        .file_handle_for_writing(file_handle_for_writing)
        .build();
    let serial = VZVirtioConsoleDeviceSerialPortConfiguration::new(attachement);
    let entropy = VZVirtioEntropyDeviceConfiguration::new();
    let memory_balloon = VZVirtioTraditionalMemoryBalloonDeviceConfiguration::new();
    let block_attachment = match VZDiskImageStorageDeviceAttachmentBuilder::new()
        .path(
            canonicalize(&disk)
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap(),
        )
        .build()
    {
        Ok(x) => x,
        Err(err) => {
            err.dump();
            return;
        }
    };
    let block_device = VZVirtioBlockDeviceConfiguration::new(block_attachment);
    let network_attachment = VZNATNetworkDeviceAttachment::new();
    let mut network_device = VZVirtioNetworkDeviceConfiguration::new(network_attachment);
    network_device.set_mac_address(VZMACAddress::random_locally_administered_address());

    let conf = VZVirtualMachineConfigurationBuilder::new()
        .boot_loader(boot_loader)
        .cpu_count(cpu_count)
        .memory_size(memory_size)
        .entropy_devices(vec![entropy])
        .memory_balloon_devices(vec![memory_balloon])
        .network_devices(vec![network_device])
        .serial_ports(vec![serial])
        .storage_devices(vec![block_device])
        .build();

    match conf.validate_with_error() {
        Ok(_) => {
            let label = std::ffi::CString::new("second").unwrap();
            let queue = unsafe { dispatch_queue_create(label.as_ptr(), NIL) };
            let vm = VZVirtualMachine::new(conf, queue);
            let dispatch_block = ConcreteBlock::new(move || {
                let completion_handler = ConcreteBlock::new(|err: Id| {
                    if err != NIL {
                        let error = unsafe { NSError(StrongPtr::new(err)) };
                        error.dump();
                    }
                });
                let completion_handler = completion_handler.copy();
                let completion_handler: &Block<(Id,), ()> = &completion_handler;
                vm.start_with_completion_handler(completion_handler);
            });
            let dispatch_block = dispatch_block.copy();
            let dispatch_block: &Block<(), ()> = &dispatch_block;
            unsafe {
                dispatch_async(queue, dispatch_block);
            }
            loop {
                unsafe {
                    sleep(100);
                }
            }
        }
        Err(e) => {
            e.dump();
            return;
        }
    }
}
