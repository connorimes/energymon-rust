extern crate libc;
extern crate energymon_sys;

use libc::{c_char};
use std::mem;
use energymon_sys::*;

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

#[cfg(test)]
mod test {
    use super::EnergyMon;

    #[test]
    fn test_interface() {
        let mut em: EnergyMon = EnergyMon::new().unwrap();
        let val = em.read();
        println!("Read {} from {} with refresh interval {}", val, em.source(), em.interval());
    }

}
