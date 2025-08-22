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

echo "Copy the service file to the systemd directory"
sudo cp systemd/wire_sentinel.service /etc/systemd/system/

echo "enable & start wire-sentinel service"
sudo systemctl daemon-reload
sudo systemctl enable wire_sentinel.service
sudo systemctl start wire_sentinel.service

echo "Installation complete."
