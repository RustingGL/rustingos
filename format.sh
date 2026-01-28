#!/bin/bash

# WARNING: This script will erase all data on the USB drive or image!
# Use with caution. Ensure you're using the correct device path.

# Variables
DEVICE="/dev/sda"
BOOT_PARTITION="/dev/sda1"
ROOT_PARTITION="/dev/sda2"
MOUNT_BOOT="/tmp/rustingos-dev-bootfs-fat32/"
MOUNT_ROOT="/tmp/rustingos-dev-rootfs-ext4/"

# Confirm that drive exits (Important to avoid writing errors!)
if [ ! -e $DEVICE ]; then
	echo "Drive does not exist (Is it plugged in?). Aborting..."
	echo
	echo "========================================================"
	echo
	exit 1
fi

# Confirm the device (Important to avoid mistakes!)
read -p "Are you sure you want to partition $DEVICE? (y/n): " CONFIRM
if [[ "$CONFIRM" != "y" ]]; then
	echo "Aborting..."
	exit 1
fi

echo "Formatting target device..."

# Unmount any existing partitions
echo "Unmounting existing partitions..."
sudo umount -q $BOOT_PARTITION 2>/dev/null
sudo umount -q $ROOT_PARTITION 2>/dev/null

# Create a new partition table (this will erase all existing data)
echo "Creating new partition table..."
sudo parted -s $DEVICE mklabel msdos

# Create the FAT32 boot partition (256MB)
echo "Creating the FAT32 boot partition..."
sudo parted -s $DEVICE mkpart primary fat32 1MiB 256MiB

# Create the EXT4 root partition (remaining space)
echo "Creating the EXT4 root partition..."
sudo parted -s $DEVICE mkpart primary ext4 256MiB 100%

# Set the partition types
echo "Setting partition types..."
sudo parted -s $DEVICE set 1 boot on

# Format the boot partition to FAT32
echo "Formatting $BOOT_PARTITION as FAT32..."
sudo mkfs.vfat -F 32 $BOOT_PARTITION

# Format the root partition to EXT4
echo "Formatting $ROOT_PARTITION as EXT4..."
sudo mkfs.ext4 -F $ROOT_PARTITION

# Label the partitions (optional but recommended)
echo "Labeling partitions..."
sudo dosfslabel $BOOT_PARTITION boot
sudo e2label $ROOT_PARTITION rootfs -p

# Mount the partitions
echo "Mounting the partitions..."
sudo mount $BOOT_PARTITION $MOUNT_BOOT
sudo mount $ROOT_PARTITION $MOUNT_ROOT

# Print completion message
echo "Formatting completed."
