#!/bin/sh

set -e

if [ "$(id -u)" -ne 0 ]; then
    echo "This script must be run as root" >&2
    exit 1
fi

echo "Building wire_sentinel binary..."
cargo build --release

echo "Installing wire_sentinel..."

echo "Creating /etc/wire-sentinel directory..."
mkdir -p /etc/wire-sentinel

echo "Copying wire-sentinel.toml to /etc/wire-sentinel/"
cp wire-sentinel.toml /etc/wire-sentinel/

echo "Copying wire_sentinel binary to /usr/local/bin/"
cp target/release/wire_sentinel /usr/local/bin/

echo "Copying wire-sentinel init script to /etc/init.d/"
cp dist/wire-sentinel /etc/init.d/

echo "Making init script executable..."
chmod +x /etc/init.d/wire-sentinel

echo "Adding wire-sentinel service to default runlevel..."
rc-update add wire-sentinel default

echo "Installation complete."
echo "To start the service, run: rc-service wire-sentinel start"
