#![no_std]
#![no_main]
#![feature(abi_efiapi)]

#[macro_use]
extern crate alloc;

use core::arch::asm;
use core::fmt::Write;
use uefi::prelude::*;
use uefi::table::runtime::ResetType;

#[entry]
fn efi_main(_image: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    system_table.stdout().reset(false).unwrap();

    writeln!(system_table.stdout(), "Hello, World!").unwrap();
    {
        let revision = system_table.uefi_revision();
        let (major, minor) = (revision.major(), revision.minor());
        log::info!("UEFI {}.{}", major, minor);
    }
    system_table.boot_services().stall(5_000_000);

    system_table.stdout().reset(false).unwrap();

    system_table.runtime_services()
        .reset(ResetType::Shutdown, Status::SUCCESS, None);
}