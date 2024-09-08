#![no_std]
#![no_main]

use core::sync::atomic::{AtomicBool, Ordering};
use defmt::*;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Input, Output};
use embassy_rp::peripherals::USB;
use embassy_rp::usb::Driver;
use embassy_usb::class::hid::{HidReaderWriter, ReportId, RequestHandler, State};
use embassy_usb::control::OutResponse;
use embassy_usb::{Builder, Config, Handler};
use io_manager::manager::KeyboardIoManager;
use report_buffer::buffer::KeyboardRingBuffer;
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};
use {defmt_rtt as _, panic_probe as _};

mod hid_helper;
mod io_manager;
mod report_buffer;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
    I2C1_IRQ => embassy_rp::i2c::InterruptHandler<embassy_rp::peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // USB device creation and initialization.
    let driver = Driver::new(p.USB, Irqs);
    let mut config = Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("Parth Somani");
    config.product = Some("Parth's Keyboard");
    config.serial_number = Some("02122002");
    config.max_power = 150;
    config.max_packet_size_0 = 64;
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
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
    let config = embassy_usb::class::hid::Config {
        report_descriptor: KeyboardReport::desc(),
        request_handler: None,
        poll_ms: 60,
        max_packet_size: 64,
    };
    let hid = HidReaderWriter::<_, 1, 8>::new(&mut builder, &mut state, config);
    let mut usb = builder.build();
    let usb_fut = usb.run();
    let (reader, mut writer) = hid.split();

    // Pico I/O pin initialization

    // the bluetooth/usb switch button
    let bluetooth_trigger_key = Input::new(p.PIN_0, embassy_rp::gpio::Pull::Down);

    // caps lock led and display and mode button.
    let caps_led = Output::new(p.PIN_1, embassy_rp::gpio::Level::Low);

    // initiating the rows
    let row_1 = Output::new(p.PIN_2, embassy_rp::gpio::Level::Low);
    let row_2 = Output::new(p.PIN_3, embassy_rp::gpio::Level::Low);
    let row_3 = Output::new(p.PIN_4, embassy_rp::gpio::Level::Low);

    // initiating the columns
    let column_1 = Input::new(p.PIN_5, embassy_rp::gpio::Pull::Down);
    let column_2 = Input::new(p.PIN_6, embassy_rp::gpio::Pull::Down);
    let column_3 = Input::new(p.PIN_7, embassy_rp::gpio::Pull::Down);
    let column_4 = Input::new(p.PIN_8, embassy_rp::gpio::Pull::Down);
    let column_5 = Input::new(p.PIN_9, embassy_rp::gpio::Pull::Down);
    let column_6 = Input::new(p.PIN_10, embassy_rp::gpio::Pull::Down);

    // left thumb cluster
    let lt_1 = Input::new(p.PIN_11, embassy_rp::gpio::Pull::Down);
    let lt_2 = Input::new(p.PIN_12, embassy_rp::gpio::Pull::Down);
    let lt_3 = Input::new(p.PIN_13, embassy_rp::gpio::Pull::Down);

    // display pins
    let display_config = embassy_rp::i2c::Config::default();
    let display_bus =
        embassy_rp::i2c::I2c::new_async(p.I2C1, p.PIN_15, p.PIN_14, Irqs, display_config);

    // mode key
    let mode = Input::new(p.PIN_16, embassy_rp::gpio::Pull::Down);

    // the gpio containing struct
    let mut keyboard_io = KeyboardIoManager::new(
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
        mode,
        display_bus,
    );

    // initializing the ring buffer to store key strokes
    let report_buffer = KeyboardRingBuffer::new();

    let in_fut = async {
        loop {
            // performing a keyboard read
            let readout = keyboard_io.generate_readout().await;

            // generating a report from the readout and adding a unique report to the buffer
            
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
