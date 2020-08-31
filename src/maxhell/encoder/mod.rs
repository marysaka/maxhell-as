#![allow(dead_code)]

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

    inst.set_register(register);

    *out = inst.0;
}

fn encode_register8(out: &mut u64, register: u8) {
    let mut inst = Register8Data(*out);

    inst.set_register(register);

    *out = inst.0;
}

fn encode_predicate(out: &mut u64, predicate_register: u8, invert_predicate: bool) {
    debug_assert!(predicate_register < 8);

    let mut inst = PredicateData(*out);

    inst.set_predicate_register(predicate_register);
    inst.set_invert_predicate(invert_predicate);

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

pub fn encode_ret(predicate_register: u8, invert_predicate: bool, cc_flags: u8) -> u64 {
    let mut inst = RetInstruction(0);

    encode_opcode(&mut inst.0, Opcode::RET);
    encode_predicate(&mut inst.0, predicate_register, invert_predicate);

    inst.set_cc_flags(cc_flags);

    inst.0
}

pub fn encode_exit(predicate_register: u8, invert_predicate: bool, cc_flags: u8, keep_refcount: bool) -> u64 {
    let mut inst = ExitInstruction(0);

    encode_opcode(&mut inst.0, Opcode::EXIT);
    encode_predicate(&mut inst.0, predicate_register, invert_predicate);

    inst.set_cc_flags(cc_flags);
    inst.set_keep_refcount(keep_refcount);

    inst.0
}

pub fn encode_nop(trigger: bool, predicate_register: u8, invert_predicate: bool, value: u16, cc_flags: u8) -> u64 {
    let mut inst = NopInstruction(0);

    encode_opcode(&mut inst.0, Opcode::NOP);
    encode_imm16(&mut inst.0, value);
    encode_predicate(&mut inst.0, predicate_register, invert_predicate);

    inst.set_trigger(trigger);
    inst.set_cc_flags(cc_flags);

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

pub fn encode_kil(predicate_register: u8, invert_predicate: bool, value: u8) -> u64 {
    debug_assert!(value < 0x20);

    let mut inst = KilInstruction(0);
    encode_opcode(&mut inst.0, Opcode::KIL);
    encode_predicate(&mut inst.0, predicate_register, invert_predicate);

    inst.set_cc_flags(value);

    inst.0
}

pub fn encode_set_lmembase(register: u8) -> u64 {
    let mut inst = SetLMEMBASEInstruction(0);

    encode_opcode(&mut inst.0, Opcode::SETLMEMBASE);
    encode_register8(&mut inst.0, register);

    inst.0
}