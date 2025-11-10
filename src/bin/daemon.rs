use std::collections::HashMap;
use std::{error::Error, future::pending};

use chainsaw::iommu::Device;
use chainsaw::{gpu, iommu};
use tokio::sync::RwLock;
use zbus::{connection, fdo, interface};

struct Daemon {
    current_mode: RwLock<u8>,
    gpu_list: HashMap<String, gpu::Gpu>,
    // Cached PCI devices for future features.
    _pci_devices: HashMap<String, Device>,
}
impl Daemon {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let pci_devices = iommu::read_pci_devices()?;
        let gpu_list = gpu::list_gpu(&pci_devices)?;
        
        // Find the discrete GPU (non-default) to check its bound state
        let dgpu_pci = gpu_list
            .values()
            .find(|gpu| !gpu.is_default())
            .map(|gpu| gpu.pci_address());
        
        // Determine initial mode: if dGPU exists and is bound, mode=1 (Hybrid), else mode=0 (Integrated)
        let mode = match dgpu_pci {
            Some(pci_addr) => {
                if gpu::is_dgpu_bound(pci_addr)? {
                    1 // Hybrid
                } else {
                    0 // Integrated
                }
            }
            None => 0, // No discrete GPU found, default to Integrated
        };

        Ok(Self {
            current_mode: RwLock::new(mode),
            _pci_devices: pci_devices,
            gpu_list,
        })
    }
}
#[interface(name = "com.chainsaw.daemon")]
impl Daemon {
    /// Set the GPU mode.
    ///
    /// 0 = Integrated, 1 = Hybrid, 2 = VFIO.
    async fn set_mode(&self, mode: u8) -> fdo::Result<String> {
        let mut current_mode_lock = self.current_mode.write().await;
        match mode {
            0 => {
                for gpu in self.gpu_list.values() {
                    // Only unbind GPU if it's not the default/boot GPU
                    if !gpu.is_default() && let Err(e) = gpu::unbind_gpu(gpu.pci_address(), gpu.slot()) {
                        return Err(fdo::Error::Failed(format!(
                            "Failed to unbind GPU {}: {}",
                            gpu.pci_address(),
                            e
                        )));
                    }
                }
                *current_mode_lock = mode;
                Ok(format!("Set mode to {}", mode))
            }
            1 => {
                for gpu in self.gpu_list.values() {
                    // Only bind GPU if it's not the default/boot GPU
                    if !gpu.is_default() && let Err(e) = gpu::bind_gpu(gpu.pci_address(), gpu.slot()) {
                        return Err(fdo::Error::Failed(format!(
                            "Failed to bind GPU {}: {}",
                            gpu.pci_address(),
                            e
                        )));
                    }
                }
                *current_mode_lock = mode;
                Ok(format!("Set mode to {}", mode))
            }
            2 => {
                *current_mode_lock = mode;
                Ok(format!("Set mode to {}", mode))
            }
            _ => Err(fdo::Error::InvalidArgs(format!("Unknown mode={}", mode))),
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
        .name("com.chainsaw.daemon")?
        .serve_at("/com/chainsaw/daemon", daemon)?
        .build()
        .await?;

    println!("Daemon started");

    pending::<()>().await;

    Ok(())
}
