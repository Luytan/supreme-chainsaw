# repctl

A tiny prototype to manage GPU mode on Linux via D-Bus. It discovers GPUs from sysfs and lets you switch between:

- 0 = Integrated
- 1 = Hybrid
- 2 = VFIO, NOT IMPLEMENTED

Status: prototype. Expect rough edges.

## Warning
This tool was tested on a amd/amd hybrid asus rog laptop, there might be issues with non-asus/nvidia laptops 

## Requirements

- Linux with IOMMU enabled and `/sys/bus/pci` available
- Rust toolchain (cargo)
- D-Bus session bus (daemon and CLI talk on the session bus)
- Permissions to write to PCI sysfs (you may need to run the daemon as root)

## Build

```bash
# from the repository root
make build
```
## Install
```bash
sudo make install
```

## Run

The daemon systemctl service should be enabled and started automatically:
systemctl status chainsawd.service

Query and set mode using the CLI:

```bash
# Show supported modes
chainsaw list

# Get current mode
chainsaw get

# Set mode (0 = Integrated, 1 = Hybrid, 2 = VFIO)
chainsaw set 1
```

Tip: use `chainsaw --help` to see the exact command syntax.

## Notes

- This tool touches PCI devices under `/sys/bus/pci`; use with care on production systems.
