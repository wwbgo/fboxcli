use clap::Subcommand;
use anyhow::Result;
use crate::api::FBoxClient;
use crate::output::{OutputFormat, print_list, print_single};

#[derive(Subcommand)]
pub enum DeviceCmd {
    /// List devices for a box
    List {
        /// FBox ID
        box_id: i64,
    },
    /// List available drivers for a box type
    Drivers {
        /// Box type: 0=Standard, 1=Mini, 2=Lite, 3=VPN
        #[arg(default_value = "0")]
        box_type: i32,
    },
    /// Get register info for a device
    Registers {
        /// Device ID
        device_id: i64,
    },
}

pub async fn handle(cmd: DeviceCmd, client: &mut FBoxClient, format: OutputFormat) -> Result<()> {
    match cmd {
        DeviceCmd::List { box_id } => {
            let items = crate::api::device::list_devices(client, box_id).await?;
            print_list(&items, format)?;
        }
        DeviceCmd::Drivers { box_type } => {
            let specs = crate::api::device::get_drivers(client, box_type).await?;
            print_list(&specs, format)?;
        }
        DeviceCmd::Registers { device_id } => {
            let regs = crate::api::device::get_registers(client, device_id).await?;
            print_single(&regs, format)?;
        }
    }
    Ok(())
}
