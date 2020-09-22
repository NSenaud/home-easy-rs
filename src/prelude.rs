use wiringpi::pin::{OutputPin, WiringPi};

pub struct EncodedFrame<'a> {
    pub pin: &'a OutputPin<WiringPi>,
    pub sender: [bool; 26],
    pub interruptor: [bool; 4],
    pub state: bool,
}

pub struct DecodedFrame {
    pub pin: u16,
    pub sender: u32,
    pub interruptor: u32,
    pub state: String,
}
