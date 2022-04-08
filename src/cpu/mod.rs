pub mod mem;
pub mod register;

use mem::Mem;
use register::Reg;

#[derive(Debug, Default)]
pub struct CPU {
    mem: Mem,
    reg: Reg,
}

impl CPU {
    pub fn new() -> Self {
        CPU::default()
    }

    pub fn lda(&mut self, addr: u16) {
        // let param = program[self.program_counter as usize];
        // self.program_counter += 1;
        // self.register_a = param;

        todo!()
    }
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
