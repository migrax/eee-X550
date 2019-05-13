/*
 * Copyright (C) 2019 Miguel Rodríguez Pérez <miguel@det.uvigo.gal>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

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
    if opt.stats {
        let status = dev.get_eee_status();
        println!("EEE Support is: {}", status.get_eee_support());
        println!("EEE TX LPI is: {}", status.get_tx_lpi_status());
        println!("EEE TX LPI count: {}", dev.get_tx_lpi_count());

        eprintln!("Status is: {:?}", status);
    }

    if let Some(hyst) = opt.hyst {
        println!("Hysteresis set to: {}", dev.set_hysteresis(hyst));
    }
}
