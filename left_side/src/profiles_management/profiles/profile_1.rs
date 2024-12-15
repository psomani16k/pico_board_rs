pub mod profile_1 {

    use usbd_hid::descriptor::KeyboardUsage;

    // LeftCtrl    = 0b00000001,
    // LeftShift   = 0b00000010,
    // LeftAlt     = 0b00000100,
    // LeftGui     = 0b00001000,
    // RightCtrl   = 0b00010000,
    // RightShift  = 0b00100000,
    // RightAlt    = 0b01000000,
    // RightGui    = 0x10000000,

    use crate::{
        io_management::left_half_manager::LeftKeyLocation,
        profiles_management::keyboard_profile::keyboard_profile::{
            KeyAction, KeyActionSet, KeyboardProfile,
        },
    };

    pub fn get_profile() -> KeyboardProfile {
        return KeyboardProfile {
            layer_key_1: LeftKeyLocation::LT3,
            layer_key_2: LeftKeyLocation::LT2,
            c1_r1: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardAa),
                layer_one_action: KeyAction::HidKey(KeyboardUsage::KeyboardBb),
                layer_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardCc),
                layer_one_and_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardDd),
            },
            c2_r1: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardAa),
                layer_one_action: KeyAction::HidKey(KeyboardUsage::KeyboardBb),
                layer_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardCc),
                layer_one_and_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardDd),
            },
            c3_r1: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardAa),
                layer_one_action: KeyAction::HidKey(KeyboardUsage::KeyboardBb),
                layer_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardCc),
                layer_one_and_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardDd),
            },
            c4_r1: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardAa),
                layer_one_action: KeyAction::HidKey(KeyboardUsage::KeyboardBb),
                layer_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardCc),
                layer_one_and_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardDd),
            },
            c5_r1: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardAa),
                layer_one_action: KeyAction::HidKey(KeyboardUsage::KeyboardBb),
                layer_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardCc),
                layer_one_and_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardDd),
            },
            c6_r1: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardAa),
                layer_one_action: KeyAction::HidKey(KeyboardUsage::KeyboardBb),
                layer_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardCc),
                layer_one_and_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardDd),
            },
            c1_r2: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardAa),
                layer_one_action: KeyAction::HidKey(KeyboardUsage::KeyboardBb),
                layer_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardCc),
                layer_one_and_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardDd),
            },
            c2_r2: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardAa),
                layer_one_action: KeyAction::HidKey(KeyboardUsage::KeyboardBb),
                layer_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardCc),
                layer_one_and_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardDd),
            },
            c3_r2: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardAa),
                layer_one_action: KeyAction::HidKey(KeyboardUsage::KeyboardBb),
                layer_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardCc),
                layer_one_and_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardDd),
            },
            c4_r2: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardAa),
                layer_one_action: KeyAction::HidKey(KeyboardUsage::KeyboardBb),
                layer_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardCc),
                layer_one_and_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardDd),
            },
            c5_r2: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardAa),
                layer_one_action: KeyAction::HidKey(KeyboardUsage::KeyboardBb),
                layer_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardCc),
                layer_one_and_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardDd),
            },
            c6_r2: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardAa),
                layer_one_action: KeyAction::HidKey(KeyboardUsage::KeyboardBb),
                layer_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardCc),
                layer_one_and_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardDd),
            },
            c1_r3: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardAa),
                layer_one_action: KeyAction::HidKey(KeyboardUsage::KeyboardBb),
                layer_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardCc),
                layer_one_and_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardDd),
            },
            c2_r3: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardAa),
                layer_one_action: KeyAction::HidKey(KeyboardUsage::KeyboardBb),
                layer_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardCc),
                layer_one_and_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardDd),
            },
            c3_r3: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardAa),
                layer_one_action: KeyAction::HidKey(KeyboardUsage::KeyboardBb),
                layer_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardCc),
                layer_one_and_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardDd),
            },
            c4_r3: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardAa),
                layer_one_action: KeyAction::HidKey(KeyboardUsage::KeyboardBb),
                layer_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardCc),
                layer_one_and_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardDd),
            },
            c5_r3: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardAa),
                layer_one_action: KeyAction::HidKey(KeyboardUsage::KeyboardBb),
                layer_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardCc),
                layer_one_and_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardDd),
            },
            c6_r3: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardAa),
                layer_one_action: KeyAction::HidKey(KeyboardUsage::KeyboardBb),
                layer_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardCc),
                layer_one_and_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardDd),
            },
            lt_1: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardAa),
                layer_one_action: KeyAction::HidKey(KeyboardUsage::KeyboardBb),
                layer_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardCc),
                layer_one_and_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardDd),
            },
            lt_2: KeyActionSet {
                base_action: KeyAction::DeadKey,
                layer_one_action: KeyAction::DeadKey,
                layer_two_action: KeyAction::DeadKey,
                layer_one_and_two_action: KeyAction::DeadKey,
            },
            lt_3: KeyActionSet {
                base_action: KeyAction::DeadKey,
                layer_one_action: KeyAction::DeadKey,
                layer_two_action: KeyAction::DeadKey,
                layer_one_and_two_action: KeyAction::DeadKey,
            },
        };
    }
}
