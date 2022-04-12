use bitflags::bitflags;

bitflags! {
    #[derive(Default)]
    pub struct ProcessorStatus: u8 {
        const CARRY             = 0b00000001;
        const ZERO              = 0b00000010;
        const INTERRUPT_DISABLE = 0b00000100;
        const DECIMAL           = 0b00001000;
        const BREAK             = 0b00010000;
        const BREAK_2           = 0b00100000;
        const OVERFLOW          = 0b01000000;
        const NEGATIVE          = 0b10000000;
    }
}

#[derive(Debug)]
pub struct Reg {
    pub x: u8,
    pub y: u8,
    pub a: u8,
    pub pc: u16,
    pub sp: u8,
    pub p: ProcessorStatus,
}

impl Default for Reg {
    fn default() -> Self {
        Reg {
            x: 0,
            y: 0,
            a: 0,
            pc: 0,
            sp: Reg::STACK_RESET,
            p: ProcessorStatus::default(),
        }
    }
}

impl Reg {
    const STACK_RESET: u8 = 0xfd;

    pub fn update_zero_and_negative_flags(&mut self, value: u8) -> ProcessorStatus {
        if value == 0 {
            //is zero
            self.p.insert(ProcessorStatus::ZERO);
        } else {
            self.p.remove(ProcessorStatus::ZERO);
        }

        if value & 0b1000_0000 != 0 {
            //is negative
            self.p.insert(ProcessorStatus::NEGATIVE);
        } else {
            self.p.remove(ProcessorStatus::NEGATIVE);
        }

        self.p
    }

    pub fn reset(&mut self, pc_start_addr: u16) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = Reg::STACK_RESET;
        self.p = ProcessorStatus::from_bits_truncate(0b100100);

        self.pc = pc_start_addr;
    }
}
