#[derive(PartialEq, Debug)]
pub enum MemoryError {
    WriteToZero,
    WriteNotIntended
}


#[derive(PartialEq, Debug)]
pub enum RegisterCodes {
    Rzero,
    Rat,
    Rv0, Rv1,
    Ra0, Ra1, Ra2, Ra3,
    Rt0, Rt1, Rt2, Rt3,
    Rt4, Rt5, Rt6, Rt7,
    Rs0, Rs1, Rs2, Rs3,
    Rs4, Rs5, Rs6, Rs7,
    Rt8, Rt9, Rk0, Rk1,
    Rgp, Rsp, Rfp, Rra,

}

use RegisterCodes::*;
const RegisterCodeID: [RegisterCodes; 32] = [
    Rzero,
    Rat,
    Rv0, Rv1,
    Ra0, Ra1, Ra2, Ra3,
    Rt0, Rt1, Rt2, Rt3,
    Rt4, Rt5, Rt6, Rt7,
    Rs0, Rs1, Rs2, Rs3,
    Rs4, Rs5, Rs6, Rs7,
    Rt8, Rt9, Rk0, Rk1,
    Rgp, Rsp, Rfp, Rra

];

pub struct Registers {
    registers: [u32; 32]
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            registers: [0; 32],
        }
    }
    pub fn code_to_register(code: &str) -> Option<RegisterCodes> {
        let code = code.to_owned();
        let code = code.to_ascii_lowercase();

        match code.as_ref() {
            "zero" => Some(Rzero),
            "at" => Some(Rat),
            "v0" => Some(Rv0), "v1" => Some(Rv1),

            "a0" => Some(Ra0), "a1" => Some(Ra1), "a2" => Some(Ra2), "a3" => Some(Ra3),

            "t0" => Some(Rt0), "t1" => Some(Rt1), "t2" => Some(Rt2), "t3" => Some(Rt3),
            "t4" => Some(Rt4), "t5" => Some(Rt5), "t6" => Some(Rt6), "t7" => Some(Rt7),

            "s0" => Some(Rs0), "s1" => Some(Rs1), "s2" => Some(Rs2), "s3" => Some(Rs3),
            "s4" => Some(Rs4), "s5" => Some(Rs5), "s6" => Some(Rs6), "s7" => Some(Rs7),

            "t8" => Some(Rt8), "t9" => Some(Rt9), "k0" => Some(Rk0), "k1" => Some(Rk1),
            _ => None
        }
        
    }
    fn register_to_index(reg: &RegisterCodes) -> usize {
        RegisterCodeID.iter().position(|s| *s == *reg).expect("Provided register not in RegisterCodeID")
    }

    pub fn get_register(&self, reg: &RegisterCodes) -> Result<u32, MemoryError>{
        if *reg == Rzero {
            return Ok(0)
        }
        Ok(self.registers[Registers::register_to_index(reg)])
    }
    
    pub fn set_register(&mut self, reg: &RegisterCodes, val: u32) -> Result<(), MemoryError> {
        if [Rk0, Rk1, Rzero].contains(&reg) {
            return Err(MemoryError::WriteNotIntended)
        }
        self.registers[Registers::register_to_index(reg)] = val;
        Ok(())
    }
}

pub struct Runtime {
    registers: Registers
}

mod tests {
    use super::*;
    
    #[test]
    fn test_get_set_register(){
        let mut runtime = Runtime {registers: Registers::new()};
        let register = Registers::code_to_register("a1").unwrap();
        assert_eq!(register, Ra1);
        runtime.registers.set_register(&register, 57);
        assert_eq!(runtime.registers.get_register(&register).unwrap(), 57)
    }
    #[test]
    fn test_write_not_intended(){
        let mut runtime = Runtime {registers: Registers::new()};
        let register = Registers::code_to_register("zero").unwrap();
        assert_eq!(register, Rzero);
        assert_eq!(runtime.registers.set_register(&register, 57), Err(MemoryError::WriteNotIntended));
    }
}
