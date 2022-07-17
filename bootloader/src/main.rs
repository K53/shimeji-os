#![no_std]
#![no_main]
#![feature(abi_efiapi)]

#[macro_use]
extern crate alloc;

use core::arch::asm;
use core::mem;
use alloc::slice;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::Write;
use uefi::prelude::*;
use uefi::{CStr16, Handle};
use uefi::table::runtime::ResetType;
use uefi::proto::loaded_image::LoadedImage;
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::proto::media::file::{File, RegularFile, Directory, FileMode, FileAttribute, FileInfo};
use uefi::table::boot::{AllocateType, MemoryType}; 
use goblin::elf::{Elf, program_header};


const KERNEL_BASE_ADDR: usize = 0x100000;
const EFI_PAGE_SIZE: usize = 0x1000;

#[entry]
fn efi_main(_image: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    system_table.stdout().reset(false).unwrap();

    let revision = system_table.uefi_revision();
    let (major, minor) = (revision.major(), revision.minor());
    log::info!("UEFI {}.{}", major, minor);

    let boot_services = system_table.boot_services();
    get_memory_map(boot_services);

    let elf_entry = load_kernel(_image, boot_services);

    let entry_point: extern "sysv64" fn() = unsafe {
        mem::transmute(elf_entry)
    };
    entry_point();
    system_table.runtime_services().reset(ResetType::Shutdown, Status::SUCCESS, None);
}

fn get_memory_map(boot_services: &BootServices) {
    let map_size = boot_services.memory_map_size().map_size;
    let mut memmap_buf = vec![0; map_size * 8];
    // log::info!("buffer size: {}", map_size);
    let (_map_key, desc_itr) = boot_services.memory_map(&mut memmap_buf).unwrap();
    let descriptors = desc_itr.copied().collect::<Vec<_>>();
    // descriptors.iter().for_each(|descriptor| {
    //     // log::info!("{:?}, {}, {}, {}", descriptor.ty, descriptor.phys_start, descriptor.virt_start, descriptor.page_count);
    // })
}

// ELF binary(Executable, Library)
//  - PIE(Position Independent Executable)

fn open_directory(_image: Handle, boot_services: &BootServices) -> Directory {
    let loaded_image = boot_services.handle_protocol::<LoadedImage>(_image).unwrap().get(); //handle_protocolは非推奨 後で変える
    let device = unsafe{(*loaded_image).device()};
    let file_system = boot_services.handle_protocol::<SimpleFileSystem>(device).unwrap().get();
    unsafe {
        (*file_system).open_volume().unwrap()
    }
}

fn open_file(root_dir: &mut Directory) -> RegularFile {
    let mut cstr_buf = [0u16; 32];
    let cstr_file_name = CStr16::from_str_with_buf("kernel.elf", &mut cstr_buf).unwrap();
    let file_handle = root_dir.open(cstr_file_name, FileMode::Read, FileAttribute::empty()).unwrap();
    unsafe {
        RegularFile::new(file_handle)
    }
}

fn load_elf(boot_services: &BootServices, buf: Vec<u8>) -> usize {
    let elf = Elf::parse(&buf).unwrap();

    let mut dest_start = usize::MAX;
    let mut dest_end = 0;
    for ph in elf.program_headers.iter() {
        log::info!("Program header: {} {} {} {} {}",program_header::pt_to_str(ph.p_type),ph.p_offset,ph.p_vaddr,ph.p_paddr,ph.p_memsz);

        if ph.p_type != program_header::PT_LOAD {
            continue;
        }
        dest_start = dest_start.min(ph.p_paddr as usize); // Calculate the first memory address of the PT_LOAD binary section over the all binary sections
        dest_end = dest_end.max(ph.p_paddr + ph.p_memsz); // Calculate the last memory address of the PT_LOAD binary section over the all binary sections
    }

    boot_services.allocate_pages(AllocateType::Address(dest_start), MemoryType::LOADER_DATA,(dest_end as usize - dest_start as usize + EFI_PAGE_SIZE - 1) / EFI_PAGE_SIZE).unwrap();

    for ph in elf.program_headers.iter() {
        if ph.p_type != program_header::PT_LOAD {
            continue;
        }
        let dest = unsafe {
            slice::from_raw_parts_mut(ph.p_paddr as *mut u8, ph.p_memsz as usize)
        };
        dest[..(ph.p_filesz as usize)].copy_from_slice(&buf[(ph.p_offset as usize)..(ph.p_offset as usize + ph.p_filesz as usize)]);
        dest[(ph.p_filesz as usize)..].fill(0);
    };
    elf.entry as usize
}

fn load_kernel(_image: Handle, boot_services: &BootServices) -> usize {
    let mut root_dir = open_directory(_image, &boot_services);
    let mut file = open_file(&mut root_dir);
    let file_size = file.get_boxed_info::<FileInfo>().unwrap().file_size() as usize;
    let mut buf = vec![0; file_size];
    file.read(&mut buf);
    file.close();
    load_elf(&boot_services, buf)
}