use serde::Serialize;
use usbd_hid::descriptor::{KeyboardReport, KeyboardUsage};

#[derive(Serialize, Clone, Copy, PartialEq, Eq)]
pub struct KeyboardReportHelper {
    report: KeyboardReport,
    pos: usize,
}

impl KeyboardReportHelper {
    pub const fn new() -> KeyboardReportHelper {
        return KeyboardReportHelper {
            report: KeyboardReport::default(),
            pos: 0,
        };
    }

    pub fn from_values(
        modifier: u8,
        key_1: u8,
        key_2: u8,
        key_3: u8,
        key_4: u8,
        key_5: u8,
        key_6: u8,
    ) -> KeyboardReportHelper {
        return KeyboardReportHelper {
            report: KeyboardReport {
                keycodes: [key_1, key_2, key_3, key_4, key_5, key_6],
                modifier: modifier,
                leds: 0,
                reserved: 0,
            },
            pos: 6,
        };
    }

    pub fn is_empty(&self) -> bool {
        return self.pos == 0;
    }

    pub fn add_keycode(&mut self, key: KeyboardUsage) {
        match key {
            KeyboardUsage::KeyboardLeftShift => self.add_modifier(Modifiers::LeftShift),
            KeyboardUsage::KeyboardRightShift => self.add_modifier(Modifiers::RightShift),
            KeyboardUsage::KeyboardLeftGUI => self.add_modifier(Modifiers::LeftGui),
            KeyboardUsage::KeyboardRightGUI => self.add_modifier(Modifiers::RightGui),
            KeyboardUsage::KeyboardLeftControl => self.add_modifier(Modifiers::LeftCtrl),
            KeyboardUsage::KeyboardRightControl => self.add_modifier(Modifiers::RightCtrl),
            KeyboardUsage::KeyboardLeftAlt => self.add_modifier(Modifiers::LeftAlt),
            KeyboardUsage::KeyboardRightAlt => self.add_modifier(Modifiers::RightAlt),
            key => {
                if self.pos == 6 {
                    return;
                }
                self.report.keycodes[self.pos] = key as u8;
                self.pos += 1;
            }
        }
    }

    pub fn get_report(&self) -> KeyboardReport {
        return self.report;
    }

    fn add_modifier(&mut self, modifier: Modifiers) {
        self.report.modifier = self.report.modifier | (modifier as u8);
    }
}

enum Modifiers {
    LeftCtrl = 0b00000001,
    LeftShift = 0b00000010,
    LeftAlt = 0b00000100,
    LeftGui = 0b00001000,
    RightCtrl = 0b00010000,
    RightShift = 0b00100000,
    RightAlt = 0b01000000,
    RightGui = 0b10000000,
}
