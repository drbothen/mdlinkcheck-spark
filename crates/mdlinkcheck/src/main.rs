use clap::Parser;

/// Command-line interface for mdlinkcheck
#[derive(Parser, Debug)]
#[command(name = "mdlinkcheck")]
#[command(about = "Check markdown links for broken references")]
#[command(version = "0.1.0")]
struct CliArgs {
    /// Directory to scan (default: current directory)
    #[arg(short, long)]
    path: Option<String>,

    /// Enable online checking (HTTP requests)
    #[arg(short, long)]
    online: bool,

    /// Output format (text or json)
    #[arg(short, long, default_value = "text")]
    format: String,
}

fn main() {
    todo!()
}
