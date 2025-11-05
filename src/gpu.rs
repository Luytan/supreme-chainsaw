use std::collections::HashMap;
use std::fs;
use std::path::{Path};
use crate::iommu;
use crate::iommu::Device;
/// Struct representing a GPU device
#[derive(Debug, Clone)]
pub struct Gpu {
    id: usize,
    name: String,
    pci: String,
    render: String,
    default: bool,
}

impl Gpu {
    /// Returns the PCI address of the GPU
    pub fn pci_address(&self) -> &str {
        &self.pci
    }

    /// Returns true if this GPU is the default/boot GPU
    pub fn is_default(&self) -> bool {
        self.default
    }

    /// Returns the numeric id assigned during discovery
    pub fn id(&self) -> usize {
        self.id
    }

    /// Returns the human-readable device name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the DRM render node path (e.g., /dev/dri/renderD128)
    pub fn render_node(&self) -> &str {
        &self.render
    }
}

/// Will search all device to find a gpu via driver, in the future this might be integrated to the
/// iommu::read_iommu_groups function
pub fn list_gpu(pci_devices: HashMap<String, Device>) -> Result<HashMap<String, Gpu>, Box<dyn std::error::Error>> {
    let mut gpu_map: HashMap<String, Gpu> = HashMap::new();
    let mut i:usize = 0;
    for (_, device) in &pci_devices {
        match device.class.as_str() {
            "0x030000" => {
                let boot_vga_path = Path::new("/sys/bus/pci/devices").join(&device.pci_address).join("boot_vga");
                // Read the content of the boot_vga file as a string
                let boot_vga_content = fs::read_to_string(&boot_vga_path)?;
                // Parse the string to determine if it's default
                // Trim whitespace and check the first character
                let is_default = boot_vga_content.trim() == "1";
                let gpu = Gpu {
                    id: i,
                    name: device.device_name.clone(),
                    pci: device.pci_address.clone(),
                    render: get_render(device.pci_address.clone()),
                    default: is_default,
                };
                gpu_map.insert(gpu.id.to_string(), gpu);
                i += 1;
            }
            _ => continue,
        }
    }
    Ok(gpu_map)
}
/// unbind the gpu using the pci bus
/// before unbinding, the function should check if the gpu is in use via fn is_sleeping
pub fn unbind_gpu(pci_address: &str) -> Result<(), std::io::Error> {
    // Call is_sleeping
    let is_device_sleeping = is_sleeping(pci_address)?;

    let pci_path = Path::new("/sys/bus/pci/devices").join(pci_address);
    let unbind_path = pci_path.join("driver").join("unbind");

    match is_device_sleeping {
        true => {
            // Device is sleeping (D3cold), proceed with unbind
            fs::write(unbind_path, pci_address)?;
            Ok(()) // Return Ok(()) after successful write
        }
        false => {
            // Device is not sleeping, return an error
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Device is not in sleep state (D3cold), cannot safely unbind", // Fixed typo
            ))
        }
    }
}
/// Re-bind the GPU to its driver.
///
/// Currently this triggers a PCI rescan; verification that the correct driver bound is
/// not yet implemented.
pub fn bind_gpu(pci_address: &str) -> Result<String, std::io::Error> {
    let pci_path = Path::new("/sys/bus/pci/devices").join(pci_address);
    let remove_path = pci_path.join("remove");
    fs::write(remove_path, "1")?;
    match iommu::pci_rescan(){
        Ok(..) => {
            Ok("Rescan issued; verification not yet implemented".to_string())
        },
        Err(e) => Err(e)
    }

}

/// Checks the device power_state status
fn is_sleeping(pci_address: &str) -> Result<bool, std::io::Error> {
    let pci_path = Path::new("/sys/bus/pci/devices").join(pci_address);
    let power_state_path = pci_path.join("power_state");
    let content = fs::read_to_string(&power_state_path)?;
    if content.trim() == "D3cold" {
        Ok(true)
    } else {
        Ok(false)
    }
}
fn get_process_name(pid: String) -> Option<String> {
    std::fs::read_to_string(format!("/proc/{}/status", pid))
        .ok()?
        .lines()
        .next()?
        .split_whitespace()
        .nth(1)
        .map(|s| s.to_string())
}
fn get_render(pci_address: String) -> String {
    let dri_path = format!("/dev/dri/by-path/{}-render", pci_address);
    if let Ok(render) = fs::read_link(&dri_path) {
        let render_path = render.to_string_lossy().replace("../", "/dev/dri/");
        render_path.to_string()
    } else {
        "Error: couldn't read symlink for render node".to_string()
    }
}
