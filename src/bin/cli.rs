use clap::{Parser, Subcommand};
use zbus;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set the mode
    SetMode {
        /// Mode
        /// 0 = Integrated
        /// 1 = Hybrid
        /// 2 = VFIO
        mode: u8,
    },
    /// Get the current mode
    GetMode,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let connection = zbus::connection::Builder::session()?
        .build()
        .await?;

    let proxy = zbus::Proxy::new(
        &connection,
        "com.luytan.daemon",
        "/com/luytan/daemon",
        "com.luytan.daemon",
    ).await?;

    match args.command {
        Commands::SetMode { mode } => {
            let response: String= proxy.call("SetMode", &(mode,)).await?;
            println!("{}", response);
        }
        Commands::GetMode => {
            let current_mode: u8 = proxy.call("GetMode", &()).await?;
            println!("Current gpu mode: {}", current_mode);
        }
    }

    Ok(())
}