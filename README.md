# eee-X550

[![Build Status](https://travis-ci.org/migrax/eee-X550.svg?branch=master)](https://travis-ci.org/migrax/eee-X550)

A small utility to setup and query EEE capabilities on Intel X550 NICs

## Overview

This utility complements the ixgbe kernel driver and permits consulting and 
modifying EEE related parameters os X550 Intel NICs.

## Usage
    eee_x550 [FLAGS] [OPTIONS] <device>

### FLAGS:
    -d, --disable      Disable EEE
    -e, --enable       Enable EEE
    -f, --force-eee    Force Enable EEE
        --help         Prints help information
    -s, --stats        Show stats
    -V, --version      Prints version information

### OPTIONS:
    -h, --hysteresis <hyst>    Set Tx Entry Delay. Value between 0–63µs

### ARGS:
    <device>    NIC name

## Legal

Copyright ⓒ 2019 Miguel Rodríguez Pérez <miguel@det.uvigo.gal>.

This simulator is licensed under the GNU General Public License, version 3 (GPL-3.0). For information see LICENSE
