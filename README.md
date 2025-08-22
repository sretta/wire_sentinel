PRE-RELEASE: DO NOT USE

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
