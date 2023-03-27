include .env

all: bootloader kernel

.PHONY: kernel
kernel: 
	${MAKE} -C $@ build

.PHONY: bootloader
bootloader:
	${MAKE} -C bootloader build

# "all" command invokes qemu that waits gdb connection and outputs ovmf debug log to debug.log
run: all
	QEMU_OPTS="-S -s -debugcon file:debug.log -global isa-debugcon.iobase=0x402" ${MIKANOS_BUILD_PATH}/devenv/run_qemu.sh bootloader/target/x86_64_mikan-uefi/debug/rikan.efi kernel/target/x86_64-unknown-rikan-elf/debug/kernel
