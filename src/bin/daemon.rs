use std::{error::Error, future::pending};
use zbus::{connection, interface};
use tokio;
use tokio::sync::RwLock;
struct Daemon {
    current_mode: RwLock<u8>,
}
impl Daemon {
    fn new() -> Self {
        Self {
            current_mode: RwLock::new(0), // Need to change the 0
        }
    }
}
#[interface(name = "com.luytan.daemon")]
impl Daemon {
    async fn set_mode(&self, mode: u8) -> String {
        let mut current_mode_lock = self.current_mode.write().await;
        match mode {
            0 |1|2 => {
                *current_mode_lock = mode;
                format!("Set mode to {}", mode)
            }
            _ => format!("Unknown mode={}", mode),
        }
    }
    async fn get_mode(&self) -> u8 {
        *self.current_mode.read().await
    }
}


// Although we use `tokio` here, you can use any async runtime of choice.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _conn = connection::Builder::session()?
        .name("com.luytan.daemon")?
        .serve_at("/com/luytan/daemon", Daemon { current_mode: 0.into() })?
        .build()
        .await?;

    println!("Daemon started");

    pending::<()>().await;

    Ok(())
}