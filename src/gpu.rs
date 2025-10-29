use std::fs;
use std::path::Path;
use crate::iommu;

/// Will search all device to find a gpu via driver, in the future this might be integrated to the
/// iommu::read_iommu_groups function
// another TODO: detect dgpu
pub fn list_gpu() -> std::io::Result<()>{
    let pci_devices = iommu::read_pci_devices()?;
    for pci_address in pci_devices.keys() {
        let device = &pci_devices[&pci_address.to_string()];
        match device.driver.to_string().as_str() {
            "amdgpu" | "nouveau" | "radeon" => println!("{:?}", device),
            _ => continue,
        }
        }
    Ok(())
}
/// unbind the gpu using the pci bus
// TODO: add safeguard (if sleeping or GPU in use)
pub fn unbind_gpu(pci_id: &str) -> Result<(), std::io::Error> {
    let pci_path = Path::new("/sys/bus/pci/devices").join(pci_id);
    let unbind_path = pci_path.join("driver").join("unbind");

    println!("Path: {}", unbind_path.to_string_lossy());
    println!("pci_id: {}", pci_id);
    // opening the unbind file
    //fs::write(unbind_path, pci_id)?;
    Ok(())
}
/// re-bind the gpu to the driver
pub fn bind_gpu() {}

/// Checks the device power_state status
fn get_power_state() {}
fn get_process_name(pid: String) -> Option<String> {
    std::fs::read_to_string(format!("/proc/{}/status", pid))
        .ok()?
        .lines()
        .next()?
        .split_whitespace()
        .nth(1)
        .map(|s| s.to_string())
}
