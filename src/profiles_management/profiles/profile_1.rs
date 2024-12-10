pub mod profile_1 {

    use heapless::Vec;
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
        hid_helper::keyboard_report::KeyboardReportHelper,
        profiles_management::keyboard_profile::keyboard_profile::{
            KeyAction, KeyActionSet, KeyboardProfile, LeftKeyLocations,
        },
    };

    pub fn get_profile() -> KeyboardProfile {
        return KeyboardProfile {
            layer_key_1: LeftKeyLocations::LT1,
            layer_key_2: LeftKeyLocations::LT2,
            c1_r1: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardTt),
                layer_one_action: KeyAction::HidKey(KeyboardUsage::Keyboard5Percent),
                layer_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardF6),
                layer_one_and_two_action: KeyAction::HidReport(
                    Vec::from_slice(&[
                        KeyboardReportHelper::from_values(0b00001000, 0, 0, 0, 0, 0, 0),
                        KeyboardReportHelper::from_values(
                            0b00000000,
                            KeyboardUsage::KeyboardFf as u8,
                            0,
                            0,
                            0,
                            0,
                            0,
                        ),
                        KeyboardReportHelper::from_values(
                            0b00000000,
                            KeyboardUsage::KeyboardIi as u8,
                            0,
                            0,
                            0,
                            0,
                            0,
                        ),
                        KeyboardReportHelper::from_values(
                            0b00000000,
                            KeyboardUsage::KeyboardRr as u8,
                            0,
                            0,
                            0,
                            0,
                            0,
                        ),
                        KeyboardReportHelper::from_values(
                            0b00000000,
                            KeyboardUsage::KeyboardEe as u8,
                            0,
                            0,
                            0,
                            0,
                            0,
                        ),
                        KeyboardReportHelper::from_values(
                            0b00000000,
                            KeyboardUsage::KeyboardFf as u8,
                            0,
                            0,
                            0,
                            0,
                            0,
                        ),
                        KeyboardReportHelper::from_values(
                            0b00000000,
                            KeyboardUsage::KeyboardOo as u8,
                            0,
                            0,
                            0,
                            0,
                            0,
                        ),
                        KeyboardReportHelper::from_values(
                            0b00000000,
                            KeyboardUsage::KeyboardXx as u8,
                            0,
                            0,
                            0,
                            0,
                            0,
                        ),
                        KeyboardReportHelper::from_values(
                            0b00000000,
                            KeyboardUsage::KeyboardReturn as u8,
                            0,
                            0,
                            0,
                            0,
                            0,
                        ),
                        KeyboardReportHelper::new(),
                    ])
                    .unwrap(),
                ),
            },
            c2_r1: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardRr),
                layer_one_action: KeyAction::HidKey(KeyboardUsage::Keyboard4Dollar),
                layer_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardF5),
                layer_one_and_two_action: KeyAction::DeadKey,
            },

            c3_r1: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardEe),
                layer_one_action: KeyAction::HidKey(KeyboardUsage::Keyboard3Hash),
                layer_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardF4),
                layer_one_and_two_action: KeyAction::DeadKey,
            },

            c4_r1: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardWw),
                layer_one_action: KeyAction::HidKey(KeyboardUsage::Keyboard2At),
                layer_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardF3),
                layer_one_and_two_action: KeyAction::DeadKey,
            },

            c5_r1: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardQq),
                layer_one_action: KeyAction::HidKey(KeyboardUsage::Keyboard1Exclamation),
                layer_two_action: KeyAction::HidKey(KeyboardUsage::KeyboardF2),
                layer_one_and_two_action: KeyAction::DeadKey,
            },

            c6_r1: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardTab),
                layer_one_action: KeyAction::DeadKey,
                layer_two_action: KeyAction::DeadKey,
                layer_one_and_two_action: KeyAction::DeadKey,
            },

            c1_r2: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardGg),
                layer_one_action: KeyAction::DeadKey,
                layer_two_action: KeyAction::DeadKey,
                layer_one_and_two_action: KeyAction::DeadKey,
            },

            c2_r2: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardFf),
                layer_one_action: KeyAction::DeadKey,
                layer_two_action: KeyAction::DeadKey,
                layer_one_and_two_action: KeyAction::DeadKey,
            },

            c3_r2: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardDd),
                layer_one_action: KeyAction::DeadKey,
                layer_two_action: KeyAction::DeadKey,
                layer_one_and_two_action: KeyAction::DeadKey,
            },

            c4_r2: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardSs),
                layer_one_action: KeyAction::DeadKey,
                layer_two_action: KeyAction::DeadKey,
                layer_one_and_two_action: KeyAction::DeadKey,
            },

            c5_r2: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardAa),
                layer_one_action: KeyAction::DeadKey,
                layer_two_action: KeyAction::DeadKey,
                layer_one_and_two_action: KeyAction::DeadKey,
            },

            c6_r2: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardLeftControl),
                layer_one_action: KeyAction::DeadKey,
                layer_two_action: KeyAction::DeadKey,
                layer_one_and_two_action: KeyAction::DeadKey,
            },

            c1_r3: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardBb),
                layer_one_action: KeyAction::DeadKey,
                layer_two_action: KeyAction::DeadKey,
                layer_one_and_two_action: KeyAction::DeadKey,
            },

            c2_r3: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardVv),
                layer_one_action: KeyAction::DeadKey,
                layer_two_action: KeyAction::DeadKey,
                layer_one_and_two_action: KeyAction::DeadKey,
            },

            c3_r3: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardCc),
                layer_one_action: KeyAction::DeadKey,
                layer_two_action: KeyAction::DeadKey,
                layer_one_and_two_action: KeyAction::DeadKey,
            },

            c4_r3: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardXx),
                layer_one_action: KeyAction::DeadKey,
                layer_two_action: KeyAction::DeadKey,
                layer_one_and_two_action: KeyAction::DeadKey,
            },

            c5_r3: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardZz),
                layer_one_action: KeyAction::DeadKey,
                layer_two_action: KeyAction::DeadKey,
                layer_one_and_two_action: KeyAction::DeadKey,
            },

            c6_r3: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardLeftShift),
                layer_one_action: KeyAction::DeadKey,
                layer_two_action: KeyAction::DeadKey,
                layer_one_and_two_action: KeyAction::DeadKey,
            },

            lt_1: KeyActionSet {
                base_action: KeyAction::DeadKey,
                layer_one_action: KeyAction::DeadKey,
                layer_two_action: KeyAction::DeadKey,
                layer_one_and_two_action: KeyAction::DeadKey,
            },

            lt_2: KeyActionSet {
                base_action: KeyAction::DeadKey,
                layer_one_action: KeyAction::DeadKey,
                layer_two_action: KeyAction::DeadKey,
                layer_one_and_two_action: KeyAction::DeadKey,
            },

            lt_3: KeyActionSet {
                base_action: KeyAction::HidKey(KeyboardUsage::KeyboardSpacebar),
                layer_one_action: KeyAction::DeadKey,
                layer_two_action: KeyAction::DeadKey,
                layer_one_and_two_action: KeyAction::DeadKey,
            },
        };
    }
}
