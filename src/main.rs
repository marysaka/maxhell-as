use std::env;
use std::fs::OpenOptions;
use std::io::Write;

#[macro_use]
extern crate bitfield;

pub mod maxhell;

use maxhell::definition::*;
use maxhell::encoder;

fn write_shader(file_name: &str, instructions: &[u64]) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_name)?;

    let mut position = 0;

    // TODO: proper scheduling
    let stub_sched_control = 0x1f8000fc0007e0u64;

    for instruction in instructions {
        // We already have put 3 instructions, add scheduling control
        // TODO: handle that in a real way
        if (position % 0x20) == 0 {
            file.write_all(&stub_sched_control.to_le_bytes())?;
            position += 0x8;
        }

        file.write_all(&instruction.to_le_bytes())?;

        position += 0x8;
    }

    while (position % 0x20) != 0 {
        file.write_all(&encoder::encode_nop(false, 7, false, 0, ControlCode::TRUE).to_le_bytes())?;
        position += 0x8;
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let file_name = env::args().nth(1).expect("Cannot find file argument");

    let mut instructions = Vec::new();

    instructions.push(encoder::encode_ram());
    instructions.push(encoder::encode_sam());

    instructions.push(encoder::encode_ret(7, false, ControlCode::TRUE));
    instructions.push(encoder::encode_exit(7, false, ControlCode::TRUE, false));
    instructions.push(encoder::encode_nop(false, 7, false, 0, ControlCode::TRUE));
    instructions.push(encoder::encode_get_lmembase(42));
    instructions.push(encoder::encode_set_lmembase(42));
    instructions.push(encoder::encode_ide(42, false));
    instructions.push(encoder::encode_kil(7, false, ControlCode::TRUE));
    instructions.push(encoder::encode_al2p(
        7,
        false,
        7,
        1,
        42,
        false,
        AtributeLoadMode::M128,
        0x0,
    ));
    instructions.push(encoder::encode_ald(
        7,
        false,
        1,
        0,
        0xFF,
        true,
        false,
        AtributeLoadMode::M128,
        0x0,
    ));
    instructions.push(encoder::encode_ast(
        7,
        false,
        1,
        0,
        0xFE,
        true,
        AtributeLoadMode::M128,
        0x0,
    ));
    instructions.push(encoder::encode_atoms(
        7,
        false,
        4,
        1,
        2,
        0x14,
        AtomsPrimitiveType::S32,
        AtomsOperation::EXCH,
    ));
    instructions.push(encoder::encode_atoms_cas(
        7,
        false,
        4,
        1,
        2,
        0x14,
        AtomicCasPrimitiveType::U32,
        AtomsCasOperation::CAS,
    ));
    instructions.push(encoder::encode_atom(
        7,
        false,
        4,
        1,
        2,
        0x14,
        AtomPrimitiveType::U64,
        AtomOperation::SAFE_ADD,
        false,
    ));
    instructions.push(encoder::encode_atom_cas(
        7,
        false,
        4,
        1,
        2,
        0x14,
        AtomicCasPrimitiveType::U64,
        false,
    ));
    // NOTE: do not change operation as nvdisasm seems to not support it correctly at all.
    instructions.push(encoder::encode_b2r(7, false, 0, 7, B2ROperation::BAR, 0));

    println!("Instruction: 0x{:x}", instructions.last().unwrap());

    write_shader(file_name.as_str(), &instructions)?;

    Ok(())
}
