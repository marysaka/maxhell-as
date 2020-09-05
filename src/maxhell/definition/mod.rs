use std::convert::From;

use crate::enum_with_val;

bitfield! {
    pub struct Instruction(u64);
    impl Debug;
    pub data, set_data: 31, 0;
    pub u32, from into Opcode, opcode, set_opcode : 63, 32;
}

bitfield! {
    pub struct Imm16Data(u64);
    impl Debug;
    pub u16, imm16, set_imm16: 35, 20;
}

bitfield! {
    pub struct Imm32Data(u64);
    impl Debug;
    pub u32, imm32, set_imm32: 51, 20;
}

bitfield! {
    pub struct Operand0Data(u64);
    impl Debug;
    pub u8, operand, set_operand: 7, 0;
}

bitfield! {
    pub struct Operand1Data(u64);
    impl Debug;
    pub u8, operand, set_operand: 15, 8;
}

bitfield! {
    pub struct Operand2Data(u64);
    impl Debug;
    pub u8, operand, set_operand: 27, 20;
}

bitfield! {
    pub struct Operand3Data(u64);
    impl Debug;
    pub u8, operand, set_operand: 46, 39;
}

bitfield! {
    pub struct SourcePredicateData(u64);
    impl Debug;
    pub u8, source_predicate_register, set_source_predicate_register: 18, 16;
    pub invert_source_predicate, set_invert_source_predicate: 19;
}

bitfield! {
    pub struct NopInstruction(u64);
    impl Debug;

    pub u8, from into ControlCode, cc_flags, set_cc_flags: 12, 8;
    pub trigger, set_trigger: 13;
    pub u8, source_predicate_register, set_source_predicate_register: 18, 16;
    pub invert_source_predicate, set_invert_source_predicate: 19;
    pub u16, imm16, set_imm16: 36, 20;
}

bitfield! {
    pub struct SamInstruction(u64);
    impl Debug;
}

bitfield! {
    pub struct RamInstruction(u64);
    impl Debug;
}

bitfield! {
    pub struct RetInstruction(u64);
    impl Debug;

    pub u8, from into ControlCode, cc_flags, set_cc_flags: 4, 0;
    pub u8, source_predicate_register, set_source_predicate_register: 18, 16;
    pub invert_source_predicate, set_invert_source_predicate: 19;
}

bitfield! {
    pub struct ExitInstruction(u64);
    impl Debug;

    pub u8, from into ControlCode, cc_flags, set_cc_flags: 4, 0;
    pub u8, keep_refcount, set_keep_refcount: 5;
    pub u8, source_predicate_register, set_source_predicate_register: 18, 16;
    pub invert_source_predicate, set_invert_source_predicate: 19;
}

bitfield! {
    pub struct GetLMEMBASEInstruction(u64);
    impl Debug;

    pub u8, source_register, set_source_register: 7, 0;
}

bitfield! {
    pub struct SetLMEMBASEInstruction(u64);
    impl Debug;

    pub u8, destination_register, set_destination_register: 15, 8;
}

bitfield! {
    pub struct IdeInstruction(u64);
    impl Debug;

    pub u16, imm16, set_imm16: 36, 20;
    pub disabe, set_disabe: 5;
}

bitfield! {
    pub struct KilInstruction(u64);
    impl Debug;

    pub u8, from into ControlCode, cc_flags, set_cc_flags: 4, 0;
    pub u8, source_predicate_register, set_source_predicate_register: 18, 16;
    pub invert_source_predicate, set_invert_source_predicate: 19;
}

bitfield! {
    pub struct Al2pInstruction(u64);
    impl Debug;

    pub u8, destination_register, set_destination_register: 7, 0;
    pub u8, source_register, set_source_register: 15, 8;
    pub u8, source_predicate_register, set_source_predicate_register: 18, 16;
    pub invert_source_predicate, set_invert_source_predicate: 19;
    pub i16, load_offset, set_load_offset: 30, 20;
    pub o_flag, set_o_flag: 32;

    pub u8, destination_predicate_register, set_destination_predicate_register: 46, 44;
    pub u8, from into AtributeLoadMode, mode, set_mode: 48, 47;
}

bitfield! {
    pub struct AldInstruction(u64);
    impl Debug;

    pub u8, destination_register, set_destination_register: 7, 0;
    pub u8, source_offset_register, set_source_offset_register: 15, 8;
    pub u8, source_predicate_register, set_source_predicate_register: 18, 16;
    pub invert_source_predicate, set_invert_source_predicate: 19;
    // NOTE: only valid if no_physical_flag is set.
    pub i16, load_offset, set_load_offset: 30, 20;
    pub u8, source_register, set_source_register: 46, 39;
    pub no_physical_flag, set_no_physical_flag: 31;
    pub o_flag, set_o_flag: 32;
    pub u8, from into AtributeLoadMode, mode, set_mode: 48, 47;
}

bitfield! {
    pub struct AstInstruction(u64);
    impl Debug;

    pub u8, destination_register, set_destination_register: 7, 0;
    pub u8, source_offset_register, set_source_offset_register: 15, 8;
    pub u8, source_predicate_register, set_source_predicate_register: 18, 16;
    pub invert_source_predicate, set_invert_source_predicate: 19;
    // NOTE: only valid if no_physical_flag is set.
    pub i16, load_offset, set_load_offset: 30, 20;
    pub u8, source_registerb, set_source_registerb: 46, 39;
    pub no_physical_flag, set_no_physical_flag: 31;
    pub u8, from into AtributeLoadMode, mode, set_mode: 48, 47;
}

bitfield! {
    pub struct AtomsIntruction(u64);
    impl Debug;

    pub u8, destniation_register, set_destination_register: 7, 0;
    pub u8, source_register_a, set_source_register_a: 15, 8;
    pub u8, source_predicate_register, set_source_predicate_register: 18, 16;
    pub invert_source_predicate, set_invert_source_predicate: 19;
    pub u8, source_register_b, set_source_register_b: 27, 20;
    pub u8, from into AtomsPrimitiveType, type_size, set_type_size: 30, 28;
    pub i32, register_a_offset_shr_2, set_register_a_offset_shr_2: 51, 30;
    pub u8, from into AtomsOperation, operation, set_operation: 55, 52;
}

bitfield! {
    pub struct AtomsCasIntruction(u64);
    impl Debug;

    pub u8, destniation_register, set_destination_register: 7, 0;
    pub u8, source_register_a, set_source_register_a: 15, 8;
    pub u8, source_predicate_register, set_source_predicate_register: 18, 16;
    pub invert_source_predicate, set_invert_source_predicate: 19;
    pub u8, source_register_b, set_source_register_b: 27, 20;
    pub i32, register_a_offset_shr_2, set_register_a_offset_shr_2: 51, 30;
    pub u8, from into AtomicCasPrimitiveType, type_size, set_type_size: 52, 52;
    pub u8, from into AtomsCasOperation, operation, set_operation: 54, 53;
}

bitfield! {
    pub struct AtomInstruction(u64);
    impl Debug;

    pub u8, destniation_register, set_destination_register: 7, 0;
    pub u8, source_register_a, set_source_register_a: 15, 8;
    pub u8, source_predicate_register, set_source_predicate_register: 18, 16;
    pub invert_source_predicate, set_invert_source_predicate: 19;
    pub u8, source_register_b, set_source_register_b: 27, 20;
    pub i32, register_a_offset, set_register_a_offset: 47, 28;
    pub u8, from into AtomPrimitiveType, type_size, set_type_size: 51, 49;
    pub u8, from into AtomOperation, operation, set_operation: 55, 52;
    pub e_flag, set_e_flag: 48;
}

bitfield! {
    pub struct AtomCasInstruction(u64);
    impl Debug;

    pub u8, destniation_register, set_destination_register: 7, 0;
    pub u8, source_register_a, set_source_register_a: 15, 8;
    pub u8, source_predicate_register, set_source_predicate_register: 18, 16;
    pub invert_source_predicate, set_invert_source_predicate: 19;
    pub u8, source_register_b, set_source_register_b: 27, 20;
    pub i32, register_a_offset, set_register_a_offset: 47, 28;
    pub e_flag, set_e_flag: 48;
    pub u8, from into AtomicCasPrimitiveType, type_size, set_type_size: 49, 49;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Opcode {
    AL2P,
    ALD,
    AST,
    ATOM,
    ATOM_CAS,
    ATOMS,
    ATOMS_CAS,
    EXIT,
    GETLMEMBASE,
    IDE,
    KIL,
    NOP,
    RAM,
    RET,
    SAM,
    SETLMEMBASE,
}

impl From<u32> for Opcode {
    fn from(value: u32) -> Self {
        match value {
            0xefa00000 => Opcode::AL2P,
            0xefd80000 => Opcode::ALD,
            0xeff00000 => Opcode::AST,
            0xec000000 => Opcode::ATOMS,
            0xed000000 => Opcode::ATOM,
            0xee400000 => Opcode::ATOMS_CAS,
            0xeef00000 => Opcode::ATOM_CAS,
            0xe3000000 => Opcode::EXIT,
            0xe2d00000 => Opcode::GETLMEMBASE,
            0xe3900000 => Opcode::IDE,
            0xe3300000 => Opcode::KIL,
            0x50b00000 => Opcode::NOP,
            0xe3800000 => Opcode::RAM,
            0xe3200000 => Opcode::RET,
            0xe3700000 => Opcode::SAM,
            0xe2f00000 => Opcode::SETLMEMBASE,
            _ => panic!("Invalid Opcode value"),
        }
    }
}

impl From<Opcode> for u32 {
    fn from(opcode: Opcode) -> u32 {
        match opcode {
            Opcode::AL2P => 0xefa00000,
            Opcode::ALD => 0xefd80000,
            Opcode::AST => 0xeff00000,
            Opcode::ATOMS => 0xec000000,
            Opcode::ATOM => 0xed000000,
            Opcode::ATOMS_CAS => 0xee400000,
            Opcode::ATOM_CAS => 0xeef00000,
            Opcode::EXIT => 0xe3000000,
            Opcode::GETLMEMBASE => 0xe2d00000,
            Opcode::IDE => 0xe3900000,
            Opcode::KIL => 0xe3300000,
            Opcode::NOP => 0x50b00000,
            Opcode::SAM => 0xe3700000,
            Opcode::RAM => 0xe3800000,
            Opcode::RET => 0xe3200000,
            Opcode::SETLMEMBASE => 0xe2f00000,
        }
    }
}

enum_with_val! {
    #[derive(PartialEq, Eq)]
    pub struct ControlCode(u8) {
        FALSE = 0, // never execute
        LESS_THAN = 1,
        EQUAL = 2,
        LESS_OR_EQUAL = 3,
        GREATER_THAN = 4,
        NOT_EQUAL = 5,
        GREATER_OR_EQUAL = 6,
        IS_NUMBER = 7,
        IS_NAN = 8,
        LESS_THAN_OR_NAN = 9,
        EQUAL_OR_NAN = 10,
        LESS_OR_EQUAL_OR_NAN = 11,
        GREATER_THAN_OR_NAN = 12,
        NOT_EQUAL_OR_NAN = 13,
        GREATER_OR_EQUAL_OR_NAN = 14,
        TRUE = 15, // always execute

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

        MIN = 0,
        MAX = 31
    }
}

enum_with_val! {
    #[derive(PartialEq, Eq)]
    pub struct AtributeLoadMode(u8) {
        M32 = 0,
        M64 = 1,
        M96 = 2,
        M128 = 3
    }
}

enum_with_val! {
    #[derive(PartialEq, Eq)]
    pub struct AtomsPrimitiveType(u8) {
        U32 = 0,
        S32 = 1,
        U64 = 2,
        S64 = 3,
    }
}

enum_with_val! {
    #[derive(PartialEq, Eq)]
    pub struct AtomPrimitiveType(u8) {
        U32 = 0,
        S32 = 1,
        U64 = 2,
        F32 = 3,
    }
}

enum_with_val! {
    #[derive(PartialEq, Eq)]
    pub struct AtomsOperation(u8) {
        ADD = 0,
        MIN = 1,
        MAX = 2,
        INC = 3,
        DEC = 4,
        AND = 5,
        OR = 6,
        XOR = 7,
        EXCH = 8,
    }
}

enum_with_val! {
    #[derive(PartialEq, Eq)]
    pub struct AtomOperation(u8) {
        ADD = 0,
        MIN = 1,
        MAX = 2,
        INC = 3,
        DEC = 4,
        AND = 5,
        OR = 6,
        XOR = 7,
        EXCH = 8,

        // Only for Opcode::ATOM.
        SAFE_ADD = 10
    }
}

enum_with_val! {
    #[derive(PartialEq, Eq)]
    pub struct AtomicCasPrimitiveType(u8) {
        U32 = 0,
        U64 = 1,
    }
}

enum_with_val! {
    #[derive(PartialEq, Eq)]
    pub struct AtomsCasOperation(u8) {
        CAST = 0,
        CAST_SPIN = 1,
        CAS = 2
    }
}
