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

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run()
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.mem.load(program, None);
        self.mem.write_u16(0xfffc, 0x8000);
    }

    pub fn run(&mut self) {
        let mut continue_cycle = true;
        while (continue_cycle) {
            //the pc-pointed instruction
            let word = self.mem.read(self.reg.pc);

            //execute
            continue_cycle = self.execute(word);
        }
    }

    pub fn reset(&mut self) {
        let pc_start_addr = self.mem.read_u16(0xFFFC);
        self.reg.reset(pc_start_addr);
    }

    fn execute(&mut self, word: u8) -> bool {
        //instruction first
        let op_code = OpCode::map(word).unwrap();

        if op_code.mode == AddressingMode::NoneAddressing {
            return false;
        }

        //then data
        self.reg.pc += 1;
        let addr = self.get_operand_address(&op_code.mode);
        //enters next instruction (for the next cycle)
        self.reg.pc += 1;

        match op_code.mnemonic {
            "LDA" => {
                self.lda(addr);
            }
            "BRK" => {}
            "LDX" => {
                self.ldx(addr);
            }
            "LDY" => {
                self.ldy(addr);
            }
            "STA" => {
                self.sta(addr);
            }
            "STX" => {
                self.stx(addr);
            }
            "STY" => {
                self.sty(addr);
            }
            "TXA" => {
                self.txa(addr);
            }
            "TYA" => {
                self.tya(addr);
            }
            "TAX" => {
                self.tax(addr);
            }
            "TAY" => {
                self.tay(addr);
            }
            _ => panic!("{:#?}: {:#?}", op_code.mnemonic, op_code.code),
        };
        true
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

    //load and store
    fn lda(&mut self, addr: u16) {
        let data = self.mem.read(addr);
        self.reg.a = data;
        self.reg.update_zero_and_negative_flags(data);
    }

    fn ldx(&mut self, addr: u16) {
        let data = self.mem.read(addr);
        self.reg.x = data;
        self.reg.update_zero_and_negative_flags(data);
    }

    fn ldy(&mut self, addr: u16) {
        let data = self.mem.read(addr);
        self.reg.y = data;
        self.reg.update_zero_and_negative_flags(data);
    }

    fn sta(&mut self, addr: u16) {
        self.mem.write(addr, self.reg.a);
    }

    fn stx(&mut self, addr: u16) {
        self.mem.write(addr, self.reg.x);
    }

    fn sty(&mut self, addr: u16) {
        self.mem.write(addr, self.reg.y);
    }

    //register transfer
    fn txa(&mut self, addr: u16) {
        self.reg.a = self.reg.x;
        self.reg.update_zero_and_negative_flags(self.reg.a);
    }

    fn tya(&mut self, addr: u16) {
        self.reg.a = self.reg.y;
        self.reg.update_zero_and_negative_flags(self.reg.a);
    }

    fn tax(&mut self, addr: u16) {
        self.reg.x = self.reg.a;
        self.reg.update_zero_and_negative_flags(self.reg.x);
    }

    fn tay(&mut self, addr: u16) {
        self.reg.y = self.reg.a;
        self.reg.update_zero_and_negative_flags(self.reg.y);
    }
}
