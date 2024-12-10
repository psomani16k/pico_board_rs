use usbd_hid::descriptor::KeyboardReport;

use crate::hid_helper::keyboard_report::KeyboardReportHelper;

const BUFFER_SIZE: u8 = 100;

pub(crate) struct KeyboardRingBuffer {
    the_buffer: [KeyboardReportHelper; BUFFER_SIZE as usize],
    entry_pos: usize,
    exit_pos: usize,
    count: u8,
}

impl KeyboardRingBuffer {
    pub const fn new() -> KeyboardRingBuffer {
        return KeyboardRingBuffer {
            the_buffer: [KeyboardReportHelper::new(); BUFFER_SIZE as usize],
            entry_pos: 0,
            exit_pos: 0,
            count: 0,
        };
    }

    pub fn get_report_helper(&mut self) -> Option<KeyboardReportHelper> {
        if self.entry_pos == self.exit_pos && self.count == 0 {
            return None;
        }
        let report = self.the_buffer[self.exit_pos];
        self.exit_pos += 1;
        self.count -= 1;
        self.exit_pos = self.exit_pos % BUFFER_SIZE as usize;
        return Some(report);
    }

    fn peek(&self) -> Option<KeyboardReportHelper> {
        if self.entry_pos == self.exit_pos {
            return None;
        }
        let report = self.the_buffer[self.exit_pos];
        return Some(report);
    }

    /// Puts the provided [KeyboardReportHelper] into the [KeyboardRingBuffer],
    /// if the report is the same as the previously entered report, then it is ignored.
    pub fn put_report(&mut self, report: KeyboardReportHelper) {
        if self.entry_pos == self.exit_pos && self.count != 0 {
            return;
        }
        self.the_buffer[self.entry_pos] = report;
        self.entry_pos += 1;
        self.entry_pos = self.entry_pos % BUFFER_SIZE as usize;
        self.count += 1;
    }
}
