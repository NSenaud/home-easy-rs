struct EncodedFrame {
    pin: u8,
    sender: [bool; 26],
    interruptor: [bool; 4],
}

struct DecodedFrame {
    pin: u8,
    sender: u32,
    interruptor: u8,
}

enum State {
    On,
    Off
}
