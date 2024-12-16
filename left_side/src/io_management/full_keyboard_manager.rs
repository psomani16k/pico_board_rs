use crate::{
    hid_helper::keyboard_report::KeyboardReportHelper,
    profiles_management::keyboard_profile::keyboard_profile::KeyboardProfile,
    report_buffer::buffer::KeyboardRingBuffer,
};

use super::{left_half_manager::LeftReadout, right_half_manager::RightReadout};

pub struct FullKeyboardManager {
    right_readout: RightReadout,
    left_readout: LeftReadout,
    buffer: KeyboardRingBuffer,
}

impl FullKeyboardManager {
    pub fn new(buffer: KeyboardRingBuffer) -> Self {
        return FullKeyboardManager {
            buffer,
            left_readout: LeftReadout::default(),
            right_readout: RightReadout::default(),
        };
    }

    pub fn update_right_readout(&mut self, readout: RightReadout, profile: &KeyboardProfile) {
        self.right_readout = readout;
        self.process_readouts(profile);
    }

    pub fn update_left_readout(&mut self, readout: LeftReadout, profile: &KeyboardProfile) {
        self.left_readout = readout;
        self.process_readouts(profile);
    }

    pub fn get_report_helper(&mut self) -> Option<KeyboardReportHelper> {
        self.buffer.get_report_helper()
    }

    fn process_readouts(&mut self, profile: &KeyboardProfile) {
        profile.process_readout(&self.left_readout, &self.right_readout, &mut self.buffer);
    }
}
