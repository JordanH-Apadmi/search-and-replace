use std::{path::Path, fs::File, io::{Read, Write}};

use anyhow::{Result, Ok};
use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    pub file_path_to_start : String,
    pub line_to_replace : String,
    pub new_line : String,
    pub extension : String
}

fn main() -> Result<()> {
    let args = Args::parse();
    scan_files(&args.file_path_to_start, &args.line_to_replace, &args.new_line, &args.extension)?;
    Ok(())
}


fn scan_files(path : &str, line_to_replace : &str, new_line : &str, file_extension : &str) -> Result<()> {

    let path_inst = Path::new(path);

    let files = path_inst.read_dir()?;

    for file in files {
        let safe = file?.path();

        let this_path = if let Some(v) = safe.to_str() {
            v
        } else {
            continue
        };

        println!("Looking at {}", this_path);
        let path_of_file = Path::new(this_path);
        if path_of_file.is_dir() {
            scan_files(this_path, line_to_replace, new_line, file_extension)?;
        } else {
            let delimited : Vec<&str> = this_path.split('.').collect();
            let has_file_extension = delimited.len() == 2;
            if has_file_extension {
                if delimited[1] == file_extension {
                    replace_contents(this_path, &line_to_replace, &new_line)?;
                }
            }
            
        };
    }
    Ok(())
    


}


fn replace_contents(path : &str, line_to_replace : &str, new_line : &str) -> Result<()>{

   let mut read_file = File::open(path)?;
    let mut contents = String::new();


    let _ = read_file.read_to_string(&mut contents);
    
    let binding = contents.replace(&line_to_replace, &new_line);
    let replaced_occurrences = binding.as_bytes();

    let mut write_file = File::create(path)?;

    write_file.write_all(&replaced_occurrences)?;
    Ok(())
    




}

