#![no_std]
#![no_main]
#![feature(abi_efiapi)]

#[macro_use]
extern crate alloc;


use core::arch::asm;
use alloc::vec::Vec;
use core::fmt::Write;
use uefi::prelude::*;
use uefi::{CStr16, Handle};
use uefi::table::runtime::ResetType;
use uefi::proto::loaded_image::LoadedImage;
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::proto::media::file::{File, RegularFile, Directory, FileMode, FileAttribute};

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

    // get Directory
    let loaded_image = boot_services.handle_protocol::<LoadedImage>(_image).unwrap().get();
    let device = unsafe{(*loaded_image).device()};
    let file_system = boot_services.handle_protocol::<SimpleFileSystem>(device).unwrap().get();
    let mut root_dir = unsafe{(*file_system).open_volume().unwrap()};

    // let res = root_dir.read_entry(&mut b).unwrap();
    // log::info!("{:?}", file_handle);
    // let res = root_dir.read_entry(&mut b).unwrap();
    // log::info!("{:?}", res);
    // let res = root_dir.read_entry(&mut b).unwrap();
    // log::info!("{:?}", res);

    // get RegularFile 
    let mut cstr_buf = [0u16; 32];
    let cstr_file_name = CStr16::from_str_with_buf("kernel.elf", &mut cstr_buf).unwrap();
    let file_handle = root_dir.open(cstr_file_name, FileMode::Read, FileAttribute::empty()).unwrap();
    let mut file = unsafe {RegularFile::new(file_handle)};

    let mut b = [0_u8; 1024 * 4]; // 本当はサイズ見てから必要な分を確保するべき
    file.read(&mut b);
    log::info!("{:?}", b);

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
        // log::info!("{:?}, {}, {}, {}", descriptor.ty, descriptor.phys_start, descriptor.virt_start, descriptor.page_count);
    })
}

