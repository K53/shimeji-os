#![no_std]
#![no_main]
#![feature(abi_efiapi)]

#[macro_use]
extern crate alloc;

use core::arch::asm;
use alloc::vec::Vec;
use core::fmt::Write;
use uefi::prelude::*;
use uefi::table::runtime::ResetType;

#[entry]
fn efi_main(_image: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    system_table.stdout().reset(false).unwrap();

    writeln!(system_table.stdout(), "Hello, World!").unwrap();

    let revision = system_table.uefi_revision();
    let (major, minor) = (revision.major(), revision.minor());
    log::info!("UEFI {}.{}", major, minor);
    
    let boot_services = system_table.boot_services();
    get_memory_map(boot_services);

    boot_services.stall(10_000_000);

    system_table.stdout().reset(false).unwrap();

    system_table.runtime_services()
        .reset(ResetType::Shutdown, Status::SUCCESS, None);
}

fn get_memory_map(boot_services: &BootServices) {
    let map_size = boot_services.memory_map_size().map_size;
    let mut memmap_buf = vec![0; map_size * 8];
    log::info!("buffer size: {}", map_size);
    let (_map_key, desc_itr) = boot_services.memory_map(&mut memmap_buf).unwrap();
    let descriptors = desc_itr.copied().collect::<Vec<_>>();
    descriptors.iter().for_each(|descriptor| {
        log::info!("{:?}, {}, {}, {}", descriptor.ty, descriptor.phys_start, descriptor.virt_start, descriptor.page_count);
    })
}