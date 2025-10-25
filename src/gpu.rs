use std::fs;
use std::path::Path;
use crate::iommu;

pub fn unbind_gpu(sys_path: &str, pci_id: &str) -> Result<(), std::io::Error> {
    let base_path = Path::new("/sys");
    let path_to_join = Path::new(sys_path);

    // Removes the / from the start of sys_path
    let relative_path = if let Ok(path) = path_to_join.strip_prefix("/") {
        path
    } else {
        path_to_join
    };
    let unbind_path = base_path.join(relative_path).join("driver").join("unbind");

    println!("Path: {}", unbind_path.to_string_lossy());
    println!("pci_id: {}", pci_id);
    // opening the unbind file
    fs::write(unbind_path, pci_id)?;
    Ok(())
}