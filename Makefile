include .env

all: bootloader kernel

.PHONY: kernel
kernel: 
	${MAKE} -C $@ build

.PHONY: bootloader
bootloader:
	${MAKE} -C bootloader build

run: all
	${MIKANOS_BUILD_PATH}/devenv/run_qemu.sh bootloader/target/x86_64-unknown-uefi/debug/rikan.efi kernel/target/x86_64-unknown-rikan-elf/debug/kernel
