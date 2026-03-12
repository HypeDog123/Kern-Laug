use std::env;
use std::fs;
use std::process::{Command, exit};
use std::path::Path;

/// Transpile Kern code to Rust code
fn transpile(code: &str) -> String {
    let mut rust = String::new();

    for line in code.lines() {
        let line = line.trim();

        if line.starts_with("fn main") {
            rust.push_str("fn main() {\n");
        } else if line.starts_with("print") {
            let text = line.replace("print", "").trim().to_string();
            rust.push_str(&format!("println!({});\n", text));
        } else if line == "pause" {
            // Only pause if user typed it
            rust.push_str(
                "use std::io::{self, Write};\n\
                print!(\"Press Enter to continue...\");\n\
                io::stdout().flush().unwrap();\n\
                let mut input = String::new();\n\
                io::stdin().read_line(&mut input).ok();\n"
            );
        } else if line == "}" {
            rust.push_str("}\n");
        }
    }

    rust
}

/// Compile a Kern file to .exe
fn build_kern(file: &str) {
    if !Path::new(file).exists() {
        eprintln!("File '{}' not found", file);
        exit(1);
    }

    let code = fs::read_to_string(file).expect("Could not read file");

    let rust_code = transpile(&code);

    let rust_file = file.replace(".kern", ".rs");
    fs::write(&rust_file, rust_code).expect("Could not write Rust file");
    println!("Generated Rust file: {}", rust_file);

    let exe_file = format!("{}.exe", file.replace(".kern", ""));

    let status = Command::new("rustc")
        .arg(&rust_file)
        .arg("-o")
        .arg(&exe_file)
        .status()
        .expect("Failed to run rustc");

    if status.success() {
        println!("Compiled executable: {}", exe_file);
    } else {
        eprintln!("Rust compilation failed");
    }
}

/// Run the Kern program
fn run_kern(file: &str) {
    build_kern(file);

    let exe_file = format!("{}.exe", file.replace(".kern", ""));

    let status = Command::new(&exe_file)
        .status()
        .expect("Failed to run executable");

    if !status.success() {
        eprintln!("Execution failed");
    }
}

/// Remove generated files
fn clean_kern(file: &str) {
    let rust_file = file.replace(".kern", ".rs");
    let exe_file = format!("{}.exe", file.replace(".kern", ""));

    if Path::new(&rust_file).exists() {
        fs::remove_file(&rust_file).unwrap_or_else(|_| eprintln!("Failed to remove {}", rust_file));
    }
    if Path::new(&exe_file).exists() {
        fs::remove_file(&exe_file).unwrap_or_else(|_| eprintln!("Failed to remove {}", exe_file));
    }

    println!("Cleaned generated files");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage:");
        println!("  kernc build <file.kern>   - Compile to .exe");
        println!("  kernc run <file.kern>     - Compile and run");
        println!("  kernc clean <file.kern>   - Remove generated files");
        return;
    }

    match args[1].as_str() {
        "build" => build_kern(&args[2]),
        "run" => run_kern(&args[2]),
        "clean" => clean_kern(&args[2]),
        _ => eprintln!("Unknown command"),
    }
}