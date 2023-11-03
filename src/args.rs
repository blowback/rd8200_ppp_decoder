use clap::Parser;
use std::sync::OnceLock;

#[derive(Debug, Parser)]
/// Decode and display PPP format packets from RadioDetection's RD8x000 series locators.
#[command(author, version)]
pub struct Cli {
    /// Paths of binary RD8x00 PPP files to process
    pub paths: Vec<std::path::PathBuf>,

    /// Enable debugging, specify 2 or 3 times to get more output
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    /// Treat "additional data" as a big-endian u32
    #[arg(long)]
    pub big_endian: bool,

    /// Treat "additional data" as having most-significant bit at bit position 0
    #[arg(long)]
    pub msb0: bool,

    /// Suppress the output of the "RD" data section
    #[arg(long)]
    pub no_rd: bool,

    /// Suppress the output of the "New Locator Data" section
    #[arg(long)]
    pub no_loc: bool,

    /// Suppress the output of the "MRX" section
    #[arg(long)]
    pub no_mrx: bool,

    /// Suppress the output of the "RTC" section
    #[arg(long)]
    pub no_rtc: bool,

    /// Suppress the output of the "GPS" section
    #[arg(long)]
    pub no_gps: bool,
}

static ARGS: OnceLock<Cli> = OnceLock::new();

impl Cli {
    pub fn get() -> &'static Cli {
        ARGS.get_or_init(|| Cli::parse())
    }
}
