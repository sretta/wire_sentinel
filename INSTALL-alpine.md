# Installation on Alpine Linux

This document describes how to install and run `wire-sentinel` as a service on Alpine Linux.

## Prerequisites

*   Rust and Cargo
*   An Alpine Linux system with OpenRC

## Installation

1.  Clone this repository to your Alpine Linux machine.
2.  Navigate to the repository directory.
3.  Run the installation script as root:

    ```sh
    ./install.sh
    ```

    The script will build the `wire-sentinel` binary, install it to `/opt/wire-sentinel/bin`, copy the configuration file to `/etc/wire-sentinel/wire-sentinel.toml`, and set up the OpenRC service.
    The service is configured to start automatically on boot.

## Configuration

Before starting the service, you must edit the configuration file at `/etc/wire-sentinel/wire-sentinel.toml` and replace the placeholder values with your actual data.

```toml
[config]
internal_interface = "eth0"
wg_interface = "wg0"
bearer_token = "YOUR_GANDI_API_KEY"
domain = "your.domain.tld"
peer_pubkey = "YOUR_WIREGUARD_PEER_PUBLIC_KEY"
peer_hostname = "peer-hostname"
own_hostname = "own-hostname"
```
