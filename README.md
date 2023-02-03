`corewlan-sys`
===
FFI Bindings to MacOS's CoreWLAN framework.

Usage
---
You can either read the docs at [docs.rs](https://docs.rs/corewlan-sys/latest) or [apple.com](https://developer.apple.com/documentation/corewlan?language=objc). A small example is provided below.
```rs
extern crate corewlan-sys;

fn main() -> Result<(), ()> {
    let client = CWWiFiClient::sharedWiFiClient();
    let interface = client.interface();
    let networks = interface.scanForNetworksWithName(None)?;
    for network in networks {
        println!("SSID: {}", network.ssid());
    }
}
```

Disclaimer
---
This crate does not have every interface in Apple's documentation perfectly implimented 1:1. Some things will be missing, some things will have bugs.  
If you find a bug or need a feature, open an issue. I'll try to resolve it as best and quick as I can. Better yet, make a PR!
