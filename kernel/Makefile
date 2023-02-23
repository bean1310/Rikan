include ../.env
build: 
	cargo build

run: build target/x86_64-unknown-uefi/debug/rikan.efi
	${MIKANOS_BUILD_PATH}/devenv/run_qemu.sh target/x86_64-unknown-uefi/debug/rikan.efi
