#[derive(PartialEq)]
pub struct RightReadout {
    row_1: u8,
    row_2: u8,
    row_3: u8,
    rtc: u8,
}

impl Default for RightReadout {
    fn default() -> Self {
        return RightReadout {
            row_1: 0,
            row_2: 0,
            row_3: 0,
            rtc: 0,
        };
    }
}

impl RightReadout {
    pub fn new(row_1: u8, row_2: u8, row_3: u8, right_thumb_cluster: u8) -> Self {
        return RightReadout {
            row_1,
            row_2,
            row_3,
            rtc: right_thumb_cluster,
        };
    }
    pub fn is_pressed(&self, location: &RightKeyLocation) -> bool {
        match location {
            RightKeyLocation::C7R1 => return self.row_1 & 0b10000000 != 0,
            RightKeyLocation::C8R1 => return self.row_1 & 0b01000000 != 0,
            RightKeyLocation::C9R1 => return self.row_1 & 0b00100000 != 0,
            RightKeyLocation::C10R1 => return self.row_1 & 0b00010000 != 0,
            RightKeyLocation::C11R1 => return self.row_1 & 0b00001000 != 0,
            RightKeyLocation::C12R1 => return self.row_1 & 0b00000100 != 0,
            RightKeyLocation::C7R2 => return self.row_2 & 0b10000000 != 0,
            RightKeyLocation::C8R2 => return self.row_2 & 0b01000000 != 0,
            RightKeyLocation::C9R2 => return self.row_2 & 0b00100000 != 0,
            RightKeyLocation::C10R2 => return self.row_2 & 0b00010000 != 0,
            RightKeyLocation::C11R2 => return self.row_2 & 0b00001000 != 0,
            RightKeyLocation::C12R2 => return self.row_2 & 0b00000100 != 0,
            RightKeyLocation::C7R3 => return self.row_3 & 0b10000000 != 0,
            RightKeyLocation::C8R3 => return self.row_3 & 0b01000000 != 0,
            RightKeyLocation::C9R3 => return self.row_3 & 0b00100000 != 0,
            RightKeyLocation::C10R3 => return self.row_3 & 0b00010000 != 0,
            RightKeyLocation::C11R3 => return self.row_3 & 0b00001000 != 0,
            RightKeyLocation::C12R3 => return self.row_3 & 0b00000100 != 0,
            RightKeyLocation::RT1 => return self.rtc & 0b10000000 != 0,
            RightKeyLocation::RT2 => return self.rtc & 0b01000000 != 0,
            RightKeyLocation::RT3 => return self.rtc & 0b00100000 != 0,
        }
    }
}

pub enum RightKeyLocation {
    C7R1,
    C8R1,
    C9R1,
    C10R1,
    C11R1,
    C12R1,
    C7R2,
    C8R2,
    C9R2,
    C10R2,
    C11R2,
    C12R2,
    C7R3,
    C8R3,
    C9R3,
    C10R3,
    C11R3,
    C12R3,
    RT1,
    RT2,
    RT3,
}
