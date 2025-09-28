use std::env;
use std::fs;

mod run;
use run::run;

mod instruction;
use instruction::Instruction;

mod spliter;
use spliter::split_instructions;

fn main() {
    // step 0: get command line args
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: vm <filename>");
        return;
    }

    // step 1: read the file
    let filename = &args[1];
    let instructions = fs::read_to_string(filename).expect("Failed to read file");

    // step 2: split the file into tokens
    let instruction_arr: Vec<Instruction> = split_instructions(&instructions);
    println!("{:?}", instruction_arr);

    // step 3: run the instructions
    run(&instruction_arr);
}
