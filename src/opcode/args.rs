#[derive(Debug)]
pub struct Address {
    addr: u16
}
impl From<[u8; 2]> for Address {
    fn from(opcode: [u8; 2]) -> Address {
        let [hi, lo] = opcode;
        Address {
            addr: ((hi as u16 & 0x0F) << 8) ^ lo as u16
        }
    }
}


#[derive(Debug)]
pub struct RegisterAndValue {
    register: usize,
    value: u8,
}
impl From<[u8; 2]> for RegisterAndValue {
    fn from(opcode: [u8; 2]) -> RegisterAndValue {
        RegisterAndValue {
            register: (opcode[0] & 0x0F) as usize,
            value: opcode[1],
        }
    }
}

#[derive(Debug)]
pub struct OneRegister {
    register: usize
}
impl From<[u8; 2]> for OneRegister {
    fn from(opcode: [u8; 2]) -> OneRegister {
        OneRegister {
            register: (opcode[0] & 0x0F) as usize
        }
    }
}

#[derive(Debug)]
pub struct TwoRegisters {
    register1: usize,
    register2: usize
}
impl From<[u8; 2]> for TwoRegisters {
    fn from(opcode: [u8; 2]) -> TwoRegisters {
        TwoRegisters {
            register1: (opcode[0] & 0x0F) as usize,
            register2: (opcode[1] >> 4) as usize,
        }
    }
}