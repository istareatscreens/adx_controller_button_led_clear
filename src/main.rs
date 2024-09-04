use serialport::{DataBits, FlowControl, Parity, StopBits};
use std::time::Duration;

fn main() {
    let packet_data: [&[u8]; 6] = [
        &[0xE0, 0x11, 0x01, 0x01, 0x10, 0x23],
        &[0xE0, 0x11, 0x01, 0x01, 0x10, 0x23],
        &[0xE0, 0x11, 0x01, 0x01, 0x10, 0x23],
        &[
            0xE0, 0x11, 0x01, 0x08, 0x32, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6C,
        ],
        &[0xE0, 0x11, 0x01, 0x04, 0x39, 0x00, 0x00, 0x00, 0x4F],
        &[0xE0, 0x11, 0x01, 0x01, 0x3C, 0x4F],
    ];

    let mut button_led_com = serialport::new("COM21", 115200)
        .data_bits(DataBits::Eight)
        .flow_control(FlowControl::None)
        .stop_bits(StopBits::One)
        .parity(Parity::None)
        .timeout(Duration::from_secs(100))
        .open()
        .expect("Failed to open port");

    packet_data
        .iter()
        .for_each(|&bytes| match button_led_com.write(bytes) {
            Ok(_) => println!("Successfully wrote packet to serial port."),
            Err(e) => eprintln!("Failed to write to serial port: {}", e),
        });

    std::mem::drop(button_led_com);
}
