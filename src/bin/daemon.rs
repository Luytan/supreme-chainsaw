use std::{error::Error, future::pending};
use std::collections::HashMap;
use zbus::{connection, interface};
use tokio::sync::RwLock;
use repctl::{gpu, iommu};
use repctl::iommu::Device;

struct Daemon {
    current_mode: RwLock<u8>,
    // Cached PCI devices; kept for future 
    pci_devices: HashMap<String, Device>,
    gpu_list: HashMap<String, gpu::Gpu>,
}
impl Daemon {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let pci_devices = iommu::read_pci_devices()?;
        let gpu_list = gpu::list_gpu(pci_devices.clone())?;

        Ok(Self {
            current_mode: RwLock::new(0),
            pci_devices,
            gpu_list,
        })
    }
}
#[interface(name = "com.luytan.daemon")]
impl Daemon {
    /// Set the GPU mode.
    ///
    /// 0 = Integrated, 1 = Hybrid, 2 = VFIO.
    async fn set_mode(&self, mode: u8) -> String {
        let mut current_mode_lock = self.current_mode.write().await;
        match mode {
            0 => {
                for (_, gpu) in &self.gpu_list {
                    // Only unbind GPU if it's not the default/boot GPU
                    if !gpu.is_default() {
                        if let Err(e) = gpu::unbind_gpu(gpu.pci_address()) {
                            return format!("GPU unbind failed: {}", e);
                        }
                    }
                }
                *current_mode_lock = mode;
                format!("Set mode to {}", mode)
            }
            1 => {
                for (_, gpu) in &self.gpu_list {
                    // Only bind GPU if it's not the default/boot GPU
                    if !gpu.is_default() {
                        if let Err(e) = gpu::bind_gpu(gpu.pci_address()) {
                            return format!("GPU bind failed: {}", e);
                        }
                    }
                }
                *current_mode_lock = mode;
                format!("Set mode to {}", mode)
            }
            2 => {
                *current_mode_lock = mode;
                format!("Set mode to {}", mode)
            }
            _ => format!("Unknown mode={}", mode),
        }
    }
    /// Get the current GPU mode value.
    async fn get_mode(&self) -> u8 {
        *self.current_mode.read().await
    }
    /// List human-readable supported modes.
    async fn list_mode(&self) -> Vec<String> {
        vec![
            "Integrated".to_string(),
            "Hybrid".to_string(),
            "VFIO".to_string(),
        ]
    }
    }


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let daemon = Daemon::new()?;

    // Print GPU list at startup
    println!("Detected GPUs:");
    for gpu in daemon.gpu_list.values() {
        println!(
            "- [#{id}] {name} | pci={pci} | render={render} | default={default}",
            id = gpu.id(),
            name = gpu.name(),
            pci = gpu.pci_address(),
            render = gpu.render_node(),
            default = gpu.is_default()
        );
    }
    let _conn = connection::Builder::system()?
        .name("com.luytan.daemon")?
        .serve_at("/com/luytan/daemon", daemon)?
        .build()
        .await?;

    println!("Daemon started");

    pending::<()>().await;

    Ok(())
}