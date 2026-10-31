# SigmaOS Bluetooth

This crate exposes Bluetooth adapter, device, and manager APIs. The simple
adapter and manager store adapter data and state in memory; their scan and
state operations are not proof of discovery, pairing, or radio control on a
physical Bluetooth adapter. Hardware integration and permission handling need
separate implementation and device evidence.

The crate depends on the sibling `SigmaOS-kernel` checkout through a local
path.

## License

GPL-3.0-or-later. See [`LICENSE`](LICENSE) and [`Cargo.toml`](Cargo.toml).
