use embassy_sync::blocking_mutex::{raw::CriticalSectionRawMutex, CriticalSectionMutex};

use crate::hid_helper::keyboard_report::KeyboardReport;

static mut KEYBOARD_READOUT_BUFFER: embassy_sync::blocking_mutex::Mutex<
    CriticalSectionRawMutex,
    KeyboardRingBuffer,
> = CriticalSectionMutex::new(KeyboardRingBuffer::new());

const BUFFER_SIZE: usize = 40;

struct KeyboardRingBuffer {
    pub the_buffer: [KeyboardReport; BUFFER_SIZE],
    pub entry_pos: usize,
    pub exit_pos: usize,
}

impl KeyboardRingBuffer {
    const fn new() -> KeyboardRingBuffer {
        return KeyboardRingBuffer {
            the_buffer: [KeyboardReport::new(); BUFFER_SIZE],
            entry_pos: 0,
            exit_pos: 0,
        };
    }

    pub fn enqueue(report: KeyboardReport) {
        unsafe {
            let keyboard_buffer = KEYBOARD_READOUT_BUFFER.get_mut();
            keyboard_buffer.put_report(report);
        };
    }

    pub fn dequeue() -> Option<KeyboardReport> {
        let report: Option<KeyboardReport>;
        unsafe {
            let keyboard_buffer = KEYBOARD_READOUT_BUFFER.get_mut();
            report = keyboard_buffer.get_report();
        };
        return report;
    }

    fn get_report(&mut self) -> Option<KeyboardReport> {
        if self.entry_pos == self.exit_pos {
            return None;
        }
        let report = self.the_buffer[self.exit_pos];
        self.exit_pos += 1;
        self.exit_pos = self.exit_pos % BUFFER_SIZE;
        return Some(report);
    }

    fn put_report(&mut self, report: KeyboardReport) {
        if self.entry_pos == self.exit_pos {
            return;
        }
        self.the_buffer[self.entry_pos] = report;
        self.entry_pos += 1;
        self.entry_pos = self.entry_pos % BUFFER_SIZE;
    }
}
