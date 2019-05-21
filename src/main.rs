use std::env;

mod utils;

mod mips_parser;
mod memory;

use mips_parser::*;
use memory::read_component_list_to_state;

fn main() {
    println!("==================================================");
    println!("                  [[ MICAH ]]");
    println!("    [ MIPS Interpreted Controller And Helper ]");
    println!("==================================================");

    let mut args: Vec<String> = env::args().collect();
    let num_args = args.len();
    if num_args < 2 {
        panic!("MITCH requires files as an argument:\n./mitch <file_name> <file_name...>");
    } 
    args.remove(0);

    let component_list = read_file_to_state(&args[0]).unwrap(); 

    let memory_rep;
    {
        memory_rep = read_component_list_to_state(&mut component_list);
    }

    let (label_hash, code_rep);
    {
        (label_hash, code_rep) = (None, None);
    }
}

