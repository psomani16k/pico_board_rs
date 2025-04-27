use core::{
    future::Future,
    sync::atomic::{AtomicBool, Ordering},
};

use defmt::info;
use embassy_rp::{
    bind_interrupts,
    peripherals::{I2C0, USB},
    usb::{Driver, InterruptHandler},
    Peripherals,
};
use embassy_usb::{
    class::hid::{HidReaderWriter, HidWriter, ReportId, RequestHandler, State},
    control::OutResponse,
    Builder, Handler, UsbDevice,
};
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
    I2C0_IRQ => embassy_rp::i2c::InterruptHandler<I2C0>;
});
pub struct UsbKeyboard<'a> {
    usb_device: UsbDevice<'a, Driver<'a, USB>>,
    writer: HidWriter<'a, Driver<'a, USB>, 8>,
}

impl<'a> UsbKeyboard<'_> {
    pub fn new(p: Peripherals) -> Self {
        // Create the driver, from the HAL.
        let driver = Driver::new(p.USB, Irqs);
        // Create embassy-usb Config
        let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
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
        let mut request_handler = UsbRequestHandler::new();
        let mut device_handler = UsbDeviceHandler::new();

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
        let (reader, mut writer) = hid.split();

        UsbKeyboard {
            usb_device: usb,
            writer,
        }
    }

    pub async fn run() {}
}

pub struct UsbRequestHandler {}

impl UsbRequestHandler {
    fn new() -> Self {
        UsbRequestHandler {}
    }
}

impl RequestHandler for UsbRequestHandler {
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

struct UsbDeviceHandler {
    configured: AtomicBool,
}

impl UsbDeviceHandler {
    fn new() -> Self {
        UsbDeviceHandler {
            configured: AtomicBool::new(false),
        }
    }
}

impl Handler for UsbDeviceHandler {
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
