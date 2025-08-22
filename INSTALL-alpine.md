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
    ./alpine/install.sh
    ```

    The script will build the `wire-sentinel` binary, install it to `/opt/wire-sentinel/bin`, copy the configuration file to `/etc/wire-sentinel/wire-sentinel.toml`, and set up the OpenRC service.
    The service is configured to start automatically on boot.
