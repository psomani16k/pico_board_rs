use crate::hid_helper::keys::{KeyCodes, Modifiers};
use serde::Serialize;
use usbd_hid::descriptor::AsInputReport;

#[derive(Serialize, Clone, Copy)]
pub struct KeyboardReport {
    pub modifier: u8,
    pub reserved: u8,
    pub keycodes: [u8; 12],
    pub pos: usize,
}

impl KeyboardReport {
    pub const fn new() -> KeyboardReport {
        return KeyboardReport {
            modifier: 0,
            reserved: 0,
            keycodes: [0; 12],
            pos: 0,
        };
    }

    pub fn add_keycode(&mut self, key: KeyCodes) -> bool {
        if self.pos == 12 {
            return false;
        }
        self.keycodes[self.pos] = key as u8;
        self.pos += 1;
        true
    }

    pub fn add_modifier(&mut self, modifier: Modifiers) {
        self.modifier = self.modifier | (modifier as u8);
    }

    pub const fn get_report_descriptor() -> [u8; 64] {
        return [
            0x05, 0x01, //        Usage Page (Generic Desktop)
            0x09, 0x06, //        Usage (Keyboard)
            0xA1, 0x01, //        Collection (Application)
            0x05, 0x07, //        Usage Page (Keyboard/Keypad)
            0x19, 0xE0, //        Usage Minimum (Keyboard Left Control)
            0x29, 0xE7, //        Usage Maximum (Keyboard Right GUI)
            0x15, 0x00, //        Logical Minimum (0)
            0x25, 0x01, //        Logical Maximum (1)
            0x75, 0x01, //        Report Size (1)
            0x95, 0x08, //        Report Count (8)
            0x81, 0x02, //        Input (Data,Var,Abs,NWrp,Lin,Pref,NNul,Bit)
            0x95, 0x01, //        Report Count (1)
            0x75, 0x08, //        Report Size (8)
            0x81, 0x01, //        Input (Const,Ary,Abs)
            0x95, 0x05, //        Report Count (5)
            0x75, 0x01, //        Report Size (1)
            0x05, 0x08, //        Usage Page (LEDs)
            0x19, 0x01, //        Usage Minimum (Num Lock)
            0x29, 0x05, //        Usage Maximum (Kana)
            0x91, 0x02, //        Output (Data,Var,Abs,NWrp,Lin,Pref,NNul,NVol,Bit)
            0x95, 0x01, //        Report Count (1)
            0x75, 0x03, //        Report Size (3)
            0x91, 0x01, //        Output (Const,Ary,Abs,NWrp,Lin,Pref,NNul,NVol,Bit)
            0x95,
            0x0C, //        Report Count (6) - this is the number of keycodes to carry we modify it to have 12 keys
            0x75, 0x08, //        Report Size (8)
            0x15, 0x00, //        Logical Minimum (0)
            0x26, 0x97, 0x00, //  Logical Maximum (151)
            0x05, 0x07, //        Usage Page (Keyboard/Keypad)
            0x19, 0x00, //        Usage Minimum (Undefined)
            0x29, 0x97, //        Usage Maximum (Keyboard LANG8)
            0x81, 0x00, //        Input (Data,Ary,Abs)
            0xC0, //        End Collection
        ];
    }

    pub fn get_sendable_report(&self) -> [u8; 14] {
        let report = [
            self.modifier,
            self.reserved,
            self.keycodes[0],
            self.keycodes[1],
            self.keycodes[2],
            self.keycodes[3],
            self.keycodes[4],
            self.keycodes[5],
            self.keycodes[6],
            self.keycodes[7],
            self.keycodes[8],
            self.keycodes[9],
            self.keycodes[10],
            self.keycodes[11],
        ];
        return report;
    }
}

impl AsInputReport for KeyboardReport {}
