use x86_64::instructions::port::Port;

use crate::println_log;

use super::types::*;

pub fn probe_port(abar: &HbaMem) {
    let pi = abar.pi;

    for i in 0..32 {
        if pi & 1 != 0 {
            match check_type(&abar.ports[i]) {
                PortSignature::Sata => {
                    println_log!("Found SATA port at {}", i);
                },
                PortSignature::Semb => {
                    println_log!("Found SEMB port at {}", i);
                },
                PortSignature::Pm => {
                    println_log!("Found PM port at {}", i);
                },
                PortSignature::Satapi => {
                    println_log!("Found SATAPI port at {}", i);
                },
                _ => {
                    println_log!("Unknown port type");
                }
            }
        }
    }
}

pub fn check_type(port: &HbaPort) -> PortSignature {
    let ssts = port.ssts;

    let ipm: u8 = ((ssts >> 8) & 0x0F) as u8;
    let det: u8 = (ssts & 0x01) as u8;

    if det != HBA_PORT_DET_PRESENT {
        return PortSignature::Null;
    }

    if ipm != HBA_PORT_IPM_ACTIVE {
        return PortSignature::Null;
    }

    return PortSignature::from(port.sig);
}

pub fn start_cmd(port: &mut HbaPort) {
    while port.cmd & HBA_PXCMD_CR != 0 {};

    port.cmd |= HBA_PXCMD_FRE;
    port.cmd |= HBA_PXCMD_ST;
}

pub fn stop_cmd(port: &mut HbaPort) {
    port.cmd &= !HBA_PXCMD_ST;
}