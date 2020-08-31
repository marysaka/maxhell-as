use std::convert::From;

bitfield!{
    pub struct Instruction(u64);
    impl Debug;
    pub data, set_data: 31, 0;
    pub u32, from into Opcode, opcode, set_opcode : 63, 32;
}

bitfield!{
    pub struct Imm16Data(u64);
    impl Debug;
    pub u16, imm16, set_imm16: 35, 20;
}

bitfield!{
    pub struct Imm32Data(u64);
    impl Debug;
    pub u32, imm32, set_imm32: 51, 20;
}

bitfield!{
    pub struct Register0Data(u64);
    impl Debug;
    pub u8, register, set_register: 7, 0;
}

bitfield!{
    pub struct Register8Data(u64);
    impl Debug;
    pub u8, register, set_register: 15, 8;
}

bitfield!{
    pub struct PredicateData(u64);
    impl Debug;
    pub u8, predicate_register, set_predicate_register: 18, 16;
    pub invert_predicate, set_invert_predicate: 19;
}

bitfield!{
    pub struct NopInstruction(u64);
    impl Debug;

    // TODO: Define an enum to handle this
    pub u8, cc_flags, set_cc_flags: 12, 8;
    pub trigger, set_trigger: 13;
    pub u8, predicate_register, set_predicate_register: 18, 16;
    pub invert_predicate, set_invert_predicate: 19;
    pub u16, imm16, set_imm16: 36, 20;
}

bitfield!{
    pub struct SamInstruction(u64);
    impl Debug;
}

bitfield!{
    pub struct RamInstruction(u64);
    impl Debug;
}

bitfield!{
    pub struct RetInstruction(u64);
    impl Debug;

    // TODO: Define an enum to handle this
    pub u8, cc_flags, set_cc_flags: 4, 0;
    pub u8, predicate_register, set_predicate_register: 18, 16;
    pub invert_predicate, set_invert_predicate: 19;
}

bitfield!{
    pub struct ExitInstruction(u64);
    impl Debug;

    // TODO: Define an enum to handle this
    pub u8, cc_flags, set_cc_flags: 4, 0;
    pub u8, keep_refcount, set_keep_refcount: 5;
    pub u8, predicate_register, set_predicate_register: 18, 16;
    pub invert_predicate, set_invert_predicate: 19;
}

bitfield!{
    pub struct GetLMEMBASEInstruction(u64);
    impl Debug;

    pub u8, source_register, set_source_register: 7, 0;
}

bitfield!{
    pub struct SetLMEMBASEInstruction(u64);
    impl Debug;

    pub u8, destination_register, set_destination_register: 15, 8;
}



bitfield!{
    pub struct IdeInstruction(u64);
    impl Debug;

    pub u16, imm16, set_imm16: 36, 20;
    pub disabe, set_disabe: 5;
}

bitfield!{
    pub struct KilInstruction(u64);
    impl Debug;

    pub u8, predicate_register, set_predicate_register: 18, 16;
    pub invert_predicate, set_invert_predicate: 19;
    pub u8, cc_flags, set_cc_flags: 4, 0;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Opcode {
    NOP,
    SAM,
    RAM,
    RET,
    EXIT,
    GETLMEMBASE,
    SETLMEMBASE,
    IDE,
    KIL
}

impl From<u32> for Opcode {
    fn from(value: u32) -> Self {
        match value {
            0x50b00000 => Opcode::NOP,
            0xe3700000 => Opcode::SAM,
            0xe3800000 => Opcode::RAM,
            0xe3200000 => Opcode::RET,
            0xe3000000 => Opcode::EXIT,
            0xe2d00000 => Opcode::GETLMEMBASE,
            0xe2f00000 => Opcode::SETLMEMBASE,
            0xe3900000 => Opcode::IDE,
            0xe3300000 => Opcode::KIL,
            _ => panic!("Invalid Opcode value")
        }
    }
}

impl From<Opcode> for u32 {
    fn from(opcode: Opcode) -> u32 {
        match opcode {
            Opcode::NOP => 0x50b00000,
            Opcode::SAM => 0xe3700000,
            Opcode::RAM => 0xe3800000,
            Opcode::RET => 0xe3200000,
            Opcode::EXIT => 0xe3000000,
            Opcode::GETLMEMBASE => 0xe2d00000,
            Opcode::SETLMEMBASE => 0xe2f00000,
            Opcode::IDE => 0xe3900000,
            Opcode::KIL => 0xe3300000,
        }
    }
}