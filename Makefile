BOOTLOADER_PATH=./bootloader/target/x86_64-unknown-uefi/debug/bootloader.efi 
KERNEL_PATH=./kernel/target/x86_64-unknown-none-shimejios/debug/kernel.elf 
DISK_IMG_PATH=./disk.img
MOUNT_POINT=./mnt

.PHONY: run
run: $(DISK_IMG_PATH)
	qemu-system-x86_64 -drive if=pflash,file=OVMF_CODE.fd -drive if=pflash,file=OVMF_VARS.fd -hda $(DISK_IMG_PATH) -monitor stdio


$(DISK_IMG_PATH): $(BOOTLOADER_PATH) $(KERNEL_PATH)
	qemu-img create -f raw $(DISK_IMG_PATH) 200M
	mkfs.fat -n 'ShimejiOS' -s 2 -f 2 -R 32 -F 32 $(DISK_IMG_PATH)
	hdiutil attach -mountpoint $(MOUNT_POINT) $(DISK_IMG_PATH)
	sleep 0.5
	mkdir -p $(MOUNT_POINT)/EFI/BOOT
	cp $(BOOTLOADER_PATH) $(MOUNT_POINT)/EFI/BOOT/BOOTX64.EFI
	cp $(KERNEL_PATH) $(MOUNT_POINT)/kernel.elf
	sleep 1
	hdiutil detach $(MOUNT_POINT)


$(BOOTLOADER_PATH): ./bootloader/src/*.rs ./bootloader/Cargo.toml ./bootloader/.cargo/*
	docker run -v $(shell pwd):/rust/ shimeji /bin/bash -c "cd /rust/bootloader && cargo +nightly-2022-06-18 rustc --target x86_64-unknown-uefi -Z build-std"

$(KERNEL_PATH): ./kernel/src/*.rs ./kernel/Cargo.toml ./kernel/.cargo/* ./kernel/x86_64-unknown-none-shimejios.json
	docker run -v $(shell pwd):/rust/ rust-uefi /bin/bash -c "cd /rust/kernel && cargo build"

.PHONY: clean
clean:
	-rm -f $(DISK_IMG_PATH)
	-rm -f $(BOOTLOADER_PATH)
	-rm -f $(KERNEL_PATH)