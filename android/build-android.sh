#!/bin/bash
# Build Android UI with Rust library

set -e

echo "=== Building Hide-My-Applist Android UI ==="

# 1. Build Rust library for Android
echo "Building Rust library..."
cd ..
cargo build --release --target aarch64-linux-android --features android

# 2. Copy library to Android project
echo "Copying library..."
cp target/aarch64-linux-android/release/libhide_my_applist_rust.so \
   android/app/src/main/jniLibs/arm64-v8a/

# 3. Build Android APK
echo "Building Android APK..."
cd android
./gradlew assembleRelease

echo "✓ Build complete!"
echo "APK: android/app/build/outputs/apk/release/app-release.apk"
