use clap::Parser;

/// Command-line interface for mdlinkcheck
#[derive(Parser, Debug)]
#[command(name = "mdlinkcheck")]
#[command(about = "Check markdown links for broken references")]
#[command(version = "0.1.0")]
pub struct CliArgs {
    /// Directory to scan (default: current directory)
    #[arg(short, long)]
    pub path: Option<String>,

    /// Enable online checking (HTTP requests)
    #[arg(short, long)]
    pub online: bool,

    /// Output format (text or json)
    #[arg(short, long, default_value = "text")]
    pub format: String,
}
