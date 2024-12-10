pub mod keyboard_profile {
    use crate::{
        hid_helper::keyboard_report::KeyboardReportHelper,
        report_buffer::buffer::KeyboardRingBuffer,
    };
    use heapless::Vec;
    use usbd_hid::descriptor::KeyboardUsage;

    pub struct KeyboardProfile {
        pub layer_key_1: LeftKeyLocations,
        pub layer_key_2: LeftKeyLocations,
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
            location_helper: &LeftKeyLocationHelper,
            buffer: &mut KeyboardRingBuffer,
        ) {
            let layer_one = location_helper.is_pressed(&self.layer_key_1);
            let layer_two = location_helper.is_pressed(&self.layer_key_2);
            let layer = LayerLevel::new(layer_one, layer_two);
            let mut report = KeyboardReportHelper::new();

            if location_helper.is_pressed(&LeftKeyLocations::C1R1) {
                if self.c1_r1.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocations::C1R2) {
                if self.c1_r2.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocations::C1R3) {
                if self.c1_r3.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocations::C2R1) {
                if self.c2_r1.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocations::C2R2) {
                if self.c2_r2.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocations::C2R3) {
                if self.c2_r3.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocations::C3R1) {
                if self.c3_r1.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocations::C3R2) {
                if self.c3_r2.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocations::C3R3) {
                if self.c3_r3.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocations::C4R1) {
                if self.c4_r1.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocations::C4R2) {
                if self.c4_r2.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocations::C4R3) {
                if self.c4_r3.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocations::C5R1) {
                if self.c5_r1.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocations::C5R2) {
                if self.c5_r2.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocations::C5R3) {
                if self.c5_r3.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocations::C6R1) {
                if self.c6_r1.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocations::C6R2) {
                if self.c6_r2.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocations::C6R3) {
                if self.c6_r3.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocations::LT1) {
                if self.lt_1.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocations::LT2) {
                if self.lt_2.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            if location_helper.is_pressed(&LeftKeyLocations::LT3) {
                if self.lt_3.process_key(&layer, buffer, &mut report) {
                    return;
                }
            }
            buffer.put_report(report);
        }

        fn get_action_set(&self, key: LeftKeyLocations) -> &KeyActionSet {
            match key {
                LeftKeyLocations::C1R1 => &self.c1_r1,
                LeftKeyLocations::C2R1 => &self.c2_r1,
                LeftKeyLocations::C3R1 => &self.c3_r1,
                LeftKeyLocations::C4R1 => &self.c4_r1,
                LeftKeyLocations::C5R1 => &self.c5_r1,
                LeftKeyLocations::C6R1 => &self.c6_r1,
                LeftKeyLocations::C1R2 => &self.c1_r2,
                LeftKeyLocations::C2R2 => &self.c2_r2,
                LeftKeyLocations::C3R2 => &self.c3_r2,
                LeftKeyLocations::C4R2 => &self.c4_r2,
                LeftKeyLocations::C5R2 => &self.c5_r2,
                LeftKeyLocations::C6R2 => &self.c6_r2,
                LeftKeyLocations::C1R3 => &self.c1_r3,
                LeftKeyLocations::C2R3 => &self.c2_r3,
                LeftKeyLocations::C3R3 => &self.c3_r3,
                LeftKeyLocations::C4R3 => &self.c4_r3,
                LeftKeyLocations::C5R3 => &self.c5_r3,
                LeftKeyLocations::C6R3 => &self.c6_r3,
                LeftKeyLocations::LT1 => &self.lt_1,
                LeftKeyLocations::LT2 => &self.lt_2,
                LeftKeyLocations::LT3 => &self.lt_3,
            }
        }
    }

    pub(crate) enum LeftKeyLocations {
        C1R1,
        C2R1,
        C3R1,
        C4R1,
        C5R1,
        C6R1,
        C1R2,
        C2R2,
        C3R2,
        C4R2,
        C5R2,
        C6R2,
        C1R3,
        C2R3,
        C3R3,
        C4R3,
        C5R3,
        C6R3,
        LT1,
        LT2,
        LT3,
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

    pub struct LeftKeyLocationHelper {
        pub row_1: u8,
        pub row_2: u8,
        pub row_3: u8,
        pub thumb_cluster: u8,
    }

    impl LeftKeyLocationHelper {
        pub fn is_pressed(&self, location: &LeftKeyLocations) -> bool {
            match location {
                LeftKeyLocations::C1R1 => self.row_1 & 0b00000001 != 0,
                LeftKeyLocations::C2R1 => self.row_1 & 0b00000010 != 0,
                LeftKeyLocations::C3R1 => self.row_1 & 0b00000100 != 0,
                LeftKeyLocations::C4R1 => self.row_1 & 0b00001000 != 0,
                LeftKeyLocations::C5R1 => self.row_1 & 0b00010000 != 0,
                LeftKeyLocations::C6R1 => self.row_1 & 0b00100000 != 0,
                LeftKeyLocations::C1R2 => self.row_2 & 0b00000001 != 0,
                LeftKeyLocations::C2R2 => self.row_2 & 0b00000010 != 0,
                LeftKeyLocations::C3R2 => self.row_2 & 0b00000100 != 0,
                LeftKeyLocations::C4R2 => self.row_2 & 0b00001000 != 0,
                LeftKeyLocations::C5R2 => self.row_2 & 0b00010000 != 0,
                LeftKeyLocations::C6R2 => self.row_2 & 0b00100000 != 0,
                LeftKeyLocations::C1R3 => self.row_3 & 0b00000001 != 0,
                LeftKeyLocations::C2R3 => self.row_3 & 0b00000010 != 0,
                LeftKeyLocations::C3R3 => self.row_3 & 0b00000100 != 0,
                LeftKeyLocations::C4R3 => self.row_3 & 0b00001000 != 0,
                LeftKeyLocations::C5R3 => self.row_3 & 0b00010000 != 0,
                LeftKeyLocations::C6R3 => self.row_3 & 0b00100000 != 0,
                LeftKeyLocations::LT1 => self.thumb_cluster & 0b00000001 != 0,
                LeftKeyLocations::LT2 => self.thumb_cluster & 0b00000010 != 0,
                LeftKeyLocations::LT3 => self.thumb_cluster & 0b00000100 != 0,
            }
        }
    }
}
