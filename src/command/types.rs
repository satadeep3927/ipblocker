use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    version,
    name = "IronWatch",
    author,
    about = "A basic command-line interface for interacting with the IronWatch Tool.",
    long_about = "A simple command-line interface (CLI) designed to provide users with an intuitive way to interact with the IronWatch Tool. This interface allows users to easily perform actions such as blocking or unblocking IP addresses, viewing blocked IP lists, and managing settings through a series of straightforward commands. The CLI is built for efficiency, providing quick access to the tool's features without the need for a graphical user interface, making it ideal for system administrators and users who prefer working within a terminal environment."
)]
pub struct Args {
    /// action to run
    #[command(subcommand)]
    pub command: Action,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Subcommand)]
pub enum Action {
    /// Scan and block potentially malicious IP addresses.
    ScanBlock {
        /// Path to Configuration File
        #[arg(short, long, default_value = "config.json")]
        config: String,
    },
    /// Scan potentially malicious IP addresses
    Scan {
        /// Path to Configuration File
        #[arg(short, long, default_value = "config.json")]
        config: String,
    },
    /// Block Given IP Address
    Block {
        /// Path to Configuration File
        #[arg(short, long, default_value = "config.json")]
        config: String,
        /// IP address to block
        #[arg(short, long)]
        ip: String,
        /// Reason to block
        #[arg(short, long, default_value = "")]
        reason: String,
    },
    /// Show Blocked IP Addresses
    Show {
        /// Path to Configuration File
        #[arg(short, long, default_value = "config.json")]
        config: String,
    },
    /// Unblock IP Address
    Unblock {
        #[arg(short, long, default_value = "config.json")]
        config: String,
        /// IP address to block
        #[arg(short, long)]
        ip: String,
    },
}
