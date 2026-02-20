extern crate clap;
extern crate tokio;

use std::fs::File;
use std::io::{BufWriter, Write};

mod FileParser;
mod InstructionParser;

use clap::builder::Str;
use clap::{Parser, command};

#[derive(Parser, Debug)]
#[command(author = "Abonite", version = "0.1.1", about = None, long_about = None)]
struct Args {
    #[arg(short, long)]
    input_file: String,
    #[arg(short, long)]
    output_file: String,
    #[arg(long, default_value_t = 0)]
    code_start_addr: u16,
    #[arg(long, default_value_t = 0x1000)]
    stack_start_addr: u16,
    #[arg(long, default_value_t = 0x2000)]
    data_start_addr: u16,
    #[arg(long, default_value_t = String::from("bin"))]
    compile_mode: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let (asm_instructions, asm_labels) = FileParser::pars_file(args.input_file);
    let bin_code = InstructionParser::pars_instructions(asm_instructions, asm_labels);

    write_bin(args.output_file, bin_code);
}

fn write_bin(output_file_path: String, bin_code: Vec<u8>) {
    let output_file = match File::create(output_file_path) {
        Ok(f) => f,
        Err(e) => {
            println!("{}", e);
            panic!();
        }
    };

    let mut writer = BufWriter::new(output_file);

    let bin_code = bin_code.as_slice();
    match writer.write_all(bin_code) {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e);
            panic!();
        }
    };
}
