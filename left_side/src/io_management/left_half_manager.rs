use embassy_rp::{
    gpio::{Input, Output, Pin},
    Peripheral,
};
use embassy_time::Timer;

pub struct LeftIoManager<'a> {
    row_1: Output<'a>,
    row_2: Output<'a>,
    row_3: Output<'a>,
    column_1: Input<'a>,
    column_2: Input<'a>,
    column_3: Input<'a>,
    column_4: Input<'a>,
    column_5: Input<'a>,
    column_6: Input<'a>,
    lt_1: Input<'a>,
    lt_2: Input<'a>,
    lt_3: Input<'a>,
}

impl<'d> LeftIoManager<'d> {
    pub fn new(
        row_1: impl Peripheral<P = impl Pin> + 'd,
        row_2: impl Peripheral<P = impl Pin> + 'd,
        row_3: impl Peripheral<P = impl Pin> + 'd,
        column_1: impl Peripheral<P = impl Pin> + 'd,
        column_2: impl Peripheral<P = impl Pin> + 'd,
        column_3: impl Peripheral<P = impl Pin> + 'd,
        column_4: impl Peripheral<P = impl Pin> + 'd,
        column_5: impl Peripheral<P = impl Pin> + 'd,
        column_6: impl Peripheral<P = impl Pin> + 'd,
        lt_1: impl Peripheral<P = impl Pin> + 'd,
        lt_2: impl Peripheral<P = impl Pin> + 'd,
        lt_3: impl Peripheral<P = impl Pin> + 'd,
    ) -> LeftIoManager<'d> {
        let row_1 = Output::new(row_1, embassy_rp::gpio::Level::High);
        let row_2 = Output::new(row_2, embassy_rp::gpio::Level::High);
        let row_3 = Output::new(row_3, embassy_rp::gpio::Level::High);

        // initiating the columns
        let mut column_1 = Input::new(column_1, embassy_rp::gpio::Pull::Up);
        column_1.set_schmitt(true);
        let mut column_2 = Input::new(column_2, embassy_rp::gpio::Pull::Up);
        column_2.set_schmitt(true);
        let mut column_3 = Input::new(column_3, embassy_rp::gpio::Pull::Up);
        column_3.set_schmitt(true);
        let mut column_4 = Input::new(column_4, embassy_rp::gpio::Pull::Up);
        column_4.set_schmitt(true);
        let mut column_5 = Input::new(column_5, embassy_rp::gpio::Pull::Up);
        column_5.set_schmitt(true);
        let mut column_6 = Input::new(column_6, embassy_rp::gpio::Pull::Up);
        column_6.set_schmitt(true);

        // left thumb cluster
        let mut lt_1 = Input::new(lt_1, embassy_rp::gpio::Pull::Up);
        lt_1.set_schmitt(true);
        let mut lt_2 = Input::new(lt_2, embassy_rp::gpio::Pull::Up);
        lt_2.set_schmitt(true);
        let mut lt_3 = Input::new(lt_3, embassy_rp::gpio::Pull::Up);
        lt_3.set_schmitt(true);

        return LeftIoManager {
            row_1,
            row_2,
            row_3,
            column_1,
            column_2,
            column_3,
            column_4,
            column_5,
            column_6,
            lt_1,
            lt_2,
            lt_3,
        };
    }

    pub async fn produce_readout(&mut self) -> LeftReadout {
        // readout
        let mut row_1_readout: u8 = 0;
        self.row_1.set_low();
        Timer::after_micros(100).await;
        row_1_readout = self.column_1.is_low() as u8 * 0b00100000 + row_1_readout;
        row_1_readout = self.column_2.is_low() as u8 * 0b00010000 + row_1_readout;
        row_1_readout = self.column_3.is_low() as u8 * 0b00001000 + row_1_readout;
        row_1_readout = self.column_4.is_low() as u8 * 0b00000100 + row_1_readout;
        row_1_readout = self.column_5.is_low() as u8 * 0b00000010 + row_1_readout;
        row_1_readout = self.column_6.is_low() as u8 * 0b00000001 + row_1_readout;
        self.row_1.set_high();
        Timer::after_micros(100).await;

        let mut row_2_readout: u8 = 0;
        self.row_2.set_low();
        Timer::after_micros(100).await;
        row_2_readout = self.column_1.is_low() as u8 * 0b00100000 + row_2_readout;
        row_2_readout = self.column_2.is_low() as u8 * 0b00010000 + row_2_readout;
        row_2_readout = self.column_3.is_low() as u8 * 0b00001000 + row_2_readout;
        row_2_readout = self.column_4.is_low() as u8 * 0b00000100 + row_2_readout;
        row_2_readout = self.column_5.is_low() as u8 * 0b00000010 + row_2_readout;
        row_2_readout = self.column_6.is_low() as u8 * 0b00000001 + row_2_readout;
        self.row_2.set_high();
        Timer::after_micros(100).await;

        let mut row_3_readout: u8 = 0;
        self.row_3.set_low();
        Timer::after_micros(100).await;
        row_3_readout = self.column_1.is_low() as u8 * 0b00100000 + row_3_readout;
        row_3_readout = self.column_2.is_low() as u8 * 0b00010000 + row_3_readout;
        row_3_readout = self.column_3.is_low() as u8 * 0b00001000 + row_3_readout;
        row_3_readout = self.column_4.is_low() as u8 * 0b00000100 + row_3_readout;
        row_3_readout = self.column_5.is_low() as u8 * 0b00000010 + row_3_readout;
        row_3_readout = self.column_6.is_low() as u8 * 0b00000001 + row_3_readout;
        self.row_3.set_high();
        Timer::after_micros(100).await;

        let mut left_thumb_cluster_readout: u8 = 0;
        left_thumb_cluster_readout =
            self.lt_1.is_low() as u8 * 0b00000001 + left_thumb_cluster_readout;
        left_thumb_cluster_readout =
            self.lt_2.is_low() as u8 * 0b00000010 + left_thumb_cluster_readout;
        left_thumb_cluster_readout =
            self.lt_3.is_low() as u8 * 0b00000100 + left_thumb_cluster_readout;

        return LeftReadout {
            ltc: left_thumb_cluster_readout,
            row_1: row_1_readout,
            row_2: row_2_readout,
            row_3: row_3_readout,
        };
    }
}

#[derive(PartialEq)]
pub struct LeftReadout {
    row_1: u8,
    row_2: u8,
    row_3: u8,
    ltc: u8,
}
impl Default for LeftReadout {
    fn default() -> Self {
        return LeftReadout {
            ltc: 0,
            row_1: 0,
            row_2: 0,
            row_3: 0,
        };
    }
}

impl LeftReadout {
    pub fn is_pressed(&self, location: &LeftKeyLocation) -> bool {
        match location {
            LeftKeyLocation::C1R1 => return self.row_1 & 0b00100000 != 0,
            LeftKeyLocation::C2R1 => return self.row_1 & 0b00010000 != 0,
            LeftKeyLocation::C3R1 => return self.row_1 & 0b00001000 != 0,
            LeftKeyLocation::C4R1 => return self.row_1 & 0b00000100 != 0,
            LeftKeyLocation::C5R1 => return self.row_1 & 0b00000010 != 0,
            LeftKeyLocation::C6R1 => return self.row_1 & 0b00000001 != 0,
            LeftKeyLocation::C1R2 => return self.row_2 & 0b00100000 != 0,
            LeftKeyLocation::C2R2 => return self.row_2 & 0b00010000 != 0,
            LeftKeyLocation::C3R2 => return self.row_2 & 0b00001000 != 0,
            LeftKeyLocation::C4R2 => return self.row_2 & 0b00000100 != 0,
            LeftKeyLocation::C5R2 => return self.row_2 & 0b00000010 != 0,
            LeftKeyLocation::C6R2 => return self.row_2 & 0b00000001 != 0,
            LeftKeyLocation::C1R3 => return self.row_3 & 0b00100000 != 0,
            LeftKeyLocation::C2R3 => return self.row_3 & 0b00010000 != 0,
            LeftKeyLocation::C3R3 => return self.row_3 & 0b00001000 != 0,
            LeftKeyLocation::C4R3 => return self.row_3 & 0b00000100 != 0,
            LeftKeyLocation::C5R3 => return self.row_3 & 0b00000010 != 0,
            LeftKeyLocation::C6R3 => return self.row_3 & 0b00000001 != 0,
            LeftKeyLocation::LT1 => return self.ltc & 0b00000001 != 0,
            LeftKeyLocation::LT2 => return self.ltc & 0b00000010 != 0,
            LeftKeyLocation::LT3 => return self.ltc & 0b00000100 != 0,
        }
    }
}

pub enum LeftKeyLocation {
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
}
