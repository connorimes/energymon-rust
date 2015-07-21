use std::mem;
use libc::{c_char};
use std::sync::{Arc, Once, ONCE_INIT};
use std::cell::Cell;
use energymon_sys::*;

#[derive(Clone)]
pub struct SingletonEnergyMon {
    em: Arc<Cell<em_impl>>,
}

impl SingletonEnergyMon {
    pub fn instance() -> Option<SingletonEnergyMon> {
        static mut EM: *const SingletonEnergyMon = 0 as *const SingletonEnergyMon;
        static ONCE: Once = ONCE_INIT;

        unsafe {
            ONCE.call_once(|| {
                println!("Initializing singleton energy monitor");
                let mut em: em_impl = mem::uninitialized();
                match em_impl_get(&mut em) {
                    0 => (),
                    _ => {
                        println!("Failed to get singleton energy monitor");
                        return;
                    }
                }
                match (em.finit)(&mut em) {
                    0 => (),
                    _ => {
                        println!("Failed to initialize singleton energy monitor");
                        return;
                    }
                }
                let sem: SingletonEnergyMon = SingletonEnergyMon {
                    em: Arc::new(Cell::new(em))
                };
                // put it on the heap
                EM = mem::transmute(Box::new(sem));
            });
            if EM.is_null() {
                None
            } else {
                Some((*EM).clone())
            }
        }
    }

    pub fn read(&self) -> u64 {
        (self.em.get().fread)(&self.em.get())
    }

    pub fn source(&self) -> String {
        const BUFSIZE: usize = 100;
        let mut buf: [c_char; BUFSIZE] = [0; BUFSIZE];
        let ret: *mut c_char = (self.em.get().fsource)(buf.as_mut_ptr());
        if ret.is_null() {
            return "UNKNOWN".to_owned();
        }
        let buf: &[u8] = unsafe {
            mem::transmute::<&[c_char], &[u8]>(&buf)
        };
        String::from_utf8_lossy(buf).into_owned()
    }

    pub fn interval(&self) -> u64 {
        (self.em.get().finterval)(&self.em.get())
    }

    pub unsafe fn destroy(&self) {
        println!("Finishing singleton energy monitor");
        (self.em.get().ffinish)(&mut self.em.get());
    }
}

#[cfg(test)]
mod test {
    use super::SingletonEnergyMon;

    #[test]
    fn test_singleton() {
        for _ in 0..2 {
            let sem: SingletonEnergyMon = SingletonEnergyMon::instance().unwrap();
            let val = sem.read();
            println!("Read {} from {} singleton with refresh interval {}", val, sem.source(),
                     sem.interval());
        }
        unsafe {
            SingletonEnergyMon::instance().unwrap().destroy();
        }
    }

}
