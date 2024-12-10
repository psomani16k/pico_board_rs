#![no_std]
#![no_main]

mod hid_helper;
mod profiles_management;
mod report_buffer;

use core::sync::atomic::{AtomicBool, Ordering};
use defmt::*;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Input, Output, Pull};
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_time::Timer;
use embassy_usb::class::hid::{HidReaderWriter, ReportId, RequestHandler, State};
use embassy_usb::control::OutResponse;
use embassy_usb::{Builder, Config, Handler};
use hid_helper::keyboard_report::KeyboardReportHelper;
use profiles_management::keyboard_profile::keyboard_profile::LeftKeyLocationHelper;
use profiles_management::profiles::profile_1::profile_1::get_profile;
use report_buffer::buffer::KeyboardRingBuffer;
use usbd_hid::descriptor::{KeyboardReport, KeyboardUsage, SerializedDescriptor};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // ----------------------------------------------------------------------------------------------------------------------------------------
    // ---------------------- INITIALIZING ----------------------------------------------------------------------------------------------------
    // ----------------------------------------------------------------------------------------------------------------------------------------
    let p = embassy_rp::init(Default::default());
    // Create the driver, from the HAL.
    let driver = Driver::new(p.USB, Irqs);

    // Create embassy-usb Config
    let mut config = Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("Parth");
    config.product = Some("Parth's Keyboard");
    config.serial_number = Some("12345678");
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    // Create embassy-usb DeviceBuilder using the driver and config.
    // It needs some buffers for building the descriptors.
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    // You can also add a Microsoft OS descriptor.
    let mut msos_descriptor = [0; 256];
    let mut control_buf = [0; 64];
    let mut request_handler = MyRequestHandler {};
    let mut device_handler = MyDeviceHandler::new();

    let mut state = State::new();

    let mut builder = Builder::new(
        driver,
        config,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut msos_descriptor,
        &mut control_buf,
    );

    builder.handler(&mut device_handler);

    // Create classes on the builder.
    let config = embassy_usb::class::hid::Config {
        report_descriptor: KeyboardReport::desc(),
        request_handler: None,
        poll_ms: 60,
        max_packet_size: 64,
    };
    let hid = HidReaderWriter::<_, 1, 8>::new(&mut builder, &mut state, config);

    // Build the builder.
    let mut usb = builder.build();

    // Run the USB device.
    let usb_fut = usb.run();
    let (reader, mut writer) = hid.split();

    // ----------------------------------------------------------------------------------------------------------------------------------------
    // ----------------------------------------------------------------------------------------------------------------------------------------
    // ----------------------------------------------------------------------------------------------------------------------------------------

    // ----------------------------------------------------------------------------------------------------------------------------------------
    // ------Setting up IO---------------------------------------------------------------------------------------------------------------------
    // ----------------------------------------------------------------------------------------------------------------------------------------
    // rows
    let mut row_1 = Output::new(p.PIN_0, embassy_rp::gpio::Level::High);
    let mut row_2 = Output::new(p.PIN_1, embassy_rp::gpio::Level::High);
    let mut row_3 = Output::new(p.PIN_2, embassy_rp::gpio::Level::High);

    // initiating the columns
    let mut column_1 = Input::new(p.PIN_3, embassy_rp::gpio::Pull::Up);
    column_1.set_schmitt(true);
    let mut column_2 = Input::new(p.PIN_4, embassy_rp::gpio::Pull::Up);
    column_2.set_schmitt(true);
    let mut column_3 = Input::new(p.PIN_5, embassy_rp::gpio::Pull::Up);
    column_3.set_schmitt(true);
    let mut column_4 = Input::new(p.PIN_6, embassy_rp::gpio::Pull::Up);
    column_4.set_schmitt(true);
    let mut column_5 = Input::new(p.PIN_7, embassy_rp::gpio::Pull::Up);
    column_5.set_schmitt(true);
    let mut column_6 = Input::new(p.PIN_8, embassy_rp::gpio::Pull::Up);
    column_6.set_schmitt(true);

    // left thumb cluster
    let mut lt_1 = Input::new(p.PIN_11, embassy_rp::gpio::Pull::Up);
    lt_1.set_schmitt(true);
    let mut lt_2 = Input::new(p.PIN_12, embassy_rp::gpio::Pull::Up);
    lt_2.set_schmitt(true);
    let mut lt_3 = Input::new(p.PIN_13, embassy_rp::gpio::Pull::Up);
    lt_3.set_schmitt(true);

    let mut buffer = KeyboardRingBuffer::new();

    let in_fut = async {
        let mut previous_row_1_readout: u8 = 0;
        let mut previous_row_2_readout: u8 = 0;
        let mut previous_row_3_readout: u8 = 0;
        let mut previous_left_thumb_cluster_readout: u8 = 0;
        let profile = get_profile();
        loop {
            // readout
            let mut row_1_readout: u8 = 0;
            row_1.set_low();
            Timer::after_micros(100).await;
            row_1_readout = column_1.is_low() as u8 * 0b00000001 + row_1_readout;
            row_1_readout = column_2.is_low() as u8 * 0b00000010 + row_1_readout;
            row_1_readout = column_3.is_low() as u8 * 0b00000100 + row_1_readout;
            row_1_readout = column_4.is_low() as u8 * 0b00001000 + row_1_readout;
            row_1_readout = column_5.is_low() as u8 * 0b00010000 + row_1_readout;
            row_1_readout = column_6.is_low() as u8 * 0b00100000 + row_1_readout;
            row_1.set_high();
            Timer::after_micros(100).await;

            let mut row_2_readout: u8 = 0;
            row_2.set_low();
            Timer::after_micros(100).await;
            row_2_readout = column_1.is_low() as u8 * 0b00000001 + row_2_readout;
            row_2_readout = column_2.is_low() as u8 * 0b00000010 + row_2_readout;
            row_2_readout = column_3.is_low() as u8 * 0b00000100 + row_2_readout;
            row_2_readout = column_4.is_low() as u8 * 0b00001000 + row_2_readout;
            row_2_readout = column_5.is_low() as u8 * 0b00010000 + row_2_readout;
            row_2_readout = column_6.is_low() as u8 * 0b00100000 + row_2_readout;
            row_2.set_high();
            Timer::after_micros(100).await;

            let mut row_3_readout: u8 = 0;
            row_3.set_low();
            Timer::after_micros(100).await;
            row_3_readout = column_1.is_low() as u8 * 0b00000001 + row_3_readout;
            row_3_readout = column_2.is_low() as u8 * 0b00000010 + row_3_readout;
            row_3_readout = column_3.is_low() as u8 * 0b00000100 + row_3_readout;
            row_3_readout = column_4.is_low() as u8 * 0b00001000 + row_3_readout;
            row_3_readout = column_5.is_low() as u8 * 0b00010000 + row_3_readout;
            row_3_readout = column_6.is_low() as u8 * 0b00100000 + row_3_readout;
            row_3.set_high();
            Timer::after_micros(100).await;

            let mut left_thumb_cluster_readout: u8 = 0;
            left_thumb_cluster_readout =
                lt_1.is_low() as u8 * 0b00000001 + left_thumb_cluster_readout;
            left_thumb_cluster_readout =
                lt_2.is_low() as u8 * 0b00000010 + left_thumb_cluster_readout;
            left_thumb_cluster_readout =
                lt_3.is_low() as u8 * 0b00000100 + left_thumb_cluster_readout;

            // eliminating duplicate readouts
            if row_1_readout == previous_row_1_readout
                && row_2_readout == previous_row_2_readout
                && row_3_readout == previous_row_3_readout
                && left_thumb_cluster_readout == previous_left_thumb_cluster_readout
            {
            } else {
                previous_row_1_readout = row_1_readout;
                previous_row_2_readout = row_2_readout;
                previous_row_3_readout = row_3_readout;
                previous_left_thumb_cluster_readout = left_thumb_cluster_readout;
                // processing the readout
                let location_helper = LeftKeyLocationHelper {
                    row_1: row_1_readout,
                    row_2: row_2_readout,
                    row_3: row_3_readout,
                    thumb_cluster: left_thumb_cluster_readout,
                };

                profile.process_readout(&location_helper, &mut buffer);
            }

            match buffer.get_report_helper() {
                Some(report_helper) => {
                    let report = report_helper.get_report();
                    writer.ready().await;
                    writer.write_serialize(&report).await;
                }
                None => {}
            };
            Timer::after_millis(5).await;
        }
    };

    let out_fut = async {
        reader.run(false, &mut request_handler).await;
    };

    // Run everything concurrently.
    // If we had made everything `'static` above instead, we could do this using separate tasks instead.
    join(usb_fut, join(in_fut, out_fut)).await;
}

struct MyRequestHandler {}

impl RequestHandler for MyRequestHandler {
    fn get_report(&mut self, id: ReportId, _buf: &mut [u8]) -> Option<usize> {
        info!("Get report for {:?}", id);
        None
    }

    fn set_report(&mut self, id: ReportId, data: &[u8]) -> OutResponse {
        info!("Set report for {:?}: {=[u8]}", id, data);
        OutResponse::Accepted
    }

    fn set_idle_ms(&mut self, id: Option<ReportId>, dur: u32) {
        info!("Set idle rate for {:?} to {:?}", id, dur);
    }

    fn get_idle_ms(&mut self, id: Option<ReportId>) -> Option<u32> {
        info!("Get idle rate for {:?}", id);
        None
    }
}

struct MyDeviceHandler {
    configured: AtomicBool,
}

impl MyDeviceHandler {
    fn new() -> Self {
        MyDeviceHandler {
            configured: AtomicBool::new(false),
        }
    }
}

impl Handler for MyDeviceHandler {
    fn enabled(&mut self, enabled: bool) {
        self.configured.store(false, Ordering::Relaxed);
        if enabled {
            info!("Device enabled");
        } else {
            info!("Device disabled");
        }
    }

    fn reset(&mut self) {
        self.configured.store(false, Ordering::Relaxed);
        info!("Bus reset, the Vbus current limit is 100mA");
    }

    fn addressed(&mut self, addr: u8) {
        self.configured.store(false, Ordering::Relaxed);
        info!("USB address set to: {}", addr);
    }

    fn configured(&mut self, configured: bool) {
        self.configured.store(configured, Ordering::Relaxed);
        if configured {
            info!(
                "Device configured, it may now draw up to the configured current limit from Vbus."
            )
        } else {
            info!("Device is no longer configured, the Vbus current limit is 100mA.");
        }
    }
}
