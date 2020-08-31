use std::convert::From;

use crate::enum_with_val;

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
    pub struct OperandAData(u64);
    impl Debug;
    pub u8, register, set_register: 15, 8;
}

bitfield!{
    pub struct OperandBData(u64);
    impl Debug;
    pub u8, register, set_register: 27, 20;
}

bitfield!{
    pub struct OperandCData(u64);
    impl Debug;
    pub u8, register, set_register: 46, 39;
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

    pub u8, from into ControlCode, cc_flags, set_cc_flags: 12, 8;
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

    pub u8, from into ControlCode, cc_flags, set_cc_flags: 4, 0;
    pub u8, predicate_register, set_predicate_register: 18, 16;
    pub invert_predicate, set_invert_predicate: 19;
}

bitfield!{
    pub struct ExitInstruction(u64);
    impl Debug;

    pub u8, from into ControlCode, cc_flags, set_cc_flags: 4, 0;
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

enum_with_val! {
    #[derive(PartialEq, Eq)]
    pub struct ControlCode(u8) {
        False = 0, // never execute
        LessThan = 1,
        Equal = 2,
        LessOrEqual = 3,
        GreaterThan = 4,
        NotEqual = 5,
        GreaterOrEqual = 6,
        IsNumber = 7,
        IsNaN = 8,
        LessThanOrNaN = 9,
        EqualOrNaN = 10,
        LessOrEqualOrNaN = 11,
        GreaterTanOrNaN = 12,
        NotEqualOrNaN = 13,
        GreaterOrEqualOrNaN = 14,
        True = 15, // always execute

        // TODO: figure out what those do
        OFF = 16,
        LO = 17,
        SFF = 18,
        LS = 19,
        HI = 20,
        SFT = 21,
        HS = 22,
        OFT = 23,

        // NOTE: All the instructions around here are related to the CSM patent (urgh)
        // NOTE: FCSM_* variants seems to be the opposite of the CSM one.
        // TODO: figure out anyway, with hardware testing.
        CSM_TA = 24,
        CSM_TR = 25,
        CSM_MX = 26,
        FCSM_TA = 27,
        FCSM_TR = 28,
        FCSM_MX = 29,

        RLE = 30,
        RGT = 31,

        Min = 0,
        Max = 31
    }
}

impl ControlCode {
    pub fn is_valid(value: u8) -> bool {
        value >= ControlCode::Min.0 && value <= ControlCode::Max.0
    }
}

impl From<ControlCode> for u8 {
    fn from(control: ControlCode) -> u8 {
        control.0
    }
}

impl From<u8> for ControlCode {
    fn from(value: u8) -> ControlCode {
        debug_assert!(ControlCode::is_valid(value));
        ControlCode(value)
    }
}