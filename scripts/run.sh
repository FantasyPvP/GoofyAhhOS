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

# Get absolute path to project root
script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
project_root=$(cd "$script_dir/.." &>/dev/null && pwd)

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

building() {
    echo -e "${GREEN}${BOLD}Building${NC} $1"
}

error() {
    echo -e "${RED}${BOLD}error${NC}: $1" >&2
    exit 1
}

build_dir="$project_root/build"
iso_root="$build_dir/iso_root"

# Check if we're running tests
is_test=0
if [[ $1 == *"deps"* ]]; then
    is_test=1
    kernel_path="$1"
else
    # Build the kernel normally
    cd "$project_root"
    cargo build
    kernel_path="$build_dir/target/x86_64-kernel/debug/kernel"
fi

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
    cd "$build_dir"
    git clone https://github.com/limine-bootloader/limine.git --branch=v8.x-binary --depth=1 "$build_dir/limine" || error "failed to clone limine"
    make -C "$build_dir/limine" || error "failed to build limine"
    cd "$project_root"
fi

# Copy files
info "Copying files to ISO root"
cp -v "$kernel_path" "$iso_root/boot/kernel" || error "failed to copy kernel"
cp -v "$project_root/config/limine.conf" "$iso_root/boot/limine/limine.conf" || error "failed to copy limine config"
cp -v "$build_dir/limine/limine-bios.sys" "$build_dir/limine/limine-bios-cd.bin" \
      "$build_dir/limine/limine-uefi-cd.bin" "$iso_root/boot/limine/" || error "failed to copy limine files"
cp -v "$build_dir/limine/BOOTX64.EFI" "$iso_root/EFI/BOOT/" || error "failed to copy BOOTX64.EFI"
cp -v "$build_dir/limine/BOOTIA32.EFI" "$iso_root/EFI/BOOT/" || error "failed to copy BOOTIA32.EFI"

# Create ISO
building "bootable ISO image"
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

# Check if we're running in debug mode
if [[ "$(cargo metadata --format-version=1 | jq -r '.workspace_members[0]' | cut -d' ' -f2)" == "(debug)" ]]; then
    debug_flags="-s -S"
else
    debug_flags=""
fi

# Set up test-specific flags
if [ $is_test -eq 1 ]; then
    test_flags="-device isa-debug-exit,iobase=0xf4,iosize=0x04 -display none"
    serial_flags="-serial stdio"
else
    test_flags=""
    # serial_flags="-serial tcp:127.0.0.1:1234,server,nowait"
    serial_flags="-serial stdio"
fi

# Run in QEMU
if [[ ${QEMU_FLAGS} == *-S* ]]; then
    info "Running OS in QEMU with GDB debugging enabled"
    info "To connect GDB, run: gdb"
    info "At the GDB prompt, type: target remote localhost:1234"
else
    info "Running OS in QEMU..."
fi

check_test_res() {
    qemu_exit_code=$?
    if [ $qemu_exit_code -eq 33 ]; then
        # Success case (0x10 << 1) | 1 = 33
        info "All tests passed"
        exit 0
    elif [ $qemu_exit_code -eq 35 ]; then
        # Failure case (0x11 << 1) | 1 = 35
        warning "Some tests failed"
        exit 1
    else
        # Any other exit code is treated as a failure
        warning "Some tests failed"
        exit 1
    fi
}


trap 'check_test_res "tests completed"' ERR
 
cd "$project_root"
qemu-system-x86_64 -M q35 \
    ${kvm_flag} \
    -cdrom "$build_dir/image.iso" \
    -boot d \
    -m 2G \
    ${serial_flags} \
    -no-reboot \
    ${test_flags} \
    ${debug_flags} \
    ${QEMU_FLAGS:-}
