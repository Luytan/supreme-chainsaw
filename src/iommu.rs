use std::collections::HashMap;
use std::path::Path;
use std::{fs, io};

// struct for IommuGroup
pub struct IommuGroup {
    pub id: usize,            // group number
    pub devices: Vec<String>, // a vec containing the pci address of the devices
}
// struct for Devices
pub struct Devices {
    pub pci_address: String, //address of the pci device, example: 0000:00:08.0 for a gpu
    pub iommu_group: usize,  // id of the iommu group
    pub vendor_id: String,
    pub device_id: String,
    pub vendor_name: String,
    pub device_name: String,
    pub driver: String,
}

pub fn read_iommu_groups() -> std::io::Result<HashMap<usize, IommuGroup>> {
    let base_path = Path::new("/sys/kernel/iommu_groups");

    // Check if IOMMU groups are available
    if !base_path.exists() {
        eprintln!("IOMMU groups not found. Is IOMMU enabled in the kernel?");
        return Ok(HashMap::new());
    }

    let mut groups: HashMap<usize, IommuGroup> = HashMap::new();

    // Iterate over each entry in /sys/kernel/iommu_groups/
    for entry in fs::read_dir(base_path)? {
        let entry = entry?;
        let group_dir = entry.path();

        // Group ID is the directory name (e.g., "1", "2", ...)
        let group_id_str = group_dir
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid group directory name",
                )
            })?;

        let group_id: usize = match group_id_str.parse() {
            Ok(id) => id,
            Err(_) => continue, // Skip non-numeric entries (shouldn't happen, but safe)
        };

        // Devices are in the "devices" subdirectory, example: iommu_groups/0/devices/pci_address
        let devices_dir = group_dir.join("devices");
        let mut devices = Vec::new();
        // if the devices dir exist, will get all the pci address
        if devices_dir.exists() {
            for device_entry in fs::read_dir(&devices_dir)? {
                // if ok, store path in device_entry
                let device_entry = device_entry?;
                // get file_name without the path
                let device_name = device_entry.file_name();
                // convert device_name from OsString to UTF-8
                if let Some(name_str) = device_name.to_str() {
                    // push to devices vector
                    devices.push(name_str.to_string());
                }
            }
        }
        // insert a new iommu in the hashmap
        groups.insert(
            group_id, // key
            IommuGroup {
                // value
                id: group_id,
                devices,
            },
        );
    }

    Ok(groups)
}

/* For each pci device, will try to get the:
 * attached driver, the vendor id/name, the device id/name
 *
 *
 */
pub fn read_pci_devices() -> std::io::Result<HashMap<String, Devices>> {
    let iommu_groups = read_iommu_groups()?;
    // map for devices
    let mut devices_map: HashMap<String, Devices> = HashMap::new();

    for (group_id, group) in iommu_groups {
        for pci_address in group.devices {
            let vendor_id = get_vendor_id(&pci_address)?;
            let device_id = get_device_id(&pci_address)?;
            let vendor_name = get_vendor_name(&pci_address)?;
            let device_name = get_vendor_name(&pci_address)?;
            let driver = get_driver(&pci_address);
            // create a device struct
            let device = Devices {
                pci_address: pci_address.clone(),
                iommu_group: group_id,
                vendor_id: vendor_id,
                device_id: device_id,
                vendor_name: vendor_name,
                device_name: device_name,
                driver: driver,
            };
            devices_map.insert(pci_address, device);
        }
    }
    Ok(devices_map)
}
pub fn list_iommu_groups() -> std::io::Result<()> {
    let iommu_groups = read_iommu_groups()?;

    if iommu_groups.is_empty() {
        println!("No IOMMU groups found.");
        return Ok(());
    }

    println!("IOMMU Groups detected:\n");
    for group_id in iommu_groups.keys() {
        let group = &iommu_groups[&group_id];
        println!("Group {}: {:?}", group.id, group.devices);
    }
    Ok(())
}
pub fn list_pci_devices() -> std::io::Result<()> {
    let pci_devices = read_pci_devices()?;
    if pci_devices.is_empty() {
        println!("No device??");
        return Ok(());
    }
    for pci_address in pci_devices.keys() {
        let device = &pci_devices[&pci_address.to_string()];
        println!("Device: {}", device.pci_address);
        println!("| IOMMU GROUP: {}", device.iommu_group);
        println!("| VENDOR_ID: {}", device.vendor_id);
        println!("| DEVICE_ID: {}", device.device_id);
        println!("| DRIVER: {}", device.driver);
        println!();
    }
    Ok(())
}

fn get_vendor_id(pci_address: &str) -> io::Result<String> {
    // /sys/bus/pci/devices/{PCI}/vendor
    let vendor_path = Path::new("/sys/bus/pci/devices/")
        .join(&pci_address)
        .join("vendor");
    let content = fs::read_to_string(&vendor_path)?;
    Ok(content.trim_end().to_string())
}
fn get_device_id(pci_address: &str) -> io::Result<String> {
    // /sys/bus/pci/devices/{PCI}/device
    let device_path = Path::new("/sys/bus/pci/devices/")
        .join(&pci_address)
        .join("device");
    let content = fs::read_to_string(&device_path)?;
    Ok(content.trim_end().to_string())
}
fn get_vendor_name(pci_address: &str) -> io::Result<String> {
    // IDK HOW
    Ok("NOT IMPLEMENTED".to_string())
}
fn get_device_name(pci_address: &str) -> io::Result<String> {
    // will do later
    Ok("NOT IMPLEMENTED".to_string())
}
fn get_driver(pci_address: &str) -> String {
    let driver_path = Path::new("/sys/bus/pci/devices/")
        .join(pci_address)
        .join("driver");

    fs::read_link(&driver_path)
        .ok() // ignore les erreurs
        .and_then(|p| p.file_name().map(|n| n.to_string_lossy().into_owned()))
        .unwrap_or_else(|| "none".to_string())
}
