#[macro_use]
extern crate libc;
extern crate log;
extern crate log4rs;
extern crate rppal;

mod prelude;

use std::mem;

use libc::{sched_get_priority_max, sched_setscheduler};

use prelude::*;

pub unsafe fn scheduler_realtime() {
    let mut p: libc::sched_param;
    p.sched_priority = sched_get_priority_max(libc::SCHED_RR);

    // Convince the Rust compiler that p is initialized.
    p = mem::uninitialized();
    if sched_setscheduler(0, libc::SCHED_RR, &p) == -1 {
        panic!("Failed to switch to realtime scheduler.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn it_works() {
        unsafe { scheduler_realtime() };
    }
}
