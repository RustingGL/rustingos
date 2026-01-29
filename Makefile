# Makefile - by KhanMarauder aka RustyHoodie323
.PHONY: all write full clean emulate

all: clean
	@echo "--------------------------------------------------------"
	@echo "- Make sure to update the version number if necessary! -"
	@echo "--------------------------------------------------------"
	@echo
	@mkdir -p bin/
	@cargo +nightly build --release --target aarch64-unknown-none # Compile kernel

	@# Copy drivers into destination
	@rm -rf fs/System/Drivers/* fs/Lib/*
	@cp src/rusting_io.rs fs/Lib/
	@cp src/rusting_encrypt.rs fs/Lib/
	@cp src/rusting_gl.rs fs/System/Drivers/
	@cp src/rusting_usb.rs fs/System/Drivers/

	@# Convert kernel to valid img
	@cp target/aarch64-unknown-none/release/rustingos bin/kernel.elf
	@rust-objcopy -O binary bin/kernel.elf bin/kernel.bin
	@cp bin/kernel.bin fs/System/boot/kernel8.img

	@echo
	@echo "========================================================"
	@echo

write:
	@echo "--------------------------------------------------------"
	@echo "-              Writing to bootable media               -"
	@echo "--------------------------------------------------------"
	@echo
	@echo "Preparing mount directories..."
	@sudo mkdir -p /tmp/rustingos-dev-bootfs-fat32/
	@sudo mkdir -p /tmp/rustingos-dev-rootfs-ext4/

	@sudo ./format.sh

	@echo "Copying filesystem contents..."
	@sudo cp -r fs/* /tmp/rustingos-dev-rootfs-ext4/
	@sudo cp -r fs/System/boot/* /tmp/rustingos-dev-bootfs-fat32/

	@echo "Unmounting..."
	@sudo umount /tmp/rustingos-dev-bootfs-fat32/ || true
	@sudo umount /tmp/rustingos-dev-rootfs-ext4/ || true

	@echo "Cleaning temporary directories..."
	@sudo rmdir /tmp/rustingos-dev-bootfs-fat32/ /tmp/rustingos-dev-rootfs-ext4/ || true

	@echo "Write complete."

	@echo
	@echo "========================================================"
	@echo

full: all write

emulate:
	@echo "--------------------------------------------------------"
	@echo "-                   Writing to image                   -"
	@echo "--------------------------------------------------------"
	@dd if=/dev/zero of=image.img bs=8K count=1 > /dev/null
	@dd if=bin/kernel.bin of=image.img bs=8K count=1 > /dev/null
	@echo
	@echo "========================================================"
	@echo
	
	@echo "--------------------------------------------------------"
	@echo "-                   Booting in QEMU                    -"
	@echo "--------------------------------------------------------"
	@qemu-system-aarch64 \
		-M virt \
		-cpu cortex-a72 \
		-m 1G \
		-kernel bin/kernel.elf \
		-device ramfb \
		|| true
	@echo
	@echo "========================================================"
	@echo

clean:
	@cargo clean
	@rm -rf obj/ bin/
	@rm -f fs/System/boot/kernel8.img
	@sudo rm -rf /tmp/rustingos-dev-bootfs-fat32/ /tmp/rustingos-dev-rootfs-ext4/ || true
