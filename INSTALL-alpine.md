# Installation on Alpine Linux

This document describes how to install and run `wire-sentinel` as a service on Alpine Linux.

## Prerequisites

*   Rust and Cargo
*   OpenRC
*   An Alpine Linux system

## Installation

1.  Clone this repository to your Alpine Linux machine.
2.  Navigate to the repository directory.
3.  Run the installation script as root:

    ```sh
    ./install.sh
    ```

    The script will build the `wire-sentinel` binary, install it to `/usr/local/bin`, copy the configuration file to `/etc/wire-sentinel/wire-sentinel.toml`, and set up the OpenRC service.

## Configuration

Before starting the service, you must edit the configuration file at `/etc/wire-sentinel/wire-sentinel.toml` and replace the placeholder values with your actual data.

```toml
[config]
internal_interface = "eth0"
wg_interface = "wg0"
bearer_token = "YOUR_GANDI_API_KEY"
domain = "your.domain.tld"
peer_pubkey = "YOUR_PEER_PUBLIC_KEY"
peer_hostname = "peer-hostname"
own_hostname = "own-hostname"
```

## Managing the Service

You can manage the `wire-sentinel` service using the `rc-service` command.

*   **Start the service:**

    ```sh
    rc-service wire-sentinel start
    ```

*   **Stop the service:**

    ```sh
    rc-service wire-sentinel stop
    ```

*   **Check the status of the service:**

    ```sh
    rc-service wire-sentinel status
    ```

The service is configured to start automatically on boot.
