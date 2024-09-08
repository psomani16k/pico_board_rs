use crate::hid_helper::keyboard_report::KeyboardReport;

const BUFFER_SIZE: usize = 40;

pub struct KeyboardRingBuffer {
    pub the_buffer: [KeyboardReport; BUFFER_SIZE],
    pub entry_pos: usize,
    pub exit_pos: usize,
}

impl KeyboardRingBuffer {
    pub fn new() -> KeyboardRingBuffer {
        return KeyboardRingBuffer {
            the_buffer: [KeyboardReport::new(); BUFFER_SIZE],
            entry_pos: 0,
            exit_pos: 0,
        };
    }

    pub fn get_report(&mut self) -> Option<KeyboardReport> {
        if self.entry_pos == self.exit_pos {
            return None;
        }
        let report = self.the_buffer[self.exit_pos];
        self.exit_pos += 1;
        self.exit_pos = self.exit_pos % BUFFER_SIZE;
        return Some(report);
    }

    pub fn put_report(&mut self, report: KeyboardReport) {
        if self.entry_pos == self.exit_pos {
            return;
        }
        self.the_buffer[self.entry_pos] = report;
        self.entry_pos += 1;
        self.entry_pos = self.entry_pos % BUFFER_SIZE;
    }
}
