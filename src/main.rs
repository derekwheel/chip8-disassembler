use io::Read;
use std::{env, fs, io};

#[derive(Debug)]
enum Opcode {
    // 0x0
    ClearScreen,
    Return,
    Call,
    // 0x01
    Jump,
    // 0x02
    CallSubroutine,
    // 0x03
    SkipIfEqual,
    // 0x04
    SkipIfNotEqual,
    // 0x05
    SkipIfEqualRegister,
    // 0x06
    SetValue,
    // 0x07
    AddValue,
    // 0x08
    AssignRegister,
    BitwiseOr,
    BitwiseAnd,
    BitwiseXor,
    AddRegister,
    SubtractRegister,
    RightShiftByOne,
    ReverseSubtractRegister,
    LeftShiftByOne,
    // 0x09
    SkipIfNotEqualRegister,
    // 0xA
    SetMemoryToValue,
    // 0xB
    JumpWithOffset,
    // 0xC
    RandAnd,
    // 0xD
    Draw,
    // 0xE
    SkipIfKeyPressed,
    SkipIfKeyNotPressed,
    // 0xF
    SetFromDelayTimer,
    GetKey,
    SetDelayTimer,
    SetSoundTimer,
    AddRegisterToMemory,
    SetChar,
    SetMemoryToBcd,
    RegDump,
    RegLoad,
    Unknown,
}
impl From<[u8; 2]> for Opcode {
    fn from(opcode: [u8; 2]) -> Opcode {
        let [hi, lo] = opcode;
        match hi >> 4u8 {
            0x0 => match lo {
                0xE0 => Opcode::ClearScreen,
                0xEE => Opcode::Return,
                _ => Opcode::Call,
            },
            0x1 => Opcode::Jump,
            0x2 => Opcode::CallSubroutine,
            0x3 => Opcode::SkipIfEqual,
            0x4 => Opcode::SkipIfNotEqual,
            0x5 => Opcode::SkipIfEqualRegister,
            0x6 => Opcode::SetValue,
            0x7 => Opcode::AddValue,
            0x8 => match lo & 0x0F {
                0x0 => Opcode::AssignRegister,
                0x1 => Opcode::BitwiseOr,
                0x2 => Opcode::BitwiseAnd,
                0x3 => Opcode::BitwiseXor,
                0x4 => Opcode::AddRegister,
                0x5 => Opcode::SubtractRegister,
                0x6 => Opcode::RightShiftByOne,
                0x7 => Opcode::ReverseSubtractRegister,
                0xE => Opcode::LeftShiftByOne,
                _ => Opcode::Unknown,
            },
            0x9 => Opcode::SkipIfNotEqualRegister,
            0xA => Opcode::SetMemoryToValue,
            0xB => Opcode::JumpWithOffset,
            0xC => Opcode::RandAnd,
            0xD => Opcode::Draw,
            0xE => match lo {
                0x9E => Opcode::SkipIfKeyPressed,
                0xA1 => Opcode::SkipIfKeyNotPressed,
                _ => Opcode::Unknown,
            },
            0xF => match lo {
                0x07 => Opcode::SetFromDelayTimer,
                0x0A => Opcode::GetKey,
                0x15 => Opcode::SetDelayTimer,
                0x18 => Opcode::SetSoundTimer,
                0x1E => Opcode::AddRegisterToMemory,
                0x29 => Opcode::SetChar,
                0x33 => Opcode::SetMemoryToBcd,
                0x55 => Opcode::RegDump,
                0x65 => Opcode::RegLoad,
                _ => Opcode::Unknown,
            },
            _ => Opcode::Unknown,
        }
    }
}

fn main() -> Result<(), String> {
    let filename = env::args().nth(1).ok_or("Please provide a filename")?;
    let metadata = fs::metadata(&filename).or(Err("Could not read file metadata"))?;
    let mut file = fs::File::open(filename).or(Err("Could not open file"))?;

    let mut buffer: Vec<u8> = vec![0; metadata.len() as usize];
    file.read(&mut buffer).or(Err("Error reading file"))?;

    for i in (0..metadata.len()).step_by(2) {
        let hi = buffer[i as usize];
        let lo = buffer[i as usize + 1];
        println!(
            "{:04x} {:02x} {:02x} {:?}",
            0x200 + i,
            hi,
            lo,
            Opcode::from([hi, lo])
        );
    }
    Ok(())
}
