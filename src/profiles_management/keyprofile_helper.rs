pub mod keyboard_profile {
    use crate::{
        hid_helper::{keyboard_report::KeyboardReport, keys::KeyCodes},
        report_buffer::buffer::KeyboardRingBuffer,
    };
    use heapless::Vec;

    /// A struct that encompasses 4 [KeyAction] for each key.
    pub struct KeyActionSet {
        base_action: KeyAction,
        fn_one_action: KeyAction,
        fn_two_action: KeyAction,
        fn_one_two_action: KeyAction,
    }

    /// An enum that carries the an action that needs to be carried out when the corresponding key is pressed
    /// and the associated data.  
    pub enum KeyAction {
        DeadKey,
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
                KeyAction::DeadKey => {
                    return false;
                }
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
}
