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
            let mut em: energymon = mem::uninitialized();
            match energymon_get_default(&mut em) {
                0 => (),
                _ => return Err("Failed to create energymon"),
            }
            match (em.finit)(&mut em) {
                0 => Ok(EnergyMon{ em: em }),
                _ => Err("Failed to initialize energymon"),
            }
        }
    }

    /// Cleanup the `EnergyMon`.
    fn finish(&mut self) -> i32 {
        (self.em.ffinish)(&mut self.em)
    }
}

impl Drop for EnergyMon {
    fn drop(&mut self) {
        self.finish();
    }
}

impl EnergyMonitor for EnergyMon {
    fn read_uj(&self) -> Result<u64, &'static str> {
        Ok((self.em.fread)(&self.em))
    }

    fn source(&self) -> String {
        const BUFSIZE: usize = 100;
        let mut buf: [c_char; BUFSIZE] = [0; BUFSIZE];
        let ret: *mut c_char = (self.em.fsource)(buf.as_mut_ptr(),
                                                 mem::size_of_val(&buf) as size_t);
        match ret.is_null() {
            true => "UNKNOWN".to_owned(),
            false => unsafe {
                String::from_utf8_lossy(CStr::from_ptr(buf.as_mut_ptr()).to_bytes()).into_owned()
            }
        }
    }

    fn interval_us(&self) -> u64 {
        (self.em.finterval)(&self.em)
    }
}

#[cfg(test)]
mod test {
    use super::EnergyMon;
    use energy_monitor::EnergyMonitor;

    #[test]
    fn test_interface() {
        let em: EnergyMon = EnergyMon::new().unwrap();
        let val = em.read_uj().unwrap();
        println!("Read {} from {} with refresh interval {}", val, em.source(), em.interval_us());
    }

}
