use vortex_vm::assembler::load_bytecode_file;
use vortex_vm::run::execute;
use std::env;
use std::fs;
use std::process;
fn print_usage() {
    println!("Vortex VM - Stack-Based Virtual Machine");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("USAGE:");
    println!("    vortex-vm <COMMAND> [OPTIONS]");
    println!();
    println!("COMMANDS:");
    println!("    run <file>     Execute a .vvm or .asv file (.asv files are assembled first)");
    println!("    assemble <input.asv> <output.vvm>    Assemble .asv file to .vvm bytecode");
    println!("    help           Show this help message");
    println!();
    println!("OPTIONS:");
    println!("    --version      Show version information");
    println!("    --help         Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("    vortex-vm run program.vvm");
    println!("    vortex-vm run program.asv    # Assembles first, then runs");
    println!("    vortex-vm assemble program.asv program.vvm");
    println!("    vortex-vm --help");
}

fn print_version() {
    println!("Vortex VM version {}", env!("CARGO_PKG_VERSION"));
}

fn assemble_file_to_path(input_file: &str, output_file: &str) {
    match vortex_vm::assembler::assemble_file(input_file, output_file) {
        Ok(()) => {
            println!("Successfully assembled '{}' to '{}'", input_file, output_file);
        }
        Err(e) => {
            eprintln!("Error: Failed to assemble file: {}", e);
            process::exit(1);
        }
    }
}

fn run_file(filename: &str) {
    let instructions = if filename.ends_with(".vvm") {
        // For .asv files, assemble them first to a temporary .vvm file
        println!("Assembling '{}' to bytecode...", filename);
        let temp_filename = filename.replace(".vvm", "_temp.asv");

        match vortex_vm::assembler::assemble_file(filename, &temp_filename) {
            Ok(()) => {
                // Now load and run the assembled bytecode
                match load_bytecode_file(&temp_filename) {
                    Ok(instructions) => {
                        // Clean up the temporary file
                        let _ = fs::remove_file(&temp_filename);
                        instructions
                    }
                    Err(e) => {
                        let _ = fs::remove_file(&temp_filename);
                        eprintln!("Error: Failed to load assembled bytecode: {}", e);
                        process::exit(1);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: Failed to assemble file '{}': {}", filename, e);
                process::exit(1);
            }
        }
    } else if filename.ends_with(".asv") {
        // For .asv files, load them directly
        match load_bytecode_file(filename) {
            Ok(instructions) => instructions,
            Err(e) => {
                eprintln!("Error: Failed to load bytecode file '{}': {}", filename, e);
                process::exit(1);
            }
        }
    } else {
        eprintln!("Error: Unsupported file extension for '{}'. Supported: .vvm, .asv", filename);
        process::exit(1);
    };

    // step 2: run the instructions
    let mut output_buffer = Vec::new();
    let (stack, _mem) = execute(&instructions, &mut output_buffer);

    // Print any output from Print instructions
    if !output_buffer.is_empty() {
        let output = String::from_utf8_lossy(&output_buffer);
        print!("{}", output);
    }

    println!("Final stack: {:?}", stack);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Handle case with no arguments
    if args.len() == 1 {
        print_usage();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "run" | "--run" | "-r" => {
            if args.len() < 3 {
                eprintln!("Error: 'run' command requires a filename");
                eprintln!("Usage: vortex-vm run <filename>");
                process::exit(1);
            }

            let filename = &args[2];

            run_file(filename);
        }

        "assemble" | "--assemble" | "-a" => {
            if args.len() < 4 {
                eprintln!("Error: 'assemble' command requires input and output filenames");
                eprintln!("Usage: vortex-vm assemble <input.vvm> <output.asv>");
                process::exit(1);
            }

            let input_file = &args[2];
            let output_file = &args[3];

            // Validate input file extension
            if !input_file.ends_with(".vvm") {
                eprintln!("Error: Input file '{}' must have .vvm extension", input_file);
                process::exit(1);
            }

            // Validate output file extension
            if !output_file.ends_with(".asv") {
                eprintln!("Error: Output file '{}' must have .asv extension", output_file);
                process::exit(1);
            }

            assemble_file_to_path(input_file, output_file);
        }

        "help" | "--help" | "-h" => {
            print_usage();
        }

        "--version" | "-v" => {
            print_version();
        }

        _ => {
            eprintln!("Error: Unknown command '{}'. Use 'vortex-vm --help' for usage information.", command);
            process::exit(1);
        }
    }
}

