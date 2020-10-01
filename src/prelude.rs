use rppal::gpio::OutputPin;

pub struct EncodedFrame<'a> {
    pub pin: &'a mut OutputPin,
    pub sender: [bool; 26],
    pub interruptor: [bool; 4],
    pub state: bool,
}

pub struct DecodedFrame {
    pub pin: u8,
    pub sender: u32,
    pub interruptor: u32,
    pub state: String,
}
