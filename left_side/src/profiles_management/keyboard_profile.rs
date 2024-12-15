pub mod keyboard_profile {
    use crate::{
        hid_helper::keyboard_report::KeyboardReportHelper,
        io_management::left_half_manager::{LeftKeyLocation, LeftReadout},
        report_buffer::buffer::KeyboardRingBuffer,
    };
    use heapless::Vec;
    use usbd_hid::descriptor::KeyboardUsage;

    pub struct KeyboardProfile {
        pub layer_key_1: LeftKeyLocation,
        pub layer_key_2: LeftKeyLocation,
        pub c1_r1: KeyActionSet,
        pub c2_r1: KeyActionSet,
        pub c3_r1: KeyActionSet,
        pub c4_r1: KeyActionSet,
        pub c5_r1: KeyActionSet,
        pub c6_r1: KeyActionSet,
        pub c1_r2: KeyActionSet,
        pub c2_r2: KeyActionSet,
        pub c3_r2: KeyActionSet,
        pub c4_r2: KeyActionSet,
        pub c5_r2: KeyActionSet,
        pub c6_r2: KeyActionSet,
        pub c1_r3: KeyActionSet,
        pub c2_r3: KeyActionSet,
        pub c3_r3: KeyActionSet,
        pub c4_r3: KeyActionSet,
        pub c5_r3: KeyActionSet,
        pub c6_r3: KeyActionSet,
        pub lt_1: KeyActionSet,
        pub lt_2: KeyActionSet,
        pub lt_3: KeyActionSet,
    }

    impl KeyboardProfile {
        pub fn process_readout(
            &self,
            location_helper: &LeftReadout,
            buffer: &mut KeyboardRingBuffer,
        ) {
            let layer_one = location_helper.is_pressed(&self.layer_key_1);
            let layer_two = location_helper.is_pressed(&self.layer_key_2);
            let layer = LayerLevel::new(layer_one, layer_two);
            let mut report = KeyboardReportHelper::new();

            if location_helper.is_pressed(&LeftKeyLocation::C1R1) {
                if self.c1_r1.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocation::C1R2) {
                if self.c1_r2.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocation::C1R3) {
                if self.c1_r3.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocation::C2R1) {
                if self.c2_r1.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocation::C2R2) {
                if self.c2_r2.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocation::C2R3) {
                if self.c2_r3.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocation::C3R1) {
                if self.c3_r1.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocation::C3R2) {
                if self.c3_r2.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocation::C3R3) {
                if self.c3_r3.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocation::C4R1) {
                if self.c4_r1.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocation::C4R2) {
                if self.c4_r2.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocation::C4R3) {
                if self.c4_r3.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocation::C5R1) {
                if self.c5_r1.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocation::C5R2) {
                if self.c5_r2.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocation::C5R3) {
                if self.c5_r3.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocation::C6R1) {
                if self.c6_r1.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocation::C6R2) {
                if self.c6_r2.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocation::C6R3) {
                if self.c6_r3.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocation::LT1) {
                if self.lt_1.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocation::LT2) {
                if self.lt_2.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocation::LT3) {
                if self.lt_3.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            buffer.put_report(report);
        }
    }

    pub struct KeyActionSet {
        pub(crate) base_action: KeyAction,
        pub(crate) layer_one_action: KeyAction,
        pub(crate) layer_two_action: KeyAction,
        pub(crate) layer_one_and_two_action: KeyAction,
    }

    impl KeyActionSet {
        fn process_key(
            &self,
            layer: &LayerLevel,
            buffer: &mut KeyboardRingBuffer,
            current_report: &mut KeyboardReportHelper,
        ) -> bool {
            match layer {
                LayerLevel::NoLayer => self.base_action.add_to_buffer(buffer, current_report),
                LayerLevel::LayerOne => self.layer_one_action.add_to_buffer(buffer, current_report),
                LayerLevel::LayerTwo => self.layer_two_action.add_to_buffer(buffer, current_report),
                LayerLevel::LayerOneAndTwo => self
                    .layer_one_and_two_action
                    .add_to_buffer(buffer, current_report),
            }
        }
    }

    enum LayerLevel {
        NoLayer,
        LayerOne,
        LayerTwo,
        LayerOneAndTwo,
    }
    impl LayerLevel {
        fn new(layer_one: bool, layer_two: bool) -> LayerLevel {
            if layer_one && layer_two {
                return LayerLevel::LayerOneAndTwo;
            }
            if layer_one {
                return LayerLevel::LayerOne;
            }
            if layer_two {
                return LayerLevel::LayerTwo;
            }
            return LayerLevel::NoLayer;
        }
    }

    /// An enum that carries the action that needs to be carried out when the corresponding key is pressed
    /// and the associated data.  
    pub enum KeyAction {
        DeadKey,
        HidKey(KeyboardUsage),
        HidReport(Vec<KeyboardReportHelper, 30>),
    }

    impl KeyAction {
        /// Returns true if the popultion process is complete and the hid report is added to the buffer
        /// (no need to check for any more keys). Returns false otherwise
        pub fn add_to_buffer(
            &self,
            buffer: &mut KeyboardRingBuffer,
            report: &mut KeyboardReportHelper,
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
