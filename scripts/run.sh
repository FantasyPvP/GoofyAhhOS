#!/bin/bash

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# Error handling
set -e
trap 'echo -e "${RED}${BOLD}error${NC}: build failed" >&2' ERR

# Logging functions
info() {
    echo -e "${BLUE}${BOLD}info${NC}: $1"
}

compiling() {
    echo -e "${GREEN}${BOLD}Compiling${NC} $1"
}

warning() {
    echo -e "${YELLOW}${BOLD}warning${NC}: $1" >&2
}

error() {
    echo -e "${RED}${BOLD}error${NC}: $1" >&2
    exit 1
}

build_dir=build
iso_root=$build_dir/iso_root
kernel_path=$1
kernel_name=$(basename $kernel_path)

# Check for required tools
check_tools() {
    local missing=0
    for tool in xorriso git qemu-system-x86_64; do
        if ! command -v $tool >/dev/null 2>&1; then
            error "required tool '$tool' is not installed"
            missing=1
        fi
    done
    if [ $missing -eq 1 ]; then
        error "missing required tools"
    fi
}

# Create build directory structure
info "Creating build directory structure"
mkdir -p "$iso_root/boot/limine"
mkdir -p "$iso_root/EFI/BOOT"

# Clone Limine if needed
if [ ! -d "$build_dir/limine" ]; then
    compiling "limine bootloader"
    git clone https://github.com/limine-bootloader/limine.git --branch=v8.x-binary --depth=1 "$build_dir/limine" || error "failed to clone limine"
    make -C "$build_dir/limine" || error "failed to build limine"
fi

# Copy files
info "Copying files to ISO root"
cp -v "$kernel_path" "$iso_root/boot/kernel" || error "failed to copy kernel"
cp -v config/limine.conf "$iso_root/boot/limine/" || error "failed to copy limine config"
cp -v "$build_dir/limine/limine-bios.sys" "$build_dir/limine/limine-bios-cd.bin" \
      "$build_dir/limine/limine-uefi-cd.bin" "$iso_root/boot/limine/" || error "failed to copy limine files"
cp -v "$build_dir/limine/BOOTX64.EFI" "$iso_root/EFI/BOOT/" || error "failed to copy BOOTX64.EFI"
cp -v "$build_dir/limine/BOOTIA32.EFI" "$iso_root/EFI/BOOT/" || error "failed to copy BOOTIA32.EFI"

# Create ISO
compiling "bootable ISO image"
xorriso -as mkisofs -R -r -J -b boot/limine/limine-bios-cd.bin \
        -no-emul-boot -boot-load-size 4 -boot-info-table -hfsplus \
        -apm-block-size 2048 --efi-boot boot/limine/limine-uefi-cd.bin \
        -efi-boot-part --efi-boot-image --protective-msdos-label \
        "$iso_root" -o "$build_dir/image.iso" || error "failed to create ISO"

# Install Limine
info "Installing Limine bootloader"
"$build_dir/limine/limine" bios-install "$build_dir/image.iso" || error "failed to install limine"

# Check if KVM is available
if [ "${KVM_FLAG:-enable}" = "disable" ]; then
    warning "KVM acceleration disabled by user"
    kvm_flag=""
elif [ -c "/dev/kvm" ] && [ -w "/dev/kvm" ]; then
    info "KVM acceleration enabled"
    kvm_flag="-enable-kvm"
else
    warning "KVM acceleration not available (is kvm module loaded?)"
    kvm_flag=""
fi

# Run in QEMU
if [[ ${QEMU_FLAGS} == *-S* ]]; then
    info "Running OS in QEMU with GDB debugging enabled"
    info "To connect GDB, run: gdb"
    info "At the GDB prompt, type: target remote localhost:1234"
else
    info "Running OS in QEMU..."
fi

exec qemu-system-x86_64 -M q35 \
    ${kvm_flag} \
    -cdrom "$build_dir/image.iso" \
    -boot d \
    -m 2G \
    -serial stdio \
    -no-reboot \
    -no-shutdown \
    ${QEMU_FLAGS:-}
