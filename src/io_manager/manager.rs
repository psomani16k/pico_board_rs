use embassy_rp::gpio::{Input, Output};
use embassy_time::Timer;
use heapless::Vec;

use {defmt_rtt as _, panic_probe as _};

pub struct KeyboardIoManager<'a> {
    // this is the pin layout used for the keyboard, things are just hardcoded....

    // key layout columns
    pub column_1: Input<'a>,
    pub column_2: Input<'a>,
    pub column_3: Input<'a>,
    pub column_4: Input<'a>,
    pub column_5: Input<'a>,
    pub column_6: Input<'a>,

    // key layout rows
    pub row_1: Output<'a>,
    pub row_2: Output<'a>,
    pub row_3: Output<'a>,

    // right thumb cluster pins
    pub lt_1: Input<'a>,
    pub lt_2: Input<'a>,
    pub lt_3: Input<'a>,

    // caps led
    pub caps_led: Output<'a>,

    // bluetooth trigger key
    pub bluetooth_trigger_key: Input<'a>,

    // mode key to initiate the profile chooser
    pub mode: Input<'a>,

    // display i2c bus
    pub display_bus:
        embassy_rp::i2c::I2c<'a, embassy_rp::peripherals::I2C1, embassy_rp::i2c::Async>,
}

impl<'a> KeyboardIoManager<'a> {
    pub fn new(
        mut column_1: Input<'a>,
        mut column_2: Input<'a>,
        mut column_3: Input<'a>,
        mut column_4: Input<'a>,
        mut column_5: Input<'a>,
        mut column_6: Input<'a>,
        row_1: Output<'a>,
        row_2: Output<'a>,
        row_3: Output<'a>,
        lt_1: Input<'a>,
        lt_2: Input<'a>,
        lt_3: Input<'a>,
        caps_led: Output<'a>,
        bluetooth_trigger_key: Input<'a>,
        mode: Input<'a>,
        display_bus: embassy_rp::i2c::I2c<
            'a,
            embassy_rp::peripherals::I2C1,
            embassy_rp::i2c::Async,
        >,
    ) -> Self {
        column_1.set_schmitt(true);
        column_2.set_schmitt(true);
        column_3.set_schmitt(true);
        column_4.set_schmitt(true);
        column_5.set_schmitt(true);
        column_6.set_schmitt(true);

        return KeyboardIoManager {
            column_1,
            column_2,
            column_3,
            column_4,
            column_5,
            column_6,
            row_1,
            row_2,
            row_3,
            lt_1,
            lt_2,
            lt_3,
            caps_led,
            bluetooth_trigger_key,
            display_bus,
            mode,
        };
    }

    pub async fn generate_readout(&mut self) -> KeyPressReadout {
        KeyPressReadout::new(self).await
    }

    fn set_row_high(&mut self, row: Row) {
        match row {
            Row::Top => self.row_1.set_high(),
            Row::Middle => self.row_2.set_high(),
            Row::Bottom => self.row_3.set_high(),
        };
    }

    fn set_row_low(&mut self, row: Row) {
        match row {
            Row::Top => self.row_1.set_low(),
            Row::Middle => self.row_2.set_low(),
            Row::Bottom => self.row_3.set_low(),
        }
    }

    fn read_column(&self) -> (bool, bool, bool, bool, bool, bool) {
        let col_1_val = self.column_1.is_high();
        let col_2_val = self.column_2.is_high();
        let col_3_val = self.column_3.is_high();
        let col_4_val = self.column_4.is_high();
        let col_5_val = self.column_5.is_high();
        let col_6_val = self.column_6.is_high();
        return (
            col_1_val, col_2_val, col_3_val, col_4_val, col_5_val, col_6_val,
        );
    }

    fn read_thumb_cluster(&self) -> (bool, bool, bool) {
        let t_1 = self.lt_1.is_high();
        let t_2 = self.lt_2.is_high();
        let t_3 = self.lt_3.is_high();
        return (t_1, t_2, t_3);
    }

    fn read_mode_key(&self) -> bool {
        return self.mode.is_high();
    }
}

// +---------------+
// |   C1 - R1     | - 32768
// +---------------+
// |   C2 - R1     | - 16384
// +---------------+
// |   C3 - R1     | - 8192
// +---------------+
// |   C4 - R1     | - 4096
// +---------------+
// |   C5 - R1     | - 2048
// +---------------+
// |   C6 - R1     | - 1024
// +---------------+
// |   C1 - R2     | - 512
// +---------------+
// |   C2 - R2     | - 256
// +---------------+
// |   C3 - R2     | - 128
// +---------------+
// |   C4 - R2     | - 64
// +---------------+
// |   C5 - R2     | - 32
// +---------------+
// |   C6 - R2     | - 16
// +---------------+
// |   C1 - R3     | - 8
// +---------------+
// |   C2 - R3     | - 4
// +---------------+
// |   C3 - R3     | - 2
// +---------------+
// |   C4 - R3     | - 1
// +---------------+
//
// +---------------+
// |   C5 - R3     | - 128
// +---------------+
// |   C6 - R3     | - 64
// +---------------+
// |   LT - 1      | - 32
// +---------------+
// |   LT - 2      | - 16
// +---------------+
// |   LT - 3      | - 8
// +---------------+
// |   MODE        | - 4
// +---------------+
// |   RESERVED    | - 2
// +---------------+
// |   RESERVED    | - 1
// +---------------+
#[derive(Clone, Copy)]
pub struct KeyPressReadout {
    data_1: u16,
    data_2: u8,
}

impl KeyPressReadout {
    async fn new(io_manager: &mut KeyboardIoManager<'_>) -> KeyPressReadout {
        let mut readout = KeyPressReadout {
            data_1: 0,
            data_2: 0,
        };

        // reading the bottom row
        io_manager.set_row_high(Row::Bottom);
        Timer::after_millis(1).await;
        let (c1, c2, c3, c4, c5, c6) = io_manager.read_column();
        if c1 {
            readout.data_1 = readout.data_1 + 32768;
        }
        if c2 {
            readout.data_1 = readout.data_1 + 16384;
        }
        if c3 {
            readout.data_1 = readout.data_1 + 8192;
        }
        if c4 {
            readout.data_1 = readout.data_1 + 4096;
        }
        if c5 {
            readout.data_1 = readout.data_1 + 2048;
        }
        if c6 {
            readout.data_1 = readout.data_1 + 1024;
        }
        io_manager.set_row_low(Row::Bottom);

        // reading the middle row
        io_manager.set_row_high(Row::Middle);
        Timer::after_millis(1).await;
        let (c1, c2, c3, c4, c5, c6) = io_manager.read_column();
        if c1 {
            readout.data_1 = readout.data_1 + 512;
        }
        if c2 {
            readout.data_1 = readout.data_1 + 256;
        }
        if c3 {
            readout.data_1 = readout.data_1 + 128;
        }
        if c4 {
            readout.data_1 = readout.data_1 + 64;
        }
        if c5 {
            readout.data_1 = readout.data_1 + 32;
        }
        if c6 {
            readout.data_1 = readout.data_1 + 16;
        }
        io_manager.set_row_low(Row::Middle);

        // reading the top row
        io_manager.set_row_high(Row::Top);
        Timer::after_millis(1).await;
        let (c1, c2, c3, c4, c5, c6) = io_manager.read_column();
        if c1 {
            readout.data_1 = readout.data_1 + 8;
        }
        if c2 {
            readout.data_1 = readout.data_1 + 4;
        }
        if c3 {
            readout.data_1 = readout.data_1 + 2;
        }
        if c4 {
            readout.data_1 = readout.data_1 + 1;
        }
        if c5 {
            readout.data_2 = readout.data_2 + 128;
        }
        if c6 {
            readout.data_2 = readout.data_2 + 32;
        }
        io_manager.set_row_low(Row::Top);

        // reading the thumb row
        let (lt_1, lt_2, lt_3) = io_manager.read_thumb_cluster();
        if lt_1 {
            readout.data_2 = readout.data_2 + 32;
        }
        if lt_2 {
            readout.data_2 = readout.data_2 + 16;
        }
        if lt_3 {
            readout.data_2 = readout.data_2 + 8;
        }

        // reading the mode key
        if io_manager.read_mode_key() {
            readout.data_2 = readout.data_2 + 4;
        }

        return readout;
    }

    pub fn get_high_keys(&self) -> Vec<KeyLocations, 21> {
        let mut pressed_keys: Vec<KeyLocations, 21> = Vec::new();
        let mut self_copy = self.clone();
        if self_copy.data_1 >= 32768 {
            self_copy.data_1 = self_copy.data_1 - 32786;
            let _ = pressed_keys.push(KeyLocations::C1R1);
        }
        if self_copy.data_1 >= 16384 {
            self_copy.data_1 = self_copy.data_1 - 16384;
            let _ = pressed_keys.push(KeyLocations::C2R1);
        }
        if self_copy.data_1 >= 8192 {
            self_copy.data_1 = self_copy.data_1 - 8192;
            let _ = pressed_keys.push(KeyLocations::C3R1);
        }
        if self_copy.data_1 >= 4096 {
            self_copy.data_1 = self_copy.data_1 - 4096;
            let _ = pressed_keys.push(KeyLocations::C4R1);
        }
        if self_copy.data_1 >= 2048 {
            self_copy.data_1 = self_copy.data_1 - 2048;
            let _ = pressed_keys.push(KeyLocations::C5R1);
        }
        if self_copy.data_1 >= 1024 {
            self_copy.data_1 = self_copy.data_1 - 1024;
            let _ = pressed_keys.push(KeyLocations::C6R1);
        }
        if self_copy.data_1 >= 512 {
            self_copy.data_1 = self_copy.data_1 - 512;
            let _ = pressed_keys.push(KeyLocations::C1R2);
        }
        if self_copy.data_1 >= 256 {
            self_copy.data_1 = self_copy.data_1 - 256;
            let _ = pressed_keys.push(KeyLocations::C2R2);
        }
        if self_copy.data_1 >= 128 {
            self_copy.data_1 = self_copy.data_1 - 128;
            let _ = pressed_keys.push(KeyLocations::C3R2);
        }
        if self_copy.data_1 >= 64 {
            self_copy.data_1 = self_copy.data_1 - 64;
            let _ = pressed_keys.push(KeyLocations::C4R2);
        }
        if self_copy.data_1 >= 32 {
            self_copy.data_1 = self_copy.data_1 - 32;
            let _ = pressed_keys.push(KeyLocations::C5R2);
        }
        if self_copy.data_1 >= 16 {
            self_copy.data_1 = self_copy.data_1 - 16;
            let _ = pressed_keys.push(KeyLocations::C6R2);
        }
        if self_copy.data_1 >= 8 {
            self_copy.data_1 = self_copy.data_1 - 8;
            let _ = pressed_keys.push(KeyLocations::C1R3);
        }
        if self_copy.data_1 >= 4 {
            self_copy.data_1 = self_copy.data_1 - 4;
            let _ = pressed_keys.push(KeyLocations::C2R3);
        }
        if self_copy.data_1 >= 2 {
            self_copy.data_1 = self_copy.data_1 - 2;
            let _ = pressed_keys.push(KeyLocations::C3R3);
        }
        if self_copy.data_1 >= 1 {
            self_copy.data_1 = self_copy.data_1 - 1;
            let _ = pressed_keys.push(KeyLocations::C4R3);
        }
        if self_copy.data_2 >= 128 {
            self_copy.data_2 = self_copy.data_2 - 128;
            let _ = pressed_keys.push(KeyLocations::C5R3);
        }
        if self_copy.data_2 >= 64 {
            self_copy.data_2 = self_copy.data_2 - 64;
            let _ = pressed_keys.push(KeyLocations::C6R3);
        }
        if self_copy.data_2 >= 32 {
            self_copy.data_2 = self_copy.data_2 - 32;
            let _ = pressed_keys.push(KeyLocations::LT1);
        }
        if self_copy.data_2 >= 16 {
            self_copy.data_2 = self_copy.data_2 - 16;
            let _ = pressed_keys.push(KeyLocations::LT2);
        }
        if self_copy.data_2 >= 8 {
            self_copy.data_2 = self_copy.data_2 - 8;
            let _ = pressed_keys.push(KeyLocations::LT3);
        }
        return pressed_keys;
    }

    pub fn is_pressed(&self, location: KeyLocations) -> bool {
        match location {
            KeyLocations::C1R1 => return (self.data_1 & 32768 as u16) != 0,
            KeyLocations::C2R1 => return (self.data_1 & 16384 as u16) != 0,
            KeyLocations::C3R1 => return (self.data_1 & 8192 as u16) != 0,
            KeyLocations::C4R1 => return (self.data_1 & 4096 as u16) != 0,
            KeyLocations::C5R1 => return (self.data_1 & 2048 as u16) != 0,
            KeyLocations::C6R1 => return (self.data_1 & 1024 as u16) != 0,
            KeyLocations::C1R2 => return (self.data_1 & 512 as u16) != 0,
            KeyLocations::C2R2 => return (self.data_1 & 256 as u16) != 0,
            KeyLocations::C3R2 => return (self.data_1 & 128 as u16) != 0,
            KeyLocations::C4R2 => return (self.data_1 & 64 as u16) != 0,
            KeyLocations::C5R2 => return (self.data_1 & 32 as u16) != 0,
            KeyLocations::C6R2 => return (self.data_1 & 16 as u16) != 0,
            KeyLocations::C1R3 => return (self.data_1 & 8 as u16) != 0,
            KeyLocations::C2R3 => return (self.data_1 & 4 as u16) != 0,
            KeyLocations::C3R3 => return (self.data_1 & 2 as u16) != 0,
            KeyLocations::C4R3 => return (self.data_1 & 1 as u16) != 0,
            KeyLocations::C5R3 => return (self.data_2 & 128 as u8) != 0,
            KeyLocations::C6R3 => return (self.data_2 & 64 as u8) != 0,
            KeyLocations::LT1 => return (self.data_2 & 32 as u8) != 0,
            KeyLocations::LT2 => return (self.data_2 & 16 as u8) != 0,
            KeyLocations::LT3 => return (self.data_2 & 8 as u8) != 0,
            KeyLocations::Mode => return (self.data_2 & 4 as u8) != 0,
        }
    }
}

enum Row {
    Top,    // Row 3
    Middle, // Row 2
    Bottom, // Row 1
}
enum KeyLocations {
    C1R1,
    C2R1,
    C3R1,
    C4R1,
    C5R1,
    C6R1,
    C1R2,
    C2R2,
    C3R2,
    C4R2,
    C5R2,
    C6R2,
    C1R3,
    C2R3,
    C3R3,
    C4R3,
    C5R3,
    C6R3,
    LT1,
    LT2,
    LT3,
    Mode,
}
