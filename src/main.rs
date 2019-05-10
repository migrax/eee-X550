extern crate structopt;
mod x550;
use structopt::StructOpt;
use x550::DeviceMem;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    /// NIC name
    #[structopt(parse(from_str))]
    device: String,

    /// Enable EEE
    #[structopt(short = "e", long = "enable")]
    enable: bool,

    /// Force Enable EEE
    #[structopt(short = "fe", long = "force-eee")]
    force_enable: bool,

    /// Disable EEE
    #[structopt(short = "d", long = "disable")]
    disable: bool,

    /// Show stats
    #[structopt(short = "s", long = "stats")]
    stats: bool,

    /// Set Tx Entry Delay. Value between 0–63µs
    #[structopt(short = "h", long = "hysteresis")]
    hyst: Option<u8>,
}

fn main() {
    let opt = Opt::from_args();

    let dev = DeviceMem::from_name(&opt.device).expect("Error accessing device");

    println!("Hello, world!");
}
