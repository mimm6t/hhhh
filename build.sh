#!/bin/bash
# Build script for Hide-My-Applist Rust

set -e

echo "=== Hide-My-Applist Rust Build Script ==="
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Rust is not installed${NC}"
    echo "Install Rust from: https://rustup.rs/"
    exit 1
fi

echo -e "${GREEN}✓ Rust is installed${NC}"

# Build for host (testing)
echo ""
echo "Building for host..."
cargo build --release

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Host build successful${NC}"
    echo "Binary: target/release/hma-rust"
else
    echo -e "${RED}✗ Host build failed${NC}"
    exit 1
fi

# Check if Android NDK is available
if [ -z "$ANDROID_NDK_HOME" ]; then
    echo -e "${YELLOW}Warning: ANDROID_NDK_HOME not set${NC}"
    echo "Skipping Android build"
    echo ""
    echo "To build for Android:"
    echo "1. Install Android NDK"
    echo "2. Set ANDROID_NDK_HOME environment variable"
    echo "3. Add Android targets: rustup target add aarch64-linux-android"
    exit 0
fi

# Check if Android target is installed
if ! rustup target list | grep -q "aarch64-linux-android (installed)"; then
    echo ""
    echo "Installing Android target..."
    rustup target add aarch64-linux-android
fi

# Build for Android
echo ""
echo "Building for Android (aarch64)..."

# Set up cargo config for Android
mkdir -p .cargo
cat > .cargo/config.toml << EOF
[target.aarch64-linux-android]
linker = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android30-clang"
ar = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"

[build]
target = "aarch64-linux-android"
EOF

cargo build --release --target aarch64-linux-android

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Android build successful${NC}"
    echo "Binary: target/aarch64-linux-android/release/hma-rust"
    
    # Strip binary
    echo ""
    echo "Stripping binary..."
    $ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-strip \
        target/aarch64-linux-android/release/hma-rust
    
    # Show binary size
    SIZE=$(du -h target/aarch64-linux-android/release/hma-rust | cut -f1)
    echo -e "${GREEN}Binary size: $SIZE${NC}"
else
    echo -e "${RED}✗ Android build failed${NC}"
    exit 1
fi

echo ""
echo -e "${GREEN}=== Build Complete ===${NC}"
echo ""
echo "Next steps:"
echo "1. Push wxshadow.kpm to device"
echo "2. Load kernel module: kpatch module load /path/to/wxshadow.kpm"
echo "3. Push hma-rust to device: adb push target/aarch64-linux-android/release/hma-rust /data/local/tmp/"
echo "4. Run: adb shell su -c '/data/local/tmp/hma-rust test'"
