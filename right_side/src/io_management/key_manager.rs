use embassy_rp::{
    gpio::{Input, Output, Pin},
    Peripheral,
};
use embassy_time::Timer;

pub struct RightIoKeyManager<'a> {
    row_1: Output<'a>,
    row_2: Output<'a>,
    row_3: Output<'a>,

    column_7: Input<'a>,
    column_8: Input<'a>,
    column_9: Input<'a>,
    column_10: Input<'a>,
    column_11: Input<'a>,
    column_12: Input<'a>,

    rt_1: Input<'a>,
    rt_2: Input<'a>,
    rt_3: Input<'a>,
}

impl<'d> RightIoKeyManager<'d> {
    pub fn new(
        row_1: impl Peripheral<P = impl Pin> + 'd,
        row_2: impl Peripheral<P = impl Pin> + 'd,
        row_3: impl Peripheral<P = impl Pin> + 'd,
        column_7: impl Peripheral<P = impl Pin> + 'd,
        column_8: impl Peripheral<P = impl Pin> + 'd,
        column_9: impl Peripheral<P = impl Pin> + 'd,
        column_10: impl Peripheral<P = impl Pin> + 'd,
        column_11: impl Peripheral<P = impl Pin> + 'd,
        column_12: impl Peripheral<P = impl Pin> + 'd,
        rt_1: impl Peripheral<P = impl Pin> + 'd,
        rt_2: impl Peripheral<P = impl Pin> + 'd,
        rt_3: impl Peripheral<P = impl Pin> + 'd,
    ) -> Self {
        let row_1 = Output::new(row_1, embassy_rp::gpio::Level::High);
        let row_2 = Output::new(row_2, embassy_rp::gpio::Level::High);
        let row_3 = Output::new(row_3, embassy_rp::gpio::Level::High);

        // initiating the columns
        let mut column_7 = Input::new(column_7, embassy_rp::gpio::Pull::Up);
        column_7.set_schmitt(true);
        let mut column_8 = Input::new(column_8, embassy_rp::gpio::Pull::Up);
        column_8.set_schmitt(true);
        let mut column_9 = Input::new(column_9, embassy_rp::gpio::Pull::Up);
        column_9.set_schmitt(true);
        let mut column_10 = Input::new(column_10, embassy_rp::gpio::Pull::Up);
        column_10.set_schmitt(true);
        let mut column_11 = Input::new(column_11, embassy_rp::gpio::Pull::Up);
        column_11.set_schmitt(true);
        let mut column_12 = Input::new(column_12, embassy_rp::gpio::Pull::Up);
        column_12.set_schmitt(true);

        // right thumb cluster
        let mut rt_1 = Input::new(rt_1, embassy_rp::gpio::Pull::Up);
        rt_1.set_schmitt(true);
        let mut rt_2 = Input::new(rt_2, embassy_rp::gpio::Pull::Up);
        rt_2.set_schmitt(true);
        let mut rt_3 = Input::new(rt_3, embassy_rp::gpio::Pull::Up);
        rt_3.set_schmitt(true);

        return RightIoKeyManager {
            row_1,
            row_2,
            row_3,
            column_7,
            column_8,
            column_9,
            column_10,
            column_11,
            column_12,
            rt_1,
            rt_2,
            rt_3,
        };
    }

    pub async fn produce_readout(&mut self) -> RightKeyReadout {
        let mut row_1_readout: u8 = 0;
        self.row_1.set_low();
        Timer::after_micros(100).await;
        row_1_readout = self.column_7.is_low() as u8 * 0b10000000 + row_1_readout;
        row_1_readout = self.column_8.is_low() as u8 * 0b01000000 + row_1_readout;
        row_1_readout = self.column_9.is_low() as u8 * 0b00100000 + row_1_readout;
        row_1_readout = self.column_10.is_low() as u8 * 0b00010000 + row_1_readout;
        row_1_readout = self.column_11.is_low() as u8 * 0b00001000 + row_1_readout;
        row_1_readout = self.column_12.is_low() as u8 * 0b00000100 + row_1_readout;
        self.row_1.set_high();
        Timer::after_micros(100).await;

        let mut row_2_readout: u8 = 0;
        self.row_2.set_low();
        Timer::after_micros(100).await;
        row_2_readout = self.column_7.is_low() as u8 * 0b10000000 + row_2_readout;
        row_2_readout = self.column_8.is_low() as u8 * 0b01000000 + row_2_readout;
        row_2_readout = self.column_9.is_low() as u8 * 0b00100000 + row_2_readout;
        row_2_readout = self.column_10.is_low() as u8 * 0b00010000 + row_2_readout;
        row_2_readout = self.column_11.is_low() as u8 * 0b00001000 + row_2_readout;
        row_2_readout = self.column_12.is_low() as u8 * 0b00000100 + row_2_readout;
        self.row_2.set_high();
        Timer::after_micros(100).await;

        let mut row_3_readout: u8 = 0;
        self.row_3.set_low();
        Timer::after_micros(100).await;
        row_3_readout = self.column_7.is_low() as u8 * 0b10000000 + row_3_readout;
        row_3_readout = self.column_8.is_low() as u8 * 0b01000000 + row_3_readout;
        row_3_readout = self.column_9.is_low() as u8 * 0b00100000 + row_3_readout;
        row_3_readout = self.column_10.is_low() as u8 * 0b00010000 + row_3_readout;
        row_3_readout = self.column_11.is_low() as u8 * 0b00001000 + row_3_readout;
        row_3_readout = self.column_12.is_low() as u8 * 0b00000100 + row_3_readout;
        self.row_3.set_high();
        Timer::after_micros(100).await;

        let mut left_thumb_cluster_readout: u8 = 0;
        left_thumb_cluster_readout =
            self.rt_1.is_low() as u8 * 0b10000000 + left_thumb_cluster_readout;
        left_thumb_cluster_readout =
            self.rt_2.is_low() as u8 * 0b01000000 + left_thumb_cluster_readout;
        left_thumb_cluster_readout =
            self.rt_3.is_low() as u8 * 0b00100000 + left_thumb_cluster_readout;

        RightKeyReadout {
            row_1: row_1_readout,
            row_2: row_2_readout,
            row_3: row_3_readout,
            thumb_cluster: left_thumb_cluster_readout,
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct RightKeyReadout {
    row_1: u8,
    row_2: u8,
    row_3: u8,
    thumb_cluster: u8,
}

impl Default for RightKeyReadout {
    fn default() -> Self {
        return RightKeyReadout {
            row_1: 0,
            row_2: 0,
            row_3: 0,
            thumb_cluster: 0,
        };
    }
}

impl RightKeyReadout {
    /// Returns byte in [ row_1 , row_2 , row_3 , thumb_cluster ] order
    pub fn as_ne_bytes(&self) -> [u8; 4] {
        let ret = self.row_1 as u32;
        let mut ret = ret << 8;
        ret += self.row_2 as u32;
        let mut ret = ret << 8;
        ret += self.row_3 as u32;
        let mut ret = ret << 8;
        ret += self.thumb_cluster as u32;

        return ret.to_ne_bytes();
    }
}
