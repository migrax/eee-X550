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

use std::fs::OpenOptions;
use std::os::unix::io::IntoRawFd;
use std::{mem, path, ptr};

const PAGE_SIZE: usize = 1 << 12;
const PAGE_MASK: usize = PAGE_SIZE - 1;
const MAP_OFFSET: i64 = 0x4000;
const EEE_SU: usize = 0x4380 & PAGE_MASK;
const EEE_STAT: usize = 0x4398 & PAGE_MASK;
const EEER: usize = 0x43A0 & PAGE_MASK;

#[derive(Debug)]
pub struct DeviceMem {
    memmap: *mut libc::c_void,
}

impl Drop for DeviceMem {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.memmap, 4096);
        }
    }
}

impl DeviceMem {
    pub fn from_name(name: &str) -> Result<DeviceMem, String> {
        let mem = if let Ok(resource_file) = OpenOptions::new()
            .write(true)
            .read(true)
            .open(get_source_file(name)?)
        {
            let fd = resource_file.into_raw_fd();
            let memmap = unsafe {
                libc::mmap(
                    ptr::null_mut(),
                    PAGE_SIZE,
                    libc::PROT_READ | libc::PROT_WRITE,
                    libc::MAP_SHARED,
                    fd,
                    MAP_OFFSET,
                )
            };
            if memmap == libc::MAP_FAILED {
                return Err("Could not map memory region".to_string());
            }

            memmap
        } else {
            return Err(format!("Could not open {}", name));
        };

        Ok(DeviceMem { memmap: mem })
    }

    pub fn get_eee_status(&self) -> EeeStatus {
        let val: u32 = read_register(self.memmap, EEE_STAT);

        EeeStatus::from_raw(val)
    }
}

fn read_register<S, T>(orig: *const S, offset: usize) -> T {
    let ptr = orig as *const T;
    assert!(offset < PAGE_SIZE, "offset is out of bounds");

    unsafe { ptr::read_volatile(ptr.add(offset / mem::size_of::<T>())) }
}

fn write_register<S, T>(orig: *mut S, offset: usize, value: T) {
    let ptr = orig as *mut T;
    assert!(offset < PAGE_SIZE, "offset is out of bounds");

    unsafe { ptr::write_volatile(ptr.add(offset / mem::size_of::<T>()), value) }
}

fn get_source_file(name: &str) -> Result<path::PathBuf, String> {
    let sys_name = format!("/sys/class/net/{}", name);
    let mut net_dev = path::PathBuf::from(&sys_name);

    if net_dev.is_dir() {
        net_dev.push("device");
        net_dev.push("resource0");
        Ok(net_dev)
    } else {
        Err(format!("Could not find NIC named {}", name))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct EeeStatus {
    raw_value: u32,
}

impl EeeStatus {
    fn from_raw(value: u32) -> EeeStatus {
        EeeStatus { raw_value: value }
    }

    pub fn get_eee_support(self) -> bool {
        self.raw_value & 0x2000_0000 != 0
    }

    pub fn get_tx_lpi_status(self) -> bool {
        self.raw_value & 0x8000_0000 != 0
    }
}
