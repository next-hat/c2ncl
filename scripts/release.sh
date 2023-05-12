#!/bin/sh

# This script is used to release a new version of the project.

arch=amd64
pkg_name="c2ncl"
version=$(cat Cargo.toml | grep -m 1 "version = \"" | sed 's/[^0-9.]*\([0-9.]*\).*/\1/')
release_path="./target/${pkg_name}_${version}_${arch}"

# Clear existing .deb package directory
rm -fr "${release_path}"
# Create new directories structure for .deb package
mkdir -p "${release_path}"
mkdir -p "${release_path}"/DEBIAN
mkdir -p "${release_path}"/usr/local/bin
mkdir -p "${release_path}"/usr/local/man/man1

# Build static binary
export RUSTFLAGS="-C target-feature=+crt-static"
cargo build --release --target x86_64-unknown-linux-musl

# Pack the binary to reduce size
upx --best --lzma ./target/x86_64-unknown-linux-musl/release/${pkg_name}

# Copy binary
cp ./target/x86_64-unknown-linux-musl/release/${pkg_name} "${release_path}"/usr/local/bin
# Generate DEBIAN controll
cat > "${release_path}"/DEBIAN/control <<- EOM
Package: ${pkg_name}
Version: ${version}
Architecture: ${arch}
Maintainer: Next Hat team@next-hat.com
Description: Convert docker-compose.yaml to Nanocl StateFile.yml
EOM

mkdir -p ./target/debian
dpkg-deb --build --root-owner-group "${release_path}" ./target/debian/${pkg_name}_"${version}"_"${arch}".deb
