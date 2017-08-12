#[macro_use]
extern crate log;
extern crate log4rs;
extern crate libc;
extern crate wiringpi;
extern crate home_easy;

use std::env::args;
use std::time::Duration;
use std::thread;

use log::LogLevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Root};

use home_easy::prelude::{DecodedFrame, EncodedFrame};
use home_easy::{scheduler_realtime, scheduler_standard, transmit};

fn main() {
    let stdout = ConsoleAppender::builder().build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LogLevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();

    unsafe {
        if libc::setuid(0) != 0 {
            eprintln!("This program needs root privileges!");
            return
        }
    }

    let args: Vec<String> = args().collect();

    unsafe { scheduler_realtime() };

    let user_input = DecodedFrame {
        pin: match args[1].parse::<u16>() {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Wrong input for pin parameter!");
                eprintln!("{}", e);
                return
            },
        },
        sender: match args[2].parse::<u32>() {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Wrong input for sender parameter!");
                eprintln!("{}", e);
                return
            },
        },
        interruptor: match args[3].parse::<u32>() {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Wrong input for interruptor parameter!");
                eprintln!("{}", e);
                return
            },
        },
        state: match args[4].parse::<String>() {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Wrong input for state parameter!");
                eprintln!("{}", e);
                return
            },
        },
    };

    info!("New DecodedFrame:");
    info!("\tpin: {}", user_input.pin);
    info!("\tsender: {}", user_input.sender);
    info!("\tinterruptor: {}", user_input.interruptor);
    info!("\tstate: {}", user_input.state);

    info!("Wiringpi setup...");
    let pi = wiringpi::setup();
    info!("Output pin setup...");
    let output_pin = pi.output_pin(user_input.pin);

    let output = EncodedFrame::from_decoded(&user_input, &output_pin);

    for _ in 0..5 {
        transmit(&output);
        thread::sleep(Duration::from_millis(10));
    }

    unsafe { scheduler_standard() };
}
