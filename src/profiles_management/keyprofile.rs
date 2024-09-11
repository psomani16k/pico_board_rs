pub mod keyboard_profile {
    use crate::{
        hid_helper::keyboard_report::KeyboardReport, io_manager::manager::LeftKeyboardReadOut,
        profiles_management::keyprofile_helper::keyboard_profile::KeyAction,
        report_buffer::buffer::KeyboardRingBuffer,
    };

    pub struct KeyboardProfile {
        previous_report: KeyboardReport,

        c1r1: KeyAction,
        c2r1: KeyAction,
        c3_r1: KeyAction,
        c4_r1: KeyAction,
        c5_r1: KeyAction,
        c6_r1: KeyAction,
        c1_r2: KeyAction,
        c2_r2: KeyAction,
        c3_r2: KeyAction,
        c4_r2: KeyAction,
        c5_r2: KeyAction,
        c6_r2: KeyAction,
        c1_r3: KeyAction,
        c2_r3: KeyAction,
        c3_r3: KeyAction,
        c4_r3: KeyAction,
        c5_r3: KeyAction,
        c6_r3: KeyAction,
        lt_1: KeyAction,
        lt_2: KeyAction,
        lt_3: KeyAction,
        // mode: KeyAction,
    }

    impl KeyboardProfile {
        pub fn process_readout(
            &self,
            left_readout: LeftKeyboardReadOut,
            // right_readout: RightKeyboardReadOut,
            buffer: &mut KeyboardRingBuffer,
        ) {
        }
    }
}
