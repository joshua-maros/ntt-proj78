use crate::vm_program::VmProgram;
use std::error::Error;

mod parse;
mod translate;
mod vm_program;

fn entry() -> Result<(), Box<dyn Error>> {
    // TODO: Accept directories.
    let base_name = std::env::args().skip(1).next();
    let base_name = base_name.ok_or(format!("Must specify at least one file or folder."))?;
    let mut program = VmProgram::new();
    for filename in std::env::args().skip(1) {
        if !filename.contains(".vm") {
            Err(format!(
                "The file \"{}\" has the wrong extension (expected .vm).",
                filename
            ))?;
        }
        let contents = std::fs::read_to_string(&filename);
        let contents = contents
            .map_err(|err| format!("Failed to open \"{}\", caused by:\n{}", filename, err))?;
        parse::parse(&mut program, &contents[..], &filename[..])?;
    }

    // Optional printing of intermediate representation.
    if cfg!(feature = "dump") {
        println!("Internal Representation:\n{:#?}\n", program);
    }
    let result = translate::translate(program)?;
    if cfg!(feature = "dump") {
        println!("Translated Program:\n{}\n", result);
    }

    let output_name = if base_name.contains("vm") {
        base_name.replace(".vm", ".asm")
    } else {
        format!("{}.asm", base_name)
    };
    let result = std::fs::write(&output_name, result);
    result.map_err(|err| {
        format!(
            "Failed to write result to \"{}\", caused by:\n{:?}",
            output_name, err
        )
    })?;
    println!("Wrote output to \"{}\"", output_name);
    Ok(())
}

fn main() {
    match entry() {
        Ok(_) => {
            println!("Operation completed sucessfully.");
            std::process::exit(0);
        }
        Err(err) => {
            eprintln!("Encountered an error:\n{}", err);
            drop(err);
            std::process::exit(1);
        }
    }
}
