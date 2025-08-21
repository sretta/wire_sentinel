# Installation Instructions

These instructions explain how to install and run `wire_sentinel` as a systemd service.

## 1. Build the application

First, build the application binary:

```bash
cargo build --release
```

The binary will be located at `target/release/wire_sentinel`.

## 2. Create user and group

For security, the service should run as a dedicated non-root user. Create the `wire_sentinel` user and group:

```bash
sudo groupadd --system wire_sentinel
sudo useradd --system --no-create-home --gid wire_sentinel wire_sentinel
```

## 3. Create working directory and copy files

The service is configured to use `/opt/wire_sentinel` as its working directory. This is where the configuration file will be stored.

```bash
# Create the directory
sudo mkdir -p /opt/wire_sentinel
```

Copy the compiled binary to `/usr/local/bin`:
```bash
sudo cp target/release/wire_sentinel /usr/local/bin/
```

## 4. Create configuration file

Create the configuration file at `/opt/wire_sentinel/wire-sentinel.toml`.
Make sure to fill in the correct values for your setup.

```bash
sudo touch /opt/wire_sentinel/wire-sentinel.toml
# It is recommended to use a secure method to edit this file with the required secrets.
# For example, using 'sudoedit' or another secure editor.
sudo nano /opt/wire_sentinel/wire-sentinel.toml # or your favorite editor
```

An example `wire-sentinel.toml` would be:
```toml
[config]
internal_interface = "eth0"
wg_interface = "wg0"
bearer_token = "your_gandi_api_key"
domain = "your_domain.com"
peer_pubkey = "your_wireguard_peer_public_key"
peer_hostname = "peer.your_domain.com"
own_hostname = "host.your_domain.com"
```

## 5. Set ownership

Set the correct ownership for the working directory:
```bash
sudo chown -R wire_sentinel:wire_sentinel /opt/wire_sentinel
```

## 6. Install the systemd service

Copy the service file to the systemd directory:
```bash
sudo cp wire_sentinel.service /etc/systemd/system/
```

## 7. Enable and start the service

Reload the systemd daemon, enable the service to start on boot, and start it now:
```bash
sudo systemctl daemon-reload
sudo systemctl enable wire_sentinel.service
sudo systemctl start wire_sentinel.service
```

You can check the status of the service with:
```bash
sudo systemctl status wire_sentinel.service
```

And view the logs with:
```bash
sudo journalctl -u wire_sentinel.service -f
```
