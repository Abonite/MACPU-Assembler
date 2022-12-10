extern crate clap;

mod MCAssembler;

use MCAssembler::Assembler;
use clap::{Parser, command};

#[derive(Parser, Debug)]
#[command(author = "Abonite", version = "0.1.1", about = None, long_about = None)]
struct Args {
    #[arg(short, long)]
    input_file: String,
    #[arg(short, long)]
    output_file: String,
    #[arg(short, long)]
    toml_file: String,
    #[arg(long, default_value_t = 0)]
    code_start_addr: u16,
    #[arg(long, default_value_t = 0x1000)]
    stack_start_addr: u16,
    #[arg(long, default_value_t = 0x2000)]
    data_start_addr: u16,
    #[arg(long, default_value_t = String::from("bin"))]
    compile_mode: StrIng,
}

fn main() {
    let args = Args::parse();

    let mut MCAS = Assembler::new(&args.toml_file.as_str());
    MCAS.set(Some(args.code_start_addr), Some(args.data_start_addr), Some(args.stack_start_addr), args.compile_mode);
    MCAS.generate_bcode(args.input_file.as_str(), &args.output_file.as_str());
}
