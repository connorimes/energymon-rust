//! This crate provides abstractions over the `energymon_sys` and `energymon_default_sys` crates
//! using the EnergyMonitor trait.

extern crate libc;
extern crate energy_monitor;
extern crate energymon_sys;
extern crate energymon_default_sys;

use libc::{c_int, c_ulonglong, c_char, size_t};
use std::mem;
use std::ptr;
use energy_monitor::EnergyMonitor;
use energymon_sys::*;
use energymon_default_sys::energymon_get_default;

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
        if ret.is_null() {
            return "UNKNOWN".to_owned();
        }
        let buf: &[u8] = unsafe {
            mem::transmute::<&[c_char], &[u8]>(&buf)
        };
        String::from_utf8_lossy(buf).into_owned()
    }

    fn interval_us(&self) -> u64 {
        (self.em.finterval)(&self.em)
    }
}

impl Default for EnergyMon {
    /// Returns a dummy `EnergyMon`.
    fn default() -> EnergyMon {
        extern fn default_init(_impl: *mut energymon) -> c_int { 0 };
        extern fn default_read_total(_impl: *const energymon) -> c_ulonglong { 0 };
        extern fn default_finish(_impl: *mut energymon) -> c_int { 0 };
        extern fn default_get_source(_impl: *mut c_char, _n: size_t) -> *mut c_char { ptr::null_mut() };
        extern fn default_get_interval(_impl: *const energymon) -> c_ulonglong { 1 };
        EnergyMon {
            em: energymon {
                finit: default_init,
                fread: default_read_total,
                ffinish: default_finish,
                fsource: default_get_source,
                finterval:default_get_interval,
                state: ptr::null_mut(),
            }
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
        let val = em.read_uj().unwrap();
        println!("Read {} from {} with refresh interval {}", val, em.source(), em.interval_us());
    }

    #[test]
    fn test_default() {
        let mut em: EnergyMon = Default::default();
        assert!(em.read_uj().unwrap() == 0);
        assert!(em.interval_us() == 1);
        assert!(em.finish() == 0);
        assert!(em.source().eq("UNKNOWN"));
        assert!(em.em.state.is_null())
    }

}
