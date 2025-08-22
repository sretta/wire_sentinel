# Installation on SystemD based Linux

This document describes how to install and run `wire-sentinel` as a SystemD service.

## Prerequisites

*   Rust and Cargo
*   A Linux system with SystemD

## Installation

1.  Clone this repository to your Linux machine.
2.  Navigate to the repository directory.
3.  Run the installation script as root:

    ```sh
    ./systemd/install.sh
    ```

    The script will build the `wire-sentinel` binary, install it to `/opt/wire-sentinel/bin`, copy the configuration file to `/etc/wire-sentinel/wire-sentinel.toml`, and set up the SystemD service.
    The service is configured to start automatically on boot.

## Monitor the service

You can check the status of the service with:
```bash
sudo systemctl status wire_sentinel.service
```

And view the logs with:
```bash
sudo journalctl -u wire_sentinel.service -f
```
