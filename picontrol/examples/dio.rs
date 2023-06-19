use std::time::Duration;

use picontrol::{bindings::SPIValue, PiControl};

fn main() {
    let pc = PiControl::new().unwrap();
    let vars = [
        pc.find_variable("O_3"),
        pc.find_variable("O_5"),
        pc.find_variable("O_7"),
        pc.find_variable("O_9"),
    ];

    let mut idx = 0;
    loop {
        for (i, var) in vars.iter().enumerate() {
            let value = if i == idx { 1 } else { 0 };
            let mut val = SPIValue {
                i16uAddress: var.i16uAddress,
                i8uBit: var.i8uBit,
                i8uValue: value,
            };
            pc.set_bit_value(&mut val);
        }
        idx = (idx + 1) % vars.len();
        std::thread::sleep(Duration::from_millis(500));
    }
}
