extern crate clap;
extern crate tokio;

mod MCAssembler;
mod SFSpliter;
mod Instructions;
mod DIProcessor;
mod IProcessor;

use DIProcessor::DotInstrctionsProcessor;
use Instructions::get_instructions;
use SFSpliter::SourceFileSpliter;
use IProcessor::InstructionProcessor;
use MCAssembler::Assembler;
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

fn main() {
    let args = Args::parse();

    let data = SourceFileSpliter(&args.input_file);

    let (mut dip, data) = DotInstrctionsProcessor::new(data);
    if !dip.lexical_check() {
        panic!("[ERROR] Due to early errors, compiler is stoped");
    }
    if !dip.syntax_check() {
        panic!("[ERROR] Due to early errors, complier is stoped");
    }

    let (a, b, c, d) = dip.getinfo();
    println!("[DEBUG]: set info - {:?}", a);
    println!("[DEBUG]: data info - {:?}", b);
    println!("[DEBUG]: define info - {:?}", c);
    println!("[DEBUG]: data buffer - {:?}", d);

    let mut a = InstructionProcessor::new(data);
    a.syntax_check();
}
