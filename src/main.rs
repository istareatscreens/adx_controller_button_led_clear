use serialport::{DataBits, FlowControl, Parity, StopBits};
use std::{io, time::Duration};

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

    let mut button_led_com = match serialport::new("COM21", 115200)
        .data_bits(DataBits::Eight)
        .flow_control(FlowControl::None)
        .stop_bits(StopBits::One)
        .parity(Parity::None)
        .timeout(Duration::from_secs(5))
        .open()
    {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Failed to write to serial port, check for hidden drivers, check device is set to COM21 port, try reinstalling driver. Error message: {}", e);
            println!("Press any key to exit");
            let _ = io::stdin().read_line(&mut String::new());
            panic!("exiting");
        }
    };

    packet_data
        .iter()
        .for_each(|&bytes| match button_led_com.write(bytes) {
            Ok(_) => println!("Successfully wrote packet to serial port."),
            Err(e) => eprintln!("Failed to write to serial port: {}", e),
        });

    std::mem::drop(button_led_com);
}
