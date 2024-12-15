#![no_std]
#![no_main]

mod hid_helper;
mod io_management;
mod profiles_management;
mod report_buffer;

use core::sync::atomic::{AtomicBool, Ordering};
use defmt::*;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_time::Timer;
use embassy_usb::class::hid::{HidReaderWriter, ReportId, RequestHandler, State};
use embassy_usb::control::OutResponse;
use embassy_usb::{Builder, Config, Handler};
use fixed::types::extra::{U8, U80};
use hid_helper::keyboard_report::KeyboardReportHelper;
use io_management::left_half_manager::{LeftIoManager, LeftReadout};
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

    let mut buffer = KeyboardRingBuffer::new();
    let mut left_io_manager = LeftIoManager::new(
        p.PIN_0, p.PIN_1, p.PIN_2, p.PIN_8, p.PIN_7, p.PIN_6, p.PIN_5, p.PIN_4, p.PIN_3, p.PIN_11,
        p.PIN_12, p.PIN_13,
    );

    let profile = get_profile();

    let in_fut = async {
        let mut previous_readout = LeftReadout::default();
        loop {
            let readout = left_io_manager.produce_readout().await;
            // eliminating duplicate readouts
            if readout != previous_readout {
                // processing the readout
                previous_readout = readout;
                profile.process_readout(&previous_readout, &mut buffer);
                // } else {
                //     buffer.put_report(KeyboardReportHelper::from_values(
                //         0,
                //         KeyboardUsage::KeyboardAa as u8,
                //         0,
                //         0,
                //         0,
                //         0,
                //         0,
                //     /*  x/));
            }
            match buffer.get_report_helper() {
                Some(report_helper) => {
                    let report = report_helper.get_report();
                    writer.ready().await;
                    writer.write_serialize(&report).await.unwrap();
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
