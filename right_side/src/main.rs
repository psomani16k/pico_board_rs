//! This example shows how to use the 2040 as an i2c slave.
#![no_std]
#![no_main]

mod io_management;

use embassy_executor::Spawner;
use embassy_rp::peripherals::I2C0;
use embassy_rp::{bind_interrupts, i2c};
use embassy_time::Timer;
use io_management::key_manager::{RightIoKeyManager, RightKeyReadout};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    I2C0_IRQ => i2c::InterruptHandler<I2C0>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    const LEFT_KEYBOARD_ADDRESS: u16 = 0x0069;

    let p = embassy_rp::init(Default::default());

    let left_sda = p.PIN_16;
    let left_scl = p.PIN_17;
    let mut config = i2c::Config::default();
    config.frequency = 1_000_000;
    let mut device = i2c::I2c::new_async(p.I2C0, left_scl, left_sda, Irqs, config);
    let mut key_io_handler = RightIoKeyManager::new(
        p.PIN_15, p.PIN_14, p.PIN_13, p.PIN_12, p.PIN_11, p.PIN_10, p.PIN_9, p.PIN_8, p.PIN_7,
        p.PIN_4, p.PIN_3, p.PIN_2,
    );

    // waiting for others to be ready
    Timer::after_millis(300).await;

    let mut previous_readout = RightKeyReadout::default();

    loop {
        let readout = key_io_handler.produce_readout().await;
        if previous_readout != readout {
            previous_readout = readout;
            let bytes = previous_readout.as_ne_bytes();
            device.write_async(LEFT_KEYBOARD_ADDRESS, bytes).await;
        }
        Timer::after_millis(2).await;
    }
}
