#[macro_use]
extern crate log;
extern crate libc;
extern crate log4rs;
extern crate rppal;
extern crate wiringpi;

pub mod prelude;

use std::mem::MaybeUninit;
use std::{thread, time};

use libc::{sched_get_priority_max, sched_setscheduler};
use wiringpi::pin::Value::{High, Low};
use wiringpi::pin::{OutputPin, WiringPi};

use prelude::*;

pub unsafe fn scheduler_realtime() {
    info!("Switching to realtime scheduler...");

    let mut p: libc::sched_param;
    // Convince the Rust compiler that p is initialized.
    p = MaybeUninit::uninit().assume_init();

    p.sched_priority = sched_get_priority_max(libc::SCHED_RR);
    if sched_setscheduler(0, libc::SCHED_RR, &p) == -1 {
        // TODO: Add errno readout.
        warn!("Failed to switch to realtime scheduler.");
        eprintln!("Failed to switch to realtime scheduler.");
    }

    info!("Switched to realtime scheduler.");
}

pub unsafe fn scheduler_standard() {
    info!("Switching to standard scheduler...");

    let mut p: libc::sched_param;
    // Convince the Rust compiler that p is initialized.
    p = MaybeUninit::uninit().assume_init();

    p.sched_priority = sched_get_priority_max(libc::SCHED_OTHER);
    if sched_setscheduler(0, libc::SCHED_OTHER, &p) == -1 {
        // TODO: Add errno readout.
        warn!("Failed to switch to standard scheduler.");
        eprintln!("Failed to switch to standard scheduler.");
    }

    info!("Switched to standard scheduler.");
}

fn send_bit(pin: &OutputPin<WiringPi>, bit: bool) {
    debug!("Sending bit: {}", bit);

    let small_delay = time::Duration::new(0, 275000);
    let large_delay = time::Duration::new(0, 1225000);

    if bit {
        pin.digital_write(High);
        thread::sleep(small_delay);
        pin.digital_write(Low);
        thread::sleep(large_delay);
    } else {
        pin.digital_write(High);
        thread::sleep(small_delay);
        pin.digital_write(Low);
        thread::sleep(small_delay);
    }
}

fn power2(power: usize) -> u32 {
    let mut integer: u32 = 1;
    for _ in 0..power {
        integer *= 2;
    }

    debug!("power2({}) -> {}", power, integer);

    integer
}

impl<'a> EncodedFrame<'a> {
    pub fn from_decoded<'b>(
        decoded: &DecodedFrame,
        pin: &'b OutputPin<WiringPi>,
    ) -> EncodedFrame<'b> {
        let mut encoded = EncodedFrame {
            pin: pin,
            sender: [false; 26],
            interruptor: [false; 4],
            state: match decoded.state.as_str() {
                "On" | "on" => true,
                "Off" | "off" => false,
                _ => panic!("Invalid input"),
            },
        };

        encoded.itob(decoded.sender);
        encoded.itob_interruptor(decoded.interruptor);

        info!("New EncodedFrame from DecodedFrame:");
        info!("\tpin: opened from {}", decoded.pin);
        info!("\tsender: {:?}", encoded.sender);
        info!("\tinterruptor: {:?}", encoded.interruptor);
        info!("\tstate: {}", encoded.state);

        encoded
    }

    fn itob(&mut self, mut integer: u32) {
        for i in 0..self.sender.len() {
            if (integer / power2(self.sender.len() - 1 - i)) == 1 {
                integer -= power2(self.sender.len() - 1 - i);
                self.sender[i] = true;
            } else {
                self.sender[i] = false;
            }
        }
    }

    fn itob_interruptor(&mut self, mut integer: u32) {
        for i in 0..self.interruptor.len() {
            if (integer / power2(self.interruptor.len() - 1 - i)) == 1 {
                integer -= power2(self.interruptor.len() - 1 - i);
                self.interruptor[i] = true;
            } else {
                self.interruptor[i] = false;
            }
        }
    }
}

fn send_pair(pin: &OutputPin<WiringPi>, bit: bool) {
    debug!("Send pair for bit: {}", bit);

    if bit {
        send_bit(pin, true);
        send_bit(pin, false);
    } else {
        send_bit(pin, false);
        send_bit(pin, true);
    }
}

pub fn transmit(frame: &EncodedFrame) {
    let wait_delay = time::Duration::new(0, 275000);
    let second_lock_delay = time::Duration::new(0, 2675000);
    let first_lock_delay = time::Duration::new(0, 9900000);

    frame.pin.digital_write(High);
    thread::sleep(wait_delay);
    frame.pin.digital_write(Low);
    thread::sleep(first_lock_delay);
    frame.pin.digital_write(High);
    thread::sleep(wait_delay);
    frame.pin.digital_write(Low);
    thread::sleep(second_lock_delay);
    frame.pin.digital_write(High);

    // Code from emitor (emitor ID)
    for b in frame.sender.iter() {
        send_pair(frame.pin, *b);
    }

    // 26th bit (grouped command)
    send_pair(frame.pin, false);

    // 27th bit (On or Off)
    send_pair(frame.pin, frame.state);

    // 4 last bits
    for b in frame.interruptor.iter() {
        send_pair(frame.pin, *b);
    }

    frame.pin.digital_write(High);
    thread::sleep(wait_delay);
    frame.pin.digital_write(Low);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn it_cannot_switch_to_rt_scheduler() {
        unsafe { scheduler_realtime() };
    }

    #[test]
    #[should_panic]
    fn it_cannot_switch_to_standard_scheduler() {
        unsafe { scheduler_standard() };
    }
}
