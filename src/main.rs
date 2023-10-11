use std::{path::Path, fs::File, io::{Read, Write}};

use anyhow::{Result,  Error};
use clap::Parser;
use regex::Regex;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    pub file_path_to_start : String,
    pub new_line : String,
    pub extension : String,
    pub line_to_replace : Option<String>,
    pub regex_to_replace : Option<String>,

}

fn get_replace_type(input_to_replace : &Option<String>) -> Option<ReplaceType>{
    if let Some(input) = input_to_replace {

        let re_result = Regex::new(input);

        let result = match re_result{
            Ok(regex) => { ReplaceType::RegexReplacement { regex } },
            Err(_) => ReplaceType::LineReplacement { line: input.to_string() }
        };
        return Some(result);
    
    }
    None
}

fn main() -> Result<()> {
    let args = Args::parse();


    let line_to_replace : &Option<String> = &args.line_to_replace;

    let replace_type = match get_replace_type(line_to_replace) {
        Some(it) => it,
        None => {
            println!("Please supply at least a regex or a line to replace");
            return Ok(())
    }};

    scan_files(&args.file_path_to_start, &replace_type, &args.new_line, &args.extension)?;
    anyhow::Ok(())
}

enum ReplaceType {
    RegexReplacement {regex : Regex},
    LineReplacement {line : String}
}

fn scan_files(path : &str, replacement_type : &ReplaceType, new_line : &str, file_extension : &str) -> Result<()> {

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
            scan_files(this_path, replacement_type, new_line, file_extension)?;
        } else {
            let delimited : Vec<&str> = this_path.split('.').collect();
            let has_file_extension = delimited.len() == 2;
            if has_file_extension {
                if delimited[1] == file_extension {
                    replace_contents(this_path, &replacement_type, &new_line)?;
                }
            }
            
        };
    }
    anyhow::Ok(())
    


}


fn replace_contents(path : &str, replacement_type : &ReplaceType, new_line : &str) -> Result<()>{

   let mut read_file = File::open(path)?;
    let mut contents = String::new();


    let _ = read_file.read_to_string(&mut contents);
    
    let replaced_occurrences :String = match replacement_type {
        ReplaceType::RegexReplacement { regex } => {
            let iter  = regex.find_iter(&contents);
            let replacement :Vec<&str>=  iter.map(|_|new_line).collect();
            replacement.join("")

        } ,
        ReplaceType::LineReplacement { line } => {
           contents.replace(line, &new_line)
        },
    };
    let to_bytes = replaced_occurrences.as_bytes();

    let mut write_file = File::create(path)?;

    write_file.write_all(&to_bytes)?;
    anyhow::Ok(())
    




}

