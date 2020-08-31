use std::env;
use std::fs::OpenOptions;
use std::io::Write;

#[macro_use]
extern crate bitfield;

pub mod maxhell;

use maxhell::encoder;

fn write_shader(file_name: &str, instructions: &[u64]) -> std::io::Result<()> {
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(file_name)?;

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
        file.write_all(&encoder::encode_nop(false, 7, false, 0, 0xF).to_le_bytes())?;
        position += 0x8;
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let file_name = env::args().nth(1).expect("Cannot find file argument");

    let mut instructions = Vec::new();

    instructions.push(encoder::encode_ram());
    instructions.push(encoder::encode_sam());
    instructions.push(encoder::encode_ret(7, false, 0xF));
    instructions.push(encoder::encode_exit(7, false, 0xF, false));
    instructions.push(encoder::encode_nop(false, 7, false, 0, 0xF));
    instructions.push(encoder::encode_get_lmembase(42));
    instructions.push(encoder::encode_set_lmembase(42));
    instructions.push(encoder::encode_ide(42, false));
    instructions.push(encoder::encode_kil(7, false, 0xF));

    println!("Instruction: 0x{:x}", instructions.last().unwrap());

    write_shader(file_name.as_str(), &instructions)?;

    Ok(())
}
