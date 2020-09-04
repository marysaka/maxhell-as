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

fn encode_operand0(out: &mut u64, register: u8) {
    let mut inst = Operand0Data(*out);

    inst.set_operand(register);

    *out = inst.0;
}

fn encode_operand1(out: &mut u64, register: u8) {
    let mut inst = Operand1Data(*out);

    inst.set_operand(register);

    *out = inst.0;
}

fn encode_operand2(out: &mut u64, register: u8) {
    let mut inst = Operand2Data(*out);

    inst.set_operand(register);

    *out = inst.0;
}

fn encode_operand3(out: &mut u64, register: u8) {
    let mut inst = Operand3Data(*out);

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
    encode_operand0(&mut inst.0, register);

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
    encode_operand1(&mut inst.0, register);

    inst.0
}

pub fn encode_al2p(
    source_predicate_register: u8,
    invert_source_predicate: bool,
    destination_predicate_register: u8,
    destionation_register: u8,
    source_register: u8,
    o_flag: bool,
    mode: AtributeLoadMode,
    load_offset: i16,
) -> u64 {
    debug_assert!(destination_predicate_register < 8);

    let mut inst = Al2pInstruction(0);

    encode_opcode(&mut inst.0, Opcode::AL2P);
    encode_source_predicate(
        &mut inst.0,
        source_predicate_register,
        invert_source_predicate,
    );

    encode_operand0(&mut inst.0, destionation_register);
    encode_operand1(&mut inst.0, source_register);

    inst.set_destination_predicate_register(destination_predicate_register);
    inst.set_o_flag(o_flag);
    inst.set_mode(mode);
    inst.set_load_offset(load_offset);

    inst.0
}

pub fn encode_ald(
    source_predicate_register: u8,
    invert_source_predicate: bool,
    destionation_register: u8,
    source_offset_register: u8,
    source_register: u8,
    no_physical_flag: bool,
    o_flag: bool,
    mode: AtributeLoadMode,
    load_offset: i16,
) -> u64 {
    let mut inst = AldInstruction(0);

    encode_opcode(&mut inst.0, Opcode::ALD);
    encode_source_predicate(
        &mut inst.0,
        source_predicate_register,
        invert_source_predicate,
    );
    encode_operand0(&mut inst.0, destionation_register);
    encode_operand1(&mut inst.0, source_offset_register);

    inst.set_source_register(source_register);
    inst.set_no_physical_flag(no_physical_flag);
    inst.set_o_flag(o_flag);
    inst.set_mode(mode);
    inst.set_load_offset(load_offset);

    inst.0
}

pub fn encode_ast(
    source_predicate_register: u8,
    invert_source_predicate: bool,
    destionation_offset_register: u8,
    source_register_a: u8,
    source_register_b: u8,
    no_physical_flag: bool,
    mode: AtributeLoadMode,
    load_offset: i16,
) -> u64 {
    let mut inst = AstInstruction(0);

    encode_opcode(&mut inst.0, Opcode::AST);
    encode_source_predicate(
        &mut inst.0,
        source_predicate_register,
        invert_source_predicate,
    );
    encode_operand0(&mut inst.0, destionation_offset_register);
    encode_operand1(&mut inst.0, source_register_a);

    inst.set_source_registerb(source_register_b);
    inst.set_no_physical_flag(no_physical_flag);
    inst.set_mode(mode);
    inst.set_load_offset(load_offset);

    inst.0
}

pub fn encode_atoms(
    source_predicate_register: u8,
    invert_source_predicate: bool,
    destination_register: u8,
    source_register_a: u8,
    source_register_b: u8,
    register_a_offset: i32,
    type_size: AtomsPrimitiveType,
    operation: AtomsOperation,
) -> u64 {
    let mut inst = AtomsIntruction(0);

    encode_opcode(&mut inst.0, Opcode::ATOMS);
    encode_source_predicate(
        &mut inst.0,
        source_predicate_register,
        invert_source_predicate,
    );
    encode_operand0(&mut inst.0, destination_register);
    encode_operand1(&mut inst.0, source_register_a);
    encode_operand2(&mut inst.0, source_register_b);

    inst.set_type_size(type_size);

    // NOTE: if the value is negative, this part will end up positive.
    // TODO: check on hw that this part will be negative or positive.
    inst.set_register_a_offset_shr_2(register_a_offset >> 2);
    inst.set_operation(operation);

    inst.0
}

pub fn encode_atoms_cas(
    source_predicate_register: u8,
    invert_source_predicate: bool,
    destination_register: u8,
    source_register_a: u8,
    source_register_b: u8,
    register_a_offset: i32,
    type_size: AtomicCasPrimitiveType,
    operation: AtomsCasOperation,
) -> u64 {
    debug_assert!(source_register_b != 0);

    let mut inst = AtomsCasIntruction(0);

    encode_opcode(&mut inst.0, Opcode::ATOMS_CAS);
    encode_source_predicate(
        &mut inst.0,
        source_predicate_register,
        invert_source_predicate,
    );
    encode_operand0(&mut inst.0, destination_register);
    encode_operand1(&mut inst.0, source_register_a);
    encode_operand2(&mut inst.0, source_register_b - 1);

    inst.set_type_size(type_size);

    inst.set_register_a_offset_shr_2(register_a_offset >> 2);
    inst.set_operation(operation);

    inst.0
}

pub fn encode_atom(
    source_predicate_register: u8,
    invert_source_predicate: bool,
    destination_register: u8,
    source_register_a: u8,
    source_register_b: u8,
    register_a_offset: i32,
    type_size: AtomPrimitiveType,
    operation: AtomOperation,
    e_flag: bool,
) -> u64 {
    let mut inst = AtomInstruction(0);

    encode_opcode(&mut inst.0, Opcode::ATOM);
    encode_source_predicate(
        &mut inst.0,
        source_predicate_register,
        invert_source_predicate,
    );
    encode_operand0(&mut inst.0, destination_register);
    encode_operand1(&mut inst.0, source_register_a);
    encode_operand2(&mut inst.0, source_register_b);

    inst.set_type_size(type_size);

    inst.set_register_a_offset(register_a_offset);
    inst.set_operation(operation);
    inst.set_e_flag(e_flag);

    inst.0
}

pub fn encode_atom_cas(
    source_predicate_register: u8,
    invert_source_predicate: bool,
    destination_register: u8,
    source_register_a: u8,
    source_register_b: u8,
    register_a_offset: i32,
    type_size: AtomicCasPrimitiveType,
    e_flag: bool,
) -> u64 {
    let mut inst = AtomCasInstruction(0);

    encode_opcode(&mut inst.0, Opcode::ATOM_CAS);
    encode_source_predicate(
        &mut inst.0,
        source_predicate_register,
        invert_source_predicate,
    );
    encode_operand0(&mut inst.0, destination_register);
    encode_operand1(&mut inst.0, source_register_a);
    encode_operand2(&mut inst.0, source_register_b);

    inst.set_type_size(type_size);

    inst.set_register_a_offset(register_a_offset);
    inst.set_e_flag(e_flag);

    inst.0
}
