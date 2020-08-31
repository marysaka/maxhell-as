#![allow(dead_code, clippy::too_many_arguments)]

use super::definition::*;

fn encode_opcode(out: &mut u64, opcode: Opcode) {
    let mut inst = Instruction(0);

    inst.set_opcode(opcode);

    *out = inst.0;
}

fn encode_imm16(out: &mut u64, value: u16) {
    let mut inst = Imm16Data(*out);

    inst.set_imm16(value);

    *out = inst.0;
}

fn encode_imm32(out: &mut u64, value: u32) {
    let mut inst = Imm32Data(*out);

    inst.set_imm32(value);

    *out = inst.0;
}

fn encode_register0(out: &mut u64, register: u8) {
    let mut inst = Register0Data(*out);

    inst.set_operand(register);

    *out = inst.0;
}

fn encode_operand_a(out: &mut u64, register: u8) {
    let mut inst = OperandAData(*out);

    inst.set_operand(register);

    *out = inst.0;
}

fn encode_operand_b(out: &mut u64, register: u8) {
    let mut inst = OperandBData(*out);

    inst.set_operand(register);

    *out = inst.0;
}

fn encode_operand_c(out: &mut u64, register: u8) {
    let mut inst = OperandCData(*out);

    inst.set_operand(register);

    *out = inst.0;
}

fn encode_source_predicate(out: &mut u64, predicate_register: u8, invert_predicate: bool) {
    debug_assert!(predicate_register < 8);

    let mut inst = SourcePredicateData(*out);

    inst.set_source_predicate_register(predicate_register);
    inst.set_invert_source_predicate(invert_predicate);

    *out = inst.0;
}

pub fn encode_ram() -> u64 {
    let mut inst = RamInstruction(0);

    encode_opcode(&mut inst.0, Opcode::RAM);

    inst.0
}

pub fn encode_sam() -> u64 {
    let mut inst = SamInstruction(0);

    encode_opcode(&mut inst.0, Opcode::SAM);

    inst.0
}

pub fn encode_ret(
    source_predicate_register: u8,
    invert_source_predicate: bool,
    control_code: ControlCode,
) -> u64 {
    let mut inst = RetInstruction(0);

    encode_opcode(&mut inst.0, Opcode::RET);
    encode_source_predicate(
        &mut inst.0,
        source_predicate_register,
        invert_source_predicate,
    );

    inst.set_cc_flags(control_code);

    inst.0
}

pub fn encode_exit(
    source_predicate_register: u8,
    invert_source_predicate: bool,
    control_code: ControlCode,
    keep_refcount: bool,
) -> u64 {
    let mut inst = ExitInstruction(0);

    encode_opcode(&mut inst.0, Opcode::EXIT);
    encode_source_predicate(
        &mut inst.0,
        source_predicate_register,
        invert_source_predicate,
    );

    inst.set_cc_flags(control_code);
    inst.set_keep_refcount(keep_refcount);

    inst.0
}

pub fn encode_nop(
    trigger: bool,
    source_predicate_register: u8,
    invert_source_predicate: bool,
    value: u16,
    control_code: ControlCode,
) -> u64 {
    let mut inst = NopInstruction(0);

    encode_opcode(&mut inst.0, Opcode::NOP);
    encode_imm16(&mut inst.0, value);
    encode_source_predicate(
        &mut inst.0,
        source_predicate_register,
        invert_source_predicate,
    );

    inst.set_trigger(trigger);
    inst.set_cc_flags(control_code);

    inst.0
}

pub fn encode_get_lmembase(register: u8) -> u64 {
    let mut inst = GetLMEMBASEInstruction(0);

    encode_opcode(&mut inst.0, Opcode::GETLMEMBASE);
    encode_register0(&mut inst.0, register);

    inst.0
}

pub fn encode_ide(value: u16, disabe: bool) -> u64 {
    let mut inst = IdeInstruction(0);

    encode_opcode(&mut inst.0, Opcode::IDE);
    encode_imm16(&mut inst.0, value);
    inst.set_disabe(disabe);

    inst.0
}

pub fn encode_kil(
    source_predicate_register: u8,
    invert_source_predicate: bool,
    control_code: ControlCode,
) -> u64 {
    let mut inst = KilInstruction(0);
    encode_opcode(&mut inst.0, Opcode::KIL);
    encode_source_predicate(
        &mut inst.0,
        source_predicate_register,
        invert_source_predicate,
    );

    inst.set_cc_flags(control_code);

    inst.0
}

pub fn encode_set_lmembase(register: u8) -> u64 {
    let mut inst = SetLMEMBASEInstruction(0);

    encode_opcode(&mut inst.0, Opcode::SETLMEMBASE);
    encode_operand_a(&mut inst.0, register);

    inst.0
}

pub fn encode_al2p(
    source_predicate_register: u8,
    invert_source_predicate: bool,
    destination_predicate_register: u8,
    destionation_register: u8,
    source_register: u8,
    o_flag: bool,
    mode: Al2pMode,
    value: i16,
) -> u64 {
    debug_assert!(destination_predicate_register < 8);

    let mut inst = Al2pInstruction(0);

    encode_opcode(&mut inst.0, Opcode::AL2P);
    encode_source_predicate(
        &mut inst.0,
        source_predicate_register,
        invert_source_predicate,
    );
    encode_operand_a(&mut inst.0, source_register);
    encode_register0(&mut inst.0, destionation_register);

    inst.set_destination_predicate_register(destination_predicate_register);
    inst.set_o_flag(o_flag);
    inst.set_mode(mode);
    inst.set_value(value);

    inst.0
}
