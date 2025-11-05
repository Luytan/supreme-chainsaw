# repctl

A tiny prototype to manage GPU mode on Linux via D-Bus. It discovers GPUs from sysfs and lets you switch between:

- 0 = Integrated
- 1 = Hybrid
- 2 = VFIO, NOT IMPLEMENTED

Status: prototype. Expect rough edges.

## Requirements

- Linux with IOMMU enabled and `/sys/bus/pci` available
- Rust toolchain (cargo)
- D-Bus session bus (daemon and CLI talk on the session bus)
- Permissions to write to PCI sysfs (you may need to run the daemon as root)

## Build

```bash
# from the repository root
cargo build --release
```

Artifacts:
- `target/release/daemon`
- `target/release/cli`

## Run
MUST BE IN ROOT

Start the daemon (session bus):

```bash
./target/release/daemon
```
The default mode is 0, since i didn't implement a check at startup
Query and set mode using the CLI:

```bash
# Show supported modes
./target/release/cli list-mode

# Get current mode
./target/release/cli get-mode

# Set mode (0 = Integrated, 1 = Hybrid, 2 = VFIO)
./target/release/cli set-mode 1
```

Tip: use `./target/release/cli --help` to see the exact command syntax.

## Notes

- This tool touches PCI devices under `/sys/bus/pci`; use with care on production systems.
