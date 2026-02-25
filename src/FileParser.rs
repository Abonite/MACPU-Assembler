use std::fs::File;
use std::io::read_to_string;
use std::collections::HashMap;


pub struct Instr {
    pub data: String,
    pub address: u64,
    pub line: u64
}

pub fn pars_file(file_path: String) -> (Vec<Instr>, HashMap<String, u64>) {
    let asm_file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            println!("{}", e);
            panic!();
        }
    };

    let file_data = match read_to_string(asm_file) {
        Ok(f) => f,
        Err(e) => {
            println!("{}", e);
            panic!();
        }
    };

    let file_in_lines = file_data.split('\n').collect::<Vec<&str>>();

    let file_in_lines = remove_comment(file_in_lines);
    let file_in_lines = remove_blank(file_in_lines);

    let mut instr = vec![];
    let mut label = HashMap::new();
    let mut addr_counter = 0;
    for (line, line_number) in file_in_lines {
        if line.ends_with(':') {
            label.insert(line.trim_end_matches(':').to_string(), addr_counter);
        } else if line.starts_with(".AT") {
            let new_addr = line.trim_start_matches(".AT").trim();
            if new_addr.starts_with("0x") {
                addr_counter = match u64::from_str_radix(new_addr.trim_start_matches("0x"), 16) {
                    Ok(a) => a,
                    Err(e) => {
                        println!("{}", e);
                        panic!();
                    }
                };
            } else if new_addr.starts_with("0b") {
                addr_counter = match u64::from_str_radix(new_addr.trim_start_matches("0b"), 2) {
                    Ok(a) => a,
                    Err(e) => {
                        println!("{}", e);
                        panic!();
                    }
                };
            } else {
                addr_counter = match u64::from_str_radix(new_addr, 10) {
                    Ok(a) => a,
                    Err(e) => {
                        println!("{}", e);
                        panic!();
                    }
                };
            }
        } else {
            instr.push(Instr { data: line, address: addr_counter, line: line_number});
            addr_counter += 4;
        }
    }

    return (instr, label);
}

fn remove_comment(file_in_lines: Vec<&str>) -> Vec<(String, u64)> {
    let mut result = vec![];
    let mut line_number = 0;

    for line in file_in_lines {
        line_number += 1;
        if line.starts_with(";") {
            continue;
        } else {
            let splited_line = line.split(';').collect::<Vec<&str>>();
            result.push((splited_line[0].to_string(), line_number));
        }
    }

    result
}

fn remove_blank(file_in_lines: Vec<(String, u64)>) -> Vec<(String, u64)> {
    let mut result = vec![];

    for (line, line_number) in file_in_lines {
        let line = line.trim().to_string();
        if line.is_empty() {
            continue;
        } else {
            result.push((line.to_string(), line_number));
        }
    }

    result
}