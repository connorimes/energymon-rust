extern crate libc;
extern crate energymon_sys;
extern crate energymon_default_sys;

use libc::{c_int, c_ulonglong, c_char};
use std::mem;
use std::ptr;
use energymon_sys::*;
use energymon_default_sys::em_impl_get;

/// A basic energy monitor.
pub struct EnergyMon {
    /// The native C struct.
    em: em_impl,
}

impl EnergyMon {
    /// Create and initialize an `EnergyMon`.
    pub fn new() -> Result<EnergyMon, &'static str> {
    	unsafe {
            let mut em: em_impl = mem::uninitialized();
            match em_impl_get(&mut em) {
                0 => (),
                _ => return Err("Failed to create em_impl"),
            }
            match (em.finit)(&mut em) {
            	0 => Ok(EnergyMon{ em: em }),
            	_ => Err("Failed to initialize em_impl"),
        	}
        }
    }

    /// Read the total energy from the `EnergyMon`.
    pub fn read(&self) -> u64 {
        (self.em.fread)(&self.em)
    }

    /// Get a human-readable name of the `EnergyMon`'s source.
    pub fn source(&mut self) -> String {
        const BUFSIZE: usize = 100;
        let mut buf: [c_char; BUFSIZE] = [0; BUFSIZE];
        let ret: *mut c_char = (self.em.fsource)(buf.as_mut_ptr());
        if ret.is_null() {
            return "UNKNOWN".to_owned();
        }
        let buf: &[u8] = unsafe {
            mem::transmute::<&[c_char], &[u8]>(&buf)
        };
        String::from_utf8_lossy(buf).into_owned()
    }

    /// Get the refresh interval for the `EnergyMon`.
    pub fn interval(&self) -> u64 {
        (self.em.finterval)(&self.em)
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

impl Default for EnergyMon {
    /// Returns a dummy `EnergyMon`.
    fn default() -> EnergyMon {
        extern fn default_init(_impl: *mut em_impl) -> c_int { 0 };
        extern fn default_read_total(_impl: *const em_impl) -> c_ulonglong { 0 };
        extern fn default_finish(_impl: *mut em_impl) -> c_int { 0 };
        extern fn default_get_source(_impl: *mut c_char) -> *mut c_char { ptr::null_mut() };
        extern fn default_get_interval(_impl: *const em_impl) -> c_ulonglong { 1 };
        EnergyMon {
            em: em_impl {
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

    #[test]
    fn test_interface() {
        let mut em: EnergyMon = EnergyMon::new().unwrap();
        let val = em.read();
        println!("Read {} from {} with refresh interval {}", val, em.source(), em.interval());
    }

    #[test]
    fn test_default() {
        let mut em: EnergyMon = Default::default();
        assert!(em.read() == 0);
        assert!(em.interval() == 1);
        assert!(em.finish() == 0);
        assert!(em.source().eq("UNKNOWN"));
        assert!(em.em.state.is_null())
    }

}
