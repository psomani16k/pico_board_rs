use heapless::Vec;

use crate::{
    hid_helper::{keyboard_report::KeyboardReport, keys::KeyCodes},
    io_manager::manager::LeftKeyboardReadOut,
    report_buffer::buffer::KeyboardRingBuffer,
};

trait KeyProfile {
    fn process_report(
        buffer: &mut KeyboardRingBuffer,
        left_readout: LeftKeyboardReadOut,
        // right_readout: RightKeyboardReadOut
    );
}

enum KeyAction {
    HidKey(KeyCodes),
    HidReport(Vec<KeyboardReport, 30>),
}

impl KeyAction {
    pub fn add_to_buffer(
        &self,
        buffer: &mut KeyboardRingBuffer,
        report: &mut KeyboardReport,
    ) -> bool {
        match self {
            KeyAction::HidKey(key) => {
                report.add_keycode(key.clone());
                return false;
            }
            KeyAction::HidReport(reports) => {
                for i in reports {
                    if i.is_empty() {
                        return true;
                    }
                    buffer.put_report(i.clone());
                }
                return true;
            }
        }
    }
}

pub struct KeyActionSet {
    base_action: KeyAction,
    fn_one_action: KeyAction,
    fn_two_action: KeyAction,
    fn_one_two_action: KeyAction,
}