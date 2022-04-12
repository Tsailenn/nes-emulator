#[cfg(test)]
mod tests {
    use crate::cpu::mem::Mem;
    use crate::cpu::register::Reg;
    use crate::cpu::CPU;

    fn instantiate_test_mem() -> Mem {
        let program: Vec<u8> = vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00];
        let mut mem = Mem::default();

        mem.load(program, None);

        mem
    }

    // #[test]
    // fn load_mem() {
    //     let mem = instantiate_test_mem();
    //     assert_eq!(mem.memory[0x8000 as usize], 0xa9);
    //     assert_eq!(mem.memory[0x8001 as usize], 0xc0);
    //     assert_eq!(mem.memory[0x8003 as usize], 0xe8);
    // }

    #[test]
    fn read_mem() {
        let mem = instantiate_test_mem();

        let data = mem.read(0x8000);
        assert_eq!(data, 0xa9);
    }

    #[test]
    fn read_write_mem() {
        let mut mem = instantiate_test_mem();

        mem.write_u16(0x8000, 0x1234);
        let data = mem.read_u16(0x8000);
        let data_1 = mem.read(0x8001);

        assert_eq!(data, 0x1234);
        assert_eq!(data_1, 0x12);
        println!("{}", data_1);
    }

    fn instantiate_test_reg() -> Reg {
        Reg::default()
    }

    #[test]
    fn update_zn_flags() {
        let mut reg = instantiate_test_reg();

        reg.update_zero_and_negative_flags(0b10010010);
        assert_eq!(reg.p.bits(), 0b10000000);

        reg.update_zero_and_negative_flags(0);
        assert_eq!(reg.p.bits(), 0b00000010);
    }

    #[test]
    fn reg_reset() {
        let mut reg = instantiate_test_reg();

        reg.reset(0x6969);

        assert_eq!(reg.sp, 0xfd);
        assert_eq!(reg.pc, 0x6969);
    }

    fn instantiate_test_cpu() -> CPU {
        let mut cpu = CPU::default();

        cpu.mem.write_u16(0xfffc, 0x8000);

        cpu
    }

    #[test]
    fn test_reset() {
        let mut cpu = instantiate_test_cpu();

        assert_eq!(cpu.mem.read_u16(0xfffc), 0x8000);

        cpu.reset();

        assert_eq!(cpu.reg.p.bits(), 0b00100100);
        assert_eq!(cpu.reg.pc, 0x8000);
    }
}
