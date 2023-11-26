include ../.env
build: hankaku.o
	cargo build

run: build target/x86_64-unknown-uefi/debug/rikan.efi
	${MIKANOS_BUILD_PATH}/devenv/run_qemu.sh target/x86_64-unknown-uefi/debug/rikan.efi

gdb:
	rust-gdb -tui "./target/x86_64-unknown-rikan-elf/debug/kernel" -ex "target remote :1234"

hankaku.bin: hankaku.txt
	../tools/makefont.py -o $@ $<

hankaku.o: hankaku.bin
	objcopy -I binary -O elf64-x86-64 -B i386:x86-64 $< $@