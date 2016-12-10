//! This crate provides abstractions over the `energymon_sys` and `energymon_default_sys` crates
//! using the EnergyMonitor trait.

extern crate libc;
extern crate energy_monitor;
extern crate energymon_default_sys;

use libc::{c_char, size_t};
use std::ffi::CStr;
use std::mem;
use energy_monitor::EnergyMonitor;
use energymon_default_sys::{energymon, energymon_get_default};

/// A basic energy monitor.
pub struct EnergyMon {
    /// The native C struct.
    em: energymon,
}

impl EnergyMon {
    /// Create and initialize an `EnergyMon`.
    pub fn new() -> Result<EnergyMon, &'static str> {
        unsafe {
            let mut em: energymon = mem::zeroed();
            match energymon_get_default(&mut em) {
                0 => (),
                _ => return Err("Failed to create energymon")
            }
            // Function pointers should never be NULL from now on.
            // However, if any are NULL, we'll proceed optimistically so long as it's safe
            // (it's not our job to verify the correctness of the native implementation).
            match em.finit {
                Some(f) => {
                    match f(&mut em) {
                        0 => Ok(EnergyMon{ em: em }),
                        _ => Err("Failed to initialize energymon")
                    }
                }
                // shouldn't happen, but we'll be optimistic and hope it just doesn't need initialization...
                None => Ok(EnergyMon{ em: em })
            }
        }
    }

    /// Cleanup the `EnergyMon`.
    fn finish(&mut self) -> i32 {
        match self.em.ffinish {
            Some(f) => f((&mut self.em)),
            None => 0
        }
    }
}

impl Drop for EnergyMon {
    fn drop(&mut self) {
        self.finish();
    }
}

impl EnergyMonitor for EnergyMon {
    fn read_uj(&self) -> Result<u64, &'static str> {
        match self.em.fread {
            Some(f) => Ok(f((&self.em))),
            None => Err("No read function for energymon")
        }
    }

    fn source(&self) -> String {
        match self.em.fsource {
            Some(f) => {
                const BUFSIZE: usize = 100;
                let mut buf: [c_char; BUFSIZE] = [0; BUFSIZE];
                let ret: *mut c_char = f(buf.as_mut_ptr(), mem::size_of_val(&buf) as size_t);
                match ret.is_null() {
                    true => "UNKNOWN".to_owned(),
                    false => unsafe {
                        String::from_utf8_lossy(CStr::from_ptr(buf.as_mut_ptr()).to_bytes()).into_owned()
                    }
                }
            },
            None => "UNKNOWN".to_owned()
        }
    }

    fn interval_us(&self) -> u64 {
        match self.em.finterval {
            Some(f) => f(&self.em),
            None => 0u64
        }
    }

    fn precision_uj(&self) -> u64 {
        match self.em.fprecision {
            Some(f) => f(&self.em),
            None => 0u64
        }
    }

    fn is_exclusive(&self) -> bool {
        match self.em.fexclusive {
            Some(f) => match f() {
                0 => false,
                _ => true
            },
            // optimistically assume that exclusive access isn't required since the native impl didn't specify
            None => false
        }
    }
}

#[cfg(test)]
mod test {
    use super::EnergyMon;
    use energy_monitor::EnergyMonitor;

    #[test]
    fn test_interface() {
        let em: EnergyMon = EnergyMon::new().unwrap();
        println!("Source: {}", em.source());
        println!("Interval: {}", em.interval_us());
        println!("Precision: {}", em.precision_uj());
        println!("Exclusive: {}", em.is_exclusive());
        let val = em.read_uj().unwrap();
        println!("Reading: {}", val);
    }

}
