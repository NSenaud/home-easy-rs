#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate libc;
extern crate wiringpi;
extern crate home_easy;

use std::time::Duration;
use std::thread;

use log::LogLevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Root};

use home_easy::prelude::{DecodedFrame, EncodedFrame};
use home_easy::{scheduler_realtime, transmit};

fn main() {
    // Logger output will be stdout
    let stdout = ConsoleAppender::builder().build();

    /* Argument parsing initialization */
    let matches = clap_app!(myapp =>
        (name: "dios")
        (version: "0.1.0")
        (author: "Nicolas Senaud <nicolas@senaud.fr>")
        (about: "Switch on and off wireless plugs using GPIO and a 433MHz emetter")
        (@arg PIN: +required "GPIO pin to use")
        (@arg SENDER: +required "Sender code, which authentify the emetter to the plug")
        (@arg INTERRUPTOR: +required "Interruptor number")
        (@arg STATE: +required "On or off")
        (@arg verbose:  -v --verbose ... "Sets the level of verbosity")
    ).get_matches();

    // Define the level of verbosity and configure the logger accordingly
    let config = match matches.occurrences_of("verbose") {
        0 => Config::builder()
                .appender(Appender::builder().build("stdout", Box::new(stdout)))
                .build(Root::builder().appender("stdout").build(LogLevelFilter::Warn))
                .unwrap(),
        1 => Config::builder()
                .appender(Appender::builder().build("stdout", Box::new(stdout)))
                .build(Root::builder().appender("stdout").build(LogLevelFilter::Info))
                .unwrap(),
        2 | _ => Config::builder()
                .appender(Appender::builder().build("stdout", Box::new(stdout)))
                .build(Root::builder().appender("stdout").build(LogLevelFilter::Debug))
                .unwrap(),
    };

    // Initilalize the logger.
    log4rs::init_config(config).unwrap();

    unsafe {
        if libc::setuid(0) != 0 {
            eprintln!("This program needs root privileges!");
            return
        }
    }

    unsafe { scheduler_realtime() };

    let user_input = DecodedFrame {
        pin: match matches.value_of("PIN").unwrap().parse::<u16>() {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Wrong input for pin parameter!");
                eprintln!("{}", e);
                return
            },
        },
        sender: match matches.value_of("SENDER").unwrap().parse::<u32>() {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Wrong input for sender parameter!");
                eprintln!("{}", e);
                return
            },
        },
        interruptor: match matches.value_of("INTERRUPTOR").unwrap().parse::<u32>() {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Wrong input for interruptor parameter!");
                eprintln!("{}", e);
                return
            },
        },
        state: match matches.value_of("STATE").unwrap().parse::<String>() {
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
}
