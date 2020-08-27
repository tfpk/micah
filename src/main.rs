use std::env;

mod utils;

mod mips_parser;
mod memory;
mod runtime;

use mips_parser::*;
use runtime::*;
use memory::*;

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

}

