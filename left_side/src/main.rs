#![no_std]
#![no_main]

mod ble_hid;
mod hid_helper;
mod io_management;
mod profiles_management;
mod report_buffer;
mod usb_hid;

use core::sync::atomic::{AtomicBool, Ordering};
use defmt::*;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_rp::gpio::{Input, Pin};
use embassy_rp::peripherals::{self, I2C0, USB};
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_rp::{bind_interrupts, i2c_slave, Peripheral};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::Timer;
use embassy_usb::class::hid::{HidReaderWriter, ReportId, RequestHandler, State};
use embassy_usb::control::OutResponse;
use embassy_usb::{Builder, Config, Handler};
use io_management::full_keyboard_manager::FullKeyboardManager;
use io_management::left_half_manager::{LeftIoManager, LeftReadout};
use io_management::right_half_manager::RightReadout;
use profiles_management::profiles::profile_1::profile_1::get_profile;
use report_buffer::buffer::KeyboardRingBuffer;
use usb_hid::usb_main::UsbKeyboard;
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
    I2C0_IRQ => embassy_rp::i2c::InterruptHandler<I2C0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());
    let usb;

    // doing this is safe as PIN_15 will (note to self: SHOULD NOT!!) not be
    // used later.
    unsafe {
        let comm_mode = peripherals.PIN_15.clone_unchecked();
        let comm_mode = Input::new(comm_mode, embassy_rp::gpio::Pull::Down);
        usb = comm_mode.is_low();
    }

    if usb {
        // Keyboard Over USB
        let usb_keyboard = UsbKeyboard::new(peripherals);
    } else {
        // Keyboard Over BLE
    }
}

// #[embassy_executor::main]
async fn oldmain(_spawner: Spawner) {
    // ----------------------------------------------------------------------
    // ---------------------- INITIALIZING ----------------------------------
    // ----------------------------------------------------------------------
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

    // ----------------------------------------------------------------------
    // ------Setting up IO---------------------------------------------------
    // ----------------------------------------------------------------------
    // rows

    // let mut buffer_mutex: Mutex<ThreadModeRawMutex, KeyboardRingBuffer> =
    //     Mutex::new(KeyboardRingBuffer::new());
    let ring_buffer = KeyboardRingBuffer::new();
    let readout_mutex = FullKeyboardManager::new(ring_buffer);
    let readout_mutex: Mutex<ThreadModeRawMutex, FullKeyboardManager> = Mutex::new(readout_mutex);
    let mut left_io_manager = LeftIoManager::new(
        p.PIN_0, p.PIN_1, p.PIN_2, p.PIN_8, p.PIN_7, p.PIN_6, p.PIN_5, p.PIN_4, p.PIN_3, p.PIN_11,
        p.PIN_12, p.PIN_13,
    );

    const LEFT_KEYBOARD_ADDRESS: u16 = 0x0069;

    let left_sda = p.PIN_16;
    let left_scl = p.PIN_17;
    let mut config = i2c_slave::Config::default();
    config.addr = LEFT_KEYBOARD_ADDRESS;

    let mut device = i2c_slave::I2cSlave::new(p.I2C0, left_scl, left_sda, Irqs, config);

    let profile = get_profile();

    let right_fut = async {
        let mut i2c_buffer: [u8; 4] = [0, 0, 0, 0];
        loop {
            device.listen(&mut i2c_buffer).await;
            let readout =
                RightReadout::new(i2c_buffer[3], i2c_buffer[2], i2c_buffer[1], i2c_buffer[0]);
            let mut readout_manager = readout_mutex.lock().await;
            readout_manager.update_right_readout(readout, &profile);
        }
    };

    let left_fut = async {
        let mut previous_readout = LeftReadout::default();
        loop {
            let readout = left_io_manager.produce_readout().await;
            // eliminating duplicate readouts
            if readout != previous_readout {
                // processing the readout
                previous_readout = readout;
                let mut readout_manager = readout_mutex.lock().await;
                readout_manager.update_left_readout(previous_readout, &profile);
            }
            Timer::after_millis(2).await;
        }
    };

    let readout_fut = join(right_fut, left_fut);

    let in_fut = async {
        loop {
            let mut readout_manager = readout_mutex.lock().await;
            let report_helper = readout_manager.get_report_helper();
            drop(readout_manager);
            match report_helper {
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

    let io_fut = join(out_fut, in_fut);
    let keyboard_fut = join(io_fut, readout_fut);
    // Run everything concurrently.
    // If we had made everything `'static` above instead, we could do this using separate tasks instead.
    join(usb_fut, keyboard_fut).await;
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
