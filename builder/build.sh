# Copyright (c) 2025 mcpeaps_HD
#
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT
cargo build --target aarch64-apple-darwin
cargo build --target aarch64-pc-windows-msvc --verbose
cargo build --target aarch64-pc-windows-gnullvm --verbose
cargo build --target i586-pc-windows-msvc --verbose
cargo build --target i586-unknown-linux-gnu --verbose
cargo build --target i586-unknown-linux-musl --verbose
cargo build --target i686-pc-windows-gnu --verbose
cargo build --target i686-pc-windows-gnullvm --verbose
cargo build --target i686-pc-windows-msvc --verbose
cargo build --target i686-unknown-freebsd --verbose
cargo build --target i686-unknown-linux-gnu --verbose
cargo build --target i686-unknown-linux-musl --verbose
cargo build --target x86_64-apple-darwin --verbose
cargo build --target x86_64-pc-windows-gnu --verbose
cargo build --target x86_64-pc-windows-gnullvm --verbose
cargo build --target x86_64-pc-windows-msvc --verbose
cargo build --target x86_64-unknown-freebsd --verbose
cargo build --target x86_64-unknown-linux-gnu --verbose
cargo build --target x86_64-unknown-linux-gnux32 --verbose
cargo build --target x86_64-unknown-linux-musl --verbose
cargo build --target x86_64-unknown-linux-ohos --verbose
cargo build --target x86_64-unknown-netbsd --verbose
