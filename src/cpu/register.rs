#[derive(Debug, Default)]
pub struct Reg {
    pub x: u8,
    pub y: u8,
    pub a: u8,
    pub pc: u16,
    pub sp: u8,
    pub p: u8,
}

impl Reg {
    pub fn update_zero_and_negative_flags(&mut self, value: u8) -> u8 {
        if value == 0 {
            //is zero
            self.p = self.p | 0b0000_0010;
        } else {
            self.p = self.p & 0b1111_1101;
        }

        if value & 0b1000_0000 != 0 {
            //is negative
            self.p = self.p | 0b1000_0000;
        } else {
            self.p = self.p & 0b0111_1111;
        }

        self.p
    }
}
