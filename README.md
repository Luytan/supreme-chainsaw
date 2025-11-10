# chainsaw (temporary name)

A tiny prototype to manage GPU mode on Linux. It discovers GPUs from sysfs and lets you switch between:

- 0 = Integrated
- 1 = Hybrid
- 2 = VFIO, NOT IMPLEMENTED

Status: prototype. Expect rough edges.

## Warning
This tool was tested on a amd/amd hybrid asus rog laptop, there might be issues with non-asus/nvidia laptops 

## TODO

### High Priority
- [ ] **Configuration persistence**: Save and restore GPU mode across reboots
- [ ] **VFIO mode implementation**: Bind dGPU to vfio-pci driver for VM passthrough
- [ ] **Nvidia dGPU support**: Test and fix compatibility with Nvidia hybrid laptops

### Medium Priority
- [ ] **Multi-GPU support**: Handle laptops with eGPU or multiple dGPUs
- [ ] **PCI rescan optimization**: Investigate faster alternatives to full bus rescan
- [ ] **Error recovery**: Graceful handling of failed bind/unbind operations

### Future Enhancements
- [ ] **Generic device passthrough**: Extend VFIO support to other PCI devices
- [ ] **Auto-switching**: Intelligent mode switching based on battery state(switch to integrated on battery)
- [ ] **GUI frontend**: Simple graphical interface for mode switching on kde and gnome

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
