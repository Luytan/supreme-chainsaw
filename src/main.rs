mod iommu;

fn main() -> std::io::Result<()> {
    let iommu_groups = iommu::read_iommu_groups()?;

    if iommu_groups.is_empty() {
        println!("No IOMMU groups found.");
        return Ok(());
    }

    println!("IOMMU Groups detected:\n");
    for group_id in iommu_groups.keys() {
        let group = &iommu_groups[&group_id];
        println!("Group {}: {:?}", group.id, group.devices);
    }
    println!();
    println!("--------------");
    println!();
    let pci_devices = iommu::read_pci_devices()?;
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
