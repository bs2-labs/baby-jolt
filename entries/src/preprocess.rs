use std::path::PathBuf;
use std::str::FromStr;

use ark_bn254::{Fr, G1Projective};
use ark_serialize::CanonicalSerialize;
use jolt_core::host::Program;
use jolt_core::jolt::vm::{rv32i_vm::RV32IJoltVM, Jolt, JoltPreprocessing};

pub fn generate_preprocess(guest: &str, func: &str) -> JoltPreprocessing<Fr, G1Projective> {
    let mut program = Program::new(guest);
    program.set_func(func);

    let elf = format!("./elf/{}-{}/guest", guest, func);

    program.elf = Some(PathBuf::from_str(&elf).unwrap());

    let (bytecode, memory_init) = program.decode();

    let preprocessing: JoltPreprocessing<Fr, G1Projective> =
        RV32IJoltVM::preprocess(bytecode, memory_init, 1 << 20, 1 << 20, 1 << 24);

    println!("==================== Print Preprocess size ====================");

    println!("generators size: {}", get_size(&preprocessing.generators));
    println!(
        "instruction_lookups size: {}",
        get_size(&preprocessing.instruction_lookups)
    );
    println!("bytecode size: {}", get_size(&preprocessing.bytecode));
    println!(
        "read_write_memory size: {}",
        get_size(&preprocessing.read_write_memory)
    );

    println!("==================== under instruction_lookups ====================");
    println!(
        "subtable_to_memory_indices size: {}",
        get_size(&preprocessing.instruction_lookups.subtable_to_memory_indices)
    );
    println!(
        "instruction_to_memory_indices size: {}",
        get_size(
            &preprocessing
                .instruction_lookups
                .instruction_to_memory_indices
        )
    );
    println!(
        "memory_to_subtable_index size: {}",
        get_size(&preprocessing.instruction_lookups.memory_to_subtable_index)
    );
    println!(
        "memory_to_dimension_index size: {}",
        get_size(&preprocessing.instruction_lookups.memory_to_dimension_index)
    );
    println!(
        "materialized_subtables size: {}",
        get_size(&preprocessing.instruction_lookups.materialized_subtables)
    );
    println!(
        "num_memories size: {}",
        get_size(&preprocessing.instruction_lookups.num_memories)
    );

    // println!(
    //     "instruction_lookups: {:?}",
    //     preprocessing.instruction_lookups
    // );

    // println!("bytecode: {:?}", preprocessing.bytecode);

    println!("read_write_memory: {:?}", preprocessing.read_write_memory);

    preprocessing
}

fn get_size<T: CanonicalSerialize>(t: &T) -> usize {
    let mut buffer = Vec::new();
    t.serialize_compressed(&mut buffer).unwrap();
    buffer.len()
}
