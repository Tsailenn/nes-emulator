pub mod mem;
pub mod opcode;
pub mod register;
pub mod test;

use mem::Mem;
use opcode::{AddressingMode, OpCode};
use register::Reg;

use self::register::ProcessorStatus;

#[derive(Debug, Default)]
pub struct CPU {
    mem: Mem,
    reg: Reg,
}

impl CPU {
    const STACK: u16 = 0x0100;

    pub fn new() -> Self {
        CPU::default()
    }

    pub fn lda(&mut self, addr: u16) {
        // let param = program[self.program_counter as usize];
        // self.program_counter += 1;
        // self.register_a = param;

        todo!()
    }

    pub fn run(&mut self) {
        loop {
            //the pc-pointed instruction
            let word = self.mem.read(self.reg.pc);

            //execute
            self.execute(word);
        }
    }

    pub fn reset(&mut self) {
        let pc_start_addr = self.mem.read_u16(0xFFFC);
        self.reg.reset(pc_start_addr);
    }

    fn execute(&mut self, word: u8) {
        //instruction first
        let op_code = OpCode::map(word).unwrap();
        self.reg.pc += 1;
        //then data
        let addr = self.get_operand_address(&op_code.mode);
        self.reg.pc += 1;
    }

    pub fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        //addr
        let addr = match mode {
            AddressingMode::Immediate => self.reg.pc,
            AddressingMode::ZeroPage => self.mem.read(self.reg.pc) as u16,
            AddressingMode::Absolute => self.mem.read_u16(self.reg.pc),
            AddressingMode::ZeroPageX => {
                let mem_data = self.mem.read(self.reg.pc);
                let data = mem_data.wrapping_add(self.reg.x) as u16;
                data
            }
            AddressingMode::ZeroPageY => {
                let mem_data = self.mem.read(self.reg.pc);
                let data = mem_data.wrapping_add(self.reg.y) as u16;
                data
            }
            AddressingMode::AbsoluteX => {
                let mem_data = self.mem.read_u16(self.reg.pc);
                let data = mem_data.wrapping_add(self.reg.x as u16);
                data
            }
            AddressingMode::AbsoluteY => {
                let mem_data = self.mem.read_u16(self.reg.pc);
                let data = mem_data.wrapping_add(self.reg.y as u16);
                data
            }
            AddressingMode::IndirectX => {
                let base_ptr = self.mem.read(self.reg.pc);
                let ptr = (base_ptr as u8).wrapping_add(self.reg.x);

                let data = self.mem.read_u16(ptr as u16);
                data
            }
            AddressingMode::IndirectY => {
                let base_ptr = self.mem.read(self.reg.pc);

                let data = self.mem.read_u16(base_ptr as u16);
                let deref = data.wrapping_add(self.reg.y as u16);
                deref
            }
            AddressingMode::Indirect => {
                // let hi = self.mem.read(self.reg.pc) as u16;
                // let lo = self.mem.read(self.reg.pc + 1) as u16;
                // let data = (hi << 8) | lo;
                let data = self.mem.read_u16(self.reg.pc);

                data
            }
            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        };

        addr
    }

    // pub fn read_addr_mode(&self, mode: &AddressingMode) -> u16 {
    //     match mode {
    //         AddressingMode::Immediate => self.reg.pc,
    //         AddressingMode::ZeroPage => self.mem.read(self.reg.pc) as u16,
    //         AddressingMode::Absolute => self.mem.read_u16(self.reg.pc),
    //         AddressingMode::ZeroPageX => {
    //             let mem_data = self.mem.read(self.reg.pc);
    //             let data = mem_data.wrapping_add(self.reg.x) as u16;
    //             data
    //         }
    //         AddressingMode::ZeroPageY => {
    //             let mem_data = self.mem.read(self.reg.pc);
    //             let data = mem_data.wrapping_add(self.reg.y) as u16;
    //             data
    //         }
    //         AddressingMode::AbsoluteX => {
    //             let mem_data = self.mem.read_u16(self.reg.pc);
    //             let data = mem_data.wrapping_add(self.reg.x as u16);
    //             data
    //         }
    //         AddressingMode::AbsoluteY => {
    //             let mem_data = self.mem.read_u16(self.reg.pc);
    //             let data = mem_data.wrapping_add(self.reg.y as u16);
    //             data
    //         }
    //         AddressingMode::IndirectX => {
    //             let base_ptr = self.mem.read(self.reg.pc);
    //             let ptr = (base_ptr as u8).wrapping_add(self.reg.x);

    //             let data = self.mem.read_u16(ptr as u16);
    //             data
    //         }
    //         AddressingMode::IndirectY => {
    //             let base_ptr = self.mem.read(self.reg.pc);

    //             let data = self.mem.read_u16(base_ptr as u16);
    //             let deref = data.wrapping_add(self.reg.y as u16);
    //             deref
    //         }
    //         AddressingMode::Indirect => {

    //         }
    //     }
    // }
}

// #[derive(Debug)]
// pub struct CPU {
//     pub register_a: u8,
//     pub status: u8,
//     pub program_counter: u16,
//     pub register_x: u8,
// }

// impl Default for CPU {
//     fn default() -> Self {
//         CPU {
//             register_a: 0,
//             status: 0,
//             program_counter: 0,
//             register_x: 0,
//         }
//     }
// }

// impl CPU {
//     pub fn new() -> Self {
//         CPU {
//             register_a: 0,
//             register_x: 0,
//             status: 0,
//             program_counter: 0,
//         }
//     }

//     fn lda(&mut self, value: u8) {
//         self.register_a = value;
//         self.update_zero_and_negative_flags(self.register_a);
//     }

//     fn tax(&mut self) {
//         self.register_x = self.register_a;
//         self.update_zero_and_negative_flags(self.register_x);
//     }

//     fn update_zero_and_negative_flags(&mut self, result: u8) {
//         if result == 0 {
//             self.status = self.status | 0b0000_0010;
//         } else {
//             self.status = self.status & 0b1111_1101;
//         }

//         if result & 0b1000_0000 != 0 {
//             self.status = self.status | 0b1000_0000;
//         } else {
//             self.status = self.status & 0b0111_1111;
//         }
//     }

//     fn inx(&mut self) {
//         self.register_x = self.register_x.wrapping_add(1);
//         self.update_zero_and_negative_flags(self.register_x);
//     }

//     pub fn interpret(&mut self, program: Vec<u8>) {
//         self.program_counter = 0;

//         loop {
//             let opscode = program[self.program_counter as usize];
//             self.program_counter += 1;

//             match opscode {
//                 0xA9 => {
//                     let param = program[self.program_counter as usize];
//                     self.program_counter += 1;

//                     self.lda(param);
//                 }

//                 0xAA => self.tax(),

//                 0xe8 => self.inx(),

//                 0x00 => return,

//                 _ => todo!(),
//             }
//         }
//     }
// }
