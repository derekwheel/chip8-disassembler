mod args;
use args::{Address, OneRegister, RegisterAndValue, TwoRegisters};

#[derive(Debug)]
pub enum Opcode {
    // 0x0
    ClearScreen,
    Return,
    Call(Address),
    // 0x01
    Jump(Address),
    // 0x02
    CallSubroutine(Address),
    // 0x03
    SkipIfEqual(RegisterAndValue),
    // 0x04
    SkipIfNotEqual(RegisterAndValue),
    // 0x05
    SkipIfEqualRegister(TwoRegisters),
    // 0x06
    SetValue(RegisterAndValue),
    // 0x07
    AddValue(RegisterAndValue),
    // 0x08
    AssignRegister(TwoRegisters),
    BitwiseOr(TwoRegisters),
    BitwiseAnd(TwoRegisters),
    BitwiseXor(TwoRegisters),
    AddRegister(TwoRegisters),
    SubtractRegister(TwoRegisters),
    RightShiftByOne(TwoRegisters),
    ReverseSubtractRegister(TwoRegisters),
    LeftShiftByOne(TwoRegisters),
    // 0x09
    SkipIfNotEqualRegister(TwoRegisters),
    // 0xA
    SetMemoryToValue(Address),
    // 0xB
    JumpWithOffset(Address),
    // 0xC
    RandAnd(RegisterAndValue),
    // 0xD
    Draw(TwoRegisters),
    // 0xE
    SkipIfKeyPressed(OneRegister),
    SkipIfKeyNotPressed(OneRegister),
    // 0xF
    SetFromDelayTimer(OneRegister),
    GetKey(OneRegister),
    SetDelayTimer(OneRegister),
    SetSoundTimer(OneRegister),
    AddRegisterToMemory(OneRegister),
    SetChar(OneRegister),
    SetMemoryToBcd(OneRegister),
    RegDump(OneRegister),
    RegLoad(OneRegister),
    Unknown,
}


impl From<[u8; 2]> for Opcode {
    fn from(opcode: [u8; 2]) -> Opcode {
        let [hi, lo] = opcode;
        match hi >> 4u8 {
            0x0 => match lo {
                0xE0 => Opcode::ClearScreen,
                0xEE => Opcode::Return,
                _ => Opcode::Call(Address::from(opcode)),
            },
            0x1 => Opcode::Jump(Address::from(opcode)),
            0x2 => Opcode::CallSubroutine(Address::from(opcode)),
            0x3 => Opcode::SkipIfEqual(RegisterAndValue::from(opcode)),
            0x4 => Opcode::SkipIfNotEqual(RegisterAndValue::from(opcode)),
            0x5 => Opcode::SkipIfEqualRegister(TwoRegisters::from(opcode)),
            0x6 => Opcode::SetValue(RegisterAndValue::from(opcode)),
            0x7 => Opcode::AddValue(RegisterAndValue::from(opcode)),
            0x8 => match lo & 0x0F {
                0x1 => Opcode::BitwiseOr(TwoRegisters::from(opcode)),
                0x0 => Opcode::AssignRegister(TwoRegisters::from(opcode)),
                0x2 => Opcode::BitwiseAnd(TwoRegisters::from(opcode)),
                0x3 => Opcode::BitwiseXor(TwoRegisters::from(opcode)),
                0x4 => Opcode::AddRegister(TwoRegisters::from(opcode)),
                0x5 => Opcode::SubtractRegister(TwoRegisters::from(opcode)),
                0x6 => Opcode::RightShiftByOne(TwoRegisters::from(opcode)),
                0x7 => Opcode::ReverseSubtractRegister(TwoRegisters::from(opcode)),
                0xE => Opcode::LeftShiftByOne(TwoRegisters::from(opcode)),
                _ => Opcode::Unknown,
            },
            0x9 => Opcode::SkipIfNotEqualRegister(TwoRegisters::from(opcode)),
            0xA => Opcode::SetMemoryToValue(Address::from(opcode)),
            0xB => Opcode::JumpWithOffset(Address::from(opcode)),
            0xC => Opcode::RandAnd(RegisterAndValue::from(opcode)),
            0xD => Opcode::Draw(TwoRegisters::from(opcode)),
            0xE => match lo {
                0x9E => Opcode::SkipIfKeyPressed(OneRegister::from(opcode)),
                0xA1 => Opcode::SkipIfKeyNotPressed(OneRegister::from(opcode)),
                _ => Opcode::Unknown,
            },
            0xF => match lo {
                0x07 => Opcode::SetFromDelayTimer(OneRegister::from(opcode)),
                0x0A => Opcode::GetKey(OneRegister::from(opcode)),
                0x15 => Opcode::SetDelayTimer(OneRegister::from(opcode)),
                0x18 => Opcode::SetSoundTimer(OneRegister::from(opcode)),
                0x1E => Opcode::AddRegisterToMemory(OneRegister::from(opcode)),
                0x29 => Opcode::SetChar(OneRegister::from(opcode)),
                0x33 => Opcode::SetMemoryToBcd(OneRegister::from(opcode)),
                0x55 => Opcode::RegDump(OneRegister::from(opcode)),
                0x65 => Opcode::RegLoad(OneRegister::from(opcode)),
                _ => Opcode::Unknown,
            },
            _ => Opcode::Unknown,
        }
    }
}