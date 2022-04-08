#[derive(Debug)]
pub struct Mem {
    memory: [u8; 0xffff],
}

impl Default for Mem {
    fn default() -> Self {
        Mem {
            memory: [0; 0xffff],
        }
    }
}

impl Mem {
    pub fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn write(&mut self, addr: u16, value: u8) -> u16 {
        self.memory[addr as usize] = value;
        addr
    }

    pub fn load(&mut self, program: Vec<u8>, start_addr: Option<u16>) -> u16 {
        let addr = match start_addr {
            Some(addr) => {
                if addr < 0x8000 {
                    0x8000
                } else {
                    addr
                }
            }
            None => 0x8000,
        };

        let mut count = 0;

        while addr + count < 0xffff && count < program.len() as u16 {
            self.memory[(addr + count) as usize] = program[count as usize];

            count += 1;
        }

        addr
    }

    //reads big endian u16 from memory and returns it as little endian
    pub fn read_u16(&self, addr: u16) -> u16 {
        let hi = self.read(addr) as u16;
        let lo = self.read(addr + 1) as u16;

        (lo << 8) | hi
    }

    //takes in little endian u16 and stores it as big endian u16
    pub fn write_u16(&mut self, addr: u16, value: u16) -> u16 {
        let data = value.to_be_bytes() as [u8; 2];

        let write_hi = self.write(addr, data[1]);
        let write_lo = self.write(addr + 1, data[0]);

        addr
    }
}
