use clap::Parser;
use types::{Action, Args};

use crate::modules::{
    block_module::Blocker, boostrap, scan_module::Scanner, show_module::Shower,
    unblock_module::Unblocker,
};

pub mod types;

pub async fn handle_command() {
    let args = Args::parse();

    match args.command {
        Action::Scan { config } => {
            boostrap::init_app(config).await;

            println!("✅ INITIATING SCANNING");

            let scan = Scanner::new().await;

            let suspects = scan.dispatch().await;

            println!("✅ SCAN COMPLETED");

            Scanner::display_suspects(suspects);
        }
        Action::Block { config, ip, reason } => {
            boostrap::init_app(config).await;

            println!("✅ STARTING BLOCK PROCESS");

            let blocker = Blocker::new().await;

            blocker.block_ip(&ip, &reason).await;

            blocker.sync_latest().await;

            blocker.reload_server();
        }
        Action::Show { config } => {
            boostrap::init_app(config).await;

            let shower = Shower::new().await;

            shower.dispatch().await;
        }
        Action::Unblock { config, ip } => {
            boostrap::init_app(config).await;

            let unblocker = Unblocker::new().await;

            unblocker.unblock_ip(&ip).await;

            unblocker.reload_server();

            println!("✅ UNBLOCKED IP");
        }
        Action::ScanBlock { config } => {
            boostrap::init_app(config).await;

            println!("✅ INITIATING SCANNING");

            let scanner = Scanner::new().await;

            let suspects = scanner.dispatch().await;

            println!("✅ SCAN COMPLETED");

            println!("✅ STARTING BLOCK PROCESS");

            let blocker = Blocker::new().await;

            for (ip, reason) in suspects {
                blocker.block_ip(&ip, &reason).await;
            }

            blocker.sync_latest().await;

            blocker.reload_server();

            println!("✅ PROCESS COMPLETED");
        }
    }
}
