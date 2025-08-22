#!/bin/sh

set -e

echo "Building wire_sentinel binary..."
cargo build --release

echo "Installing wire_sentinel..."

echo "Creating /etc/wire-sentinel directory..."
sudo mkdir -p /etc/wire-sentinel

echo "Copying wire-sentinel.toml to /etc/wire-sentinel/"
sudo cp wire-sentinel.toml /etc/wire-sentinel/

echo "Creating /opt/wire-sentinel/bin directory..."
sudo mkdir -p /opt/wire-sentinel/bin

echo "Copying wire_sentinel binary to /usr/local/bin/"
sudo cp target/release/wire_sentinel /opt/wire-sentinel/bin/

echo "Copying wire-sentinel init script to /etc/init.d/"
sudo cp alpine/wire-sentinel /etc/init.d/

echo "Making init script executable..."
sudo chmod +x /etc/init.d/wire-sentinel

echo "Adding wire-sentinel service to default runlevel..."
sudo rc-update add wire-sentinel default

echo "Installation complete."
echo "To start the service, run: rc-service wire-sentinel start"
