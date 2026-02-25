use std::{collections::HashMap, num::ParseIntError};
use crate::FileParser::Instr;

#[derive(Debug, Clone)]
struct Constraint {
    target_invalid_reg: Vec<String>,
    source_0_invalid_reg: Vec<String>,
    source_1_invalid_reg: Vec<String>,
    immediate_0_number_max: u32,
    immediate_1_number_max: u32
}

pub fn pars_instructions(instructions: Vec<Instr>, labels: HashMap<String, u64>) -> Vec<u8> {
    let mut result = vec![];
    for line in instructions {
        let mut bin = 0;

        if line.data.starts_with("LOAD8") || line.data.starts_with("load8") {
            bin = match InstPars::pars_load8(line.data.trim_start_matches("LOAD8").trim_start_matches("load8").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>(), labels.clone()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    println!("line: {}", line.line);
                    panic!();
                }
            };
        } else if line.data.starts_with("LOAD16") || line.data.starts_with("load16") {
            bin = match InstPars::pars_load16(line.data.trim_start_matches("LOAD16").trim_start_matches("load16").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>(), labels.clone()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    println!("line: {}", line.line);
                    panic!();
                }
            };
        } else if line.data.starts_with("LOAD32") || line.data.starts_with("load32") {
            bin = match InstPars::pars_load32(line.data.trim_start_matches("LOAD32").trim_start_matches("load32").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>(), labels.clone()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    println!("line: {}", line.line);
                    panic!();
                }
            };
        } else if line.data.starts_with("STORE8") || line.data.starts_with("store8") {
            bin = match InstPars::pars_store8(line.data.trim_start_matches("STORE8").trim_start_matches("store8").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>(), labels.clone()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    println!("line: {}", line.line);
                    panic!();
                }
            };
        } else if line.data.starts_with("STORE16") || line.data.starts_with("store16") {
            bin = match InstPars::pars_store16(line.data.trim_start_matches("STORE16").trim_start_matches("store16").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>(), labels.clone()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    println!("line: {}", line.line);
                    panic!();
                }
            };
        } else if line.data.starts_with("STORE32") || line.data.starts_with("store32") {
            bin = match InstPars::pars_store32(line.data.trim_start_matches("STORE32").trim_start_matches("store32").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>(), labels.clone()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    println!("line: {}", line.line);
                    panic!();
                }
            };
        } else if line.data.starts_with("MOVE") || line.data.starts_with("move") {
            bin = match InstPars::pars_move(line.data.trim_start_matches("MOVE").trim_start_matches("move").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>(), labels.clone()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    println!("line: {}", line.line);
                    panic!();
                }
            };
        } else if line.data.starts_with("ADD") || line.data.starts_with("add") {
            bin = match InstPars::pars_add(line.data.trim_start_matches("ADD").trim_start_matches("add").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>(), labels.clone()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    println!("line: {}", line.line);
                    panic!();
                }
            };
        } else if line.data.starts_with("SUB") || line.data.starts_with("sub") {
            bin = match InstPars::pars_sub(line.data.trim_start_matches("SUB").trim_start_matches("sub").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>(), labels.clone()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    println!("line: {}", line.line);
                    panic!();
                }
            };
        } else if line.data.starts_with("EQ") || line.data.starts_with("eq") {
            bin = match InstPars::pars_eq(line.data.trim_start_matches("EQ").trim_start_matches("eq").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>(), labels.clone()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    println!("line: {}", line.line);
                    panic!();
                }
            };
        } else if line.data.starts_with("JMP") || line.data.starts_with("jmp") {
            bin = match InstPars::pars_jmp(line.data.trim_start_matches("JMP").trim_start_matches("jmp").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>(), labels.clone()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    println!("line: {}", line.line);
                    panic!();
                }
            };
        } else if line.data.starts_with("OJMP") || line.data.starts_with("ojmp") {
            bin = match InstPars::pars_ojmp(line.data.trim_start_matches("OJMP").trim_start_matches("ojmp").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>(), labels.clone()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    println!("line: {}", line.line);
                    panic!();
                }
            };
        } else if line.data.starts_with("ZJMP") || line.data.starts_with("zjmp") {
            bin = match InstPars::pars_zjmp(line.data.trim_start_matches("ZJMP").trim_start_matches("zjmp").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>(), labels.clone()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    println!("line: {}", line.line);
                    panic!();
                }
            };
        }

        if result.len() < line.address as usize {
            let diff = line.address as usize - result.len();
            let mut zeros = vec![0_u8; diff];
            result.append(&mut zeros);
            result.append(&mut bin.to_le_bytes().to_vec());
        } else if result.len() == line.address as usize {
            result.append(&mut bin.to_le_bytes().to_vec());
        } else if result.len() > line.address as usize {
            let data = bin.to_le_bytes();
            for addr in line.address..line.address + 4 {
                if result[addr as usize] == 0 {
                    result[addr as usize] = data[(addr - line.address) as usize];
                } else {
                    println!("Instruction address conflict.");
                    panic!();
                }
            }
        }
    }

    result
}

fn get_register_label(register_name: &str) -> Result<u8, String> {
    return match register_name {
        "PC" | "pc" => Ok(0b101001),
        "ZERO" | "zero" => Ok(0b000000),

        "A0" | "a0" => Ok(1),
        "A1" | "a1" => Ok(2),
        "A2" | "a2" => Ok(3),
        "A3" | "a3" => Ok(4),
        "AR0" | "ar0" => Ok(5),
        "AR1" | "ar1" => Ok(6),
        "AR2" | "ar2" => Ok(7),
        "ASS" | "ass" => Ok(8),
        "ASP" | "asp" => Ok(9),
        "ADS" | "ads" => Ok(10),

        "B0" | "b0" => Ok(11),
        "B1" | "b1" => Ok(12),
        "B2" | "b2" => Ok(13),
        "B3" | "b3" => Ok(14),
        "BR0" | "br0" => Ok(15),
        "BR1" | "br1" => Ok(16),
        "BR2" | "br2" => Ok(17),
        "BSS" | "bss" => Ok(18),
        "BSP" | "bsp" => Ok(19),
        "BDS" | "bds" => Ok(20),

        "C0" | "c0" => Ok(21),
        "C1" | "c1" => Ok(22),
        "C2" | "c2" => Ok(23),
        "C3" | "c3" => Ok(24),
        "CR0" | "cr0" => Ok(25),
        "CR1" | "cr1" => Ok(26),
        "CR2" | "cr2" => Ok(27),
        "CSS" | "css" => Ok(28),
        "CSP" | "csp" => Ok(29),
        "CDS" | "cds" => Ok(20),

        "D0" | "d0" => Ok(31),
        "D1" | "d1" => Ok(32),
        "D2" | "d2" => Ok(33),
        "D3" | "d3" => Ok(34),
        "DR0" | "dr0" => Ok(35),
        "DR1" | "dr1" => Ok(36),
        "DR2" | "dr2" => Ok(37),
        "DSS" | "dss" => Ok(38),
        "DSP" | "dsp" => Ok(39),
        "DDS" | "dds" => Ok(40),

        _ => Err(String::from("Unknown register name."))
    }
}

fn para_immediate_num(immediate_number: &str) -> Result<u32, ParseIntError>{
    if immediate_number.starts_with("0x") {
        let immediate_number = immediate_number.trim_start_matches("0x");
        return u32::from_str_radix(immediate_number, 16);
    } else if immediate_number.starts_with("0o") {
        let immediate_number = immediate_number.trim_start_matches("0o");
        return u32::from_str_radix(immediate_number, 8);
    } else if immediate_number.starts_with("0b") {
        let immediate_number = immediate_number.trim_start_matches("0b");
        return u32::from_str_radix(immediate_number, 2);
    } else {
        return u32::from_str_radix(immediate_number, 10);
    }
}

#[derive(Debug, Clone)]
struct Register {
    name: String,
    label: u8
}

#[derive(Debug, Clone)]
enum Source {
    REG(Register),
    IMM(u32)
}

fn generate_register_ast(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<Vec<Source>, String> {
    let mut result = vec![];

    for item in register_info {
        if item.starts_with('%') {
            let register = item.trim_start_matches("%");
            result.push(Source::REG( Register {
                name: register.to_string(),
                label: match get_register_label(register) {
                    Ok(v) => v,
                    Err(e) => {
                        println!("{}", e);
                        panic!();
                    }
                }
            }));
        } else if item.starts_with('[') {
            let imdn = item.trim_start_matches("[").trim_end_matches("]").trim();
            result.push(Source::IMM(match para_immediate_num(imdn) {
                Ok(v) => v,
                Err(e) => {
                    println!("{}", e);
                    panic!();
                }
            }));
        } else if labels.contains_key(item) {
            result.push(Source::IMM(labels[item] as u32));
        } else {
            return Err(String::from("Invalid arguments."))
        }
    }

    Ok(result)
}

struct InstDiffTypePars {}

impl InstDiffTypePars {
    fn pars_ti(rast: Vec<Source>, constraint: Constraint, op_name: &str) -> Result<u32, String> {
        let target_register = match rast.get(0) {
            Some(s) => {
                match s {
                    Source::REG(r) => r,
                    Source::IMM(i) => return Err(format!("{}: Invalid target register.", op_name))
                }
            },
            None => return Err(format!("{}: missing parameters.", op_name))
        };

        let &immediate_number = match rast.get(1) {
            Some(s) => {
                match s {
                    Source::REG(_) => return Err(format!("{}: Invalid immediate number.", op_name)),
                    Source::IMM(i) => i
                }
            },
            None => return Err(format!("{}: missing parameters.", op_name))
        };

        if constraint.target_invalid_reg.contains(&target_register.name) {
            return Err(format!("{}: target register can't be {}", op_name, target_register.name));
        }

        if immediate_number > constraint.immediate_0_number_max {
            return Err(format!("{}: immediat number is grater then {}", op_name, constraint.immediate_0_number_max));
        }

        let bin_code = ((target_register.label as u32) << 16) | immediate_number;
        return Ok(bin_code);
    }

    fn pars_s(rast: Vec<Source>, constraint: Constraint, op_name: &str) -> Result<u32, String> {
        let target_register = match rast.get(0) {
            Some(s) => {
                match s {
                    Source::REG(r) => r,
                    Source::IMM(i) => return Err(format!("{}: Invalid target register.", op_name))
                }
            },
            None => return Err(format!("{}: missing parameters.", op_name))
        };

        if constraint.target_invalid_reg.contains(&target_register.name) {
            return Err(format!("{}: target register can't be {}", op_name, target_register.name));
        }

        let bin_code = ((target_register.label as u32) << 16);
        return Ok(bin_code);
    }

    fn pars_i(rast: Vec<Source>, constraint: Constraint, op_name: &str) -> Result<u32, String> {
        let &immediate_number = match rast.get(0) {
            Some(s) => {
                match s {
                    Source::REG(_) => return Err(format!("{}: Invalid immediate number.", op_name)),
                    Source::IMM(i) => i
                }
            },
            None => return Err(format!("{}: missing parameters.", op_name))
        };

        if immediate_number > constraint.immediate_0_number_max {
            return Err(format!("immediat number is grater then {}", constraint.immediate_0_number_max));
        }

        return Ok(immediate_number);
    }

    fn pars_ss(rast: Vec<Source>, constraint: Constraint, op_name: &str) -> Result<u32, String> {
        let source_0_register = match rast.get(0) {
            Some(s) => {
                match s {
                    Source::REG(r) => r,
                    Source::IMM(i) => return Err(format!("{}: Invalid source 0 register.", op_name))
                }
            },
            None => return Err(String::from("LOAD8: missing parameters."))
        };

        let source_1_register = match rast.get(1) {
            Some(s) => {
                match s {
                    Source::REG(r) => r,
                    Source::IMM(i) => return Err(format!("{}: Invalid source 1 register.", op_name))
                }
            },
            None => return Err(String::from("LOAD8: missing parameters."))
        };

        if constraint.source_0_invalid_reg.contains(&source_0_register.name) {
            return Err(format!("{}: source 0 register can't be {}", op_name, source_0_register.name));
        }

        if constraint.source_1_invalid_reg.contains(&source_1_register.name) {
            return Err(format!("{}: source 1 register can't be {}", op_name, source_1_register.name));
        }

        let bin_code = ((source_0_register.label as u32) << 10) | ((source_1_register.label as u32) << 4);
        return Ok(bin_code);
    }

    fn pars_ts(rast: Vec<Source>, constraint: Constraint, op_name: &str) -> Result<u32, String> {
        let target_register = match rast.get(0) {
            Some(s) => {
                match s {
                    Source::REG(r) => r,
                    Source::IMM(i) => return Err(format!("{}: Invalid target register.", op_name))
                }
            },
            None => return Err(format!("{}: missing parameters.", op_name))
        };

        let source_register = match rast.get(1) {
            Some(s) => {
                match s {
                    Source::REG(r) => r,
                    Source::IMM(i) => return Err(format!("{}: Invalid source register.", op_name))
                }
            },
            None => return Err(format!("{}: missing parameters.", op_name))
        };

        if constraint.target_invalid_reg.contains(&target_register.name) {
            return Err(format!("target register can't be {}", target_register.name));
        }

        if constraint.source_0_invalid_reg.contains(&source_register.name) {
            return Err(format!("source register can't be {}", source_register.name));
        }

        let bin_code = ((target_register.label as u32) << 16) | ((source_register.label as u32) << 10);
        return Ok(bin_code);
    }

    fn pars_tsi(rast: Vec<Source>, constraint: Constraint, op_name: &str) -> Result<u32, String> {
        let target_register = match rast.get(0) {
            Some(s) => {
                match s {
                    Source::REG(r) => r,
                    Source::IMM(i) => return Err(format!("{}: Invalid target register.", op_name))
                }
            },
            None => return Err(format!("{}: missing parameters.", op_name))
        };

        let source_register = match rast.get(1) {
            Some(s) => {
                match s {
                    Source::REG(r) => r,
                    Source::IMM(i) => return Err(format!("{}: Invalid source register.", op_name))
                }
            },
            None => return Err(format!("{}: missing parameters.", op_name))
        };

        let &immediate_number = match rast.get(2) {
            Some(s) => {
                match s {
                    Source::REG(_) => return Err(format!("{}: Invalid immediate number.", op_name)),
                    Source::IMM(i) => i
                }
            },
            None => return Err(format!("{}: missing parameters.", op_name))
        };

        if constraint.target_invalid_reg.contains(&target_register.name) {
            return Err(format!("target register can't be {}", target_register.name));
        }

        if constraint.source_0_invalid_reg.contains(&source_register.name) {
            return Err(format!("source register can't be {}", source_register.name));
        }

        if immediate_number > constraint.immediate_0_number_max {
            return Err(format!("immediat number is grater then {}", constraint.immediate_0_number_max));
        }

        let bin_code = ((target_register.label as u32) << 16) | ((source_register.label as u32) << 10) | immediate_number;
        return Ok(bin_code);
    }

    fn pars_tss(rast: Vec<Source>, constraint: Constraint, op_name: &str) -> Result<u32, String> {
        let target_register = match rast.get(0) {
            Some(s) => {
                match s {
                    Source::REG(r) => r,
                    Source::IMM(i) => return Err(format!("{}: Invalid target register.", op_name))
                }
            },
            None => return Err(format!("{}: missing parameters.", op_name))
        };

        let source_0_register = match rast.get(1) {
            Some(s) => {
                match s {
                    Source::REG(r) => r,
                    Source::IMM(i) => return Err(format!("{}: Invalid source register.", op_name))
                }
            },
            None => return Err(format!("{}: missing parameters.", op_name))
        };

        let source_1_register = match rast.get(2) {
            Some(s) => {
                match s {
                    Source::REG(r) => r,
                    Source::IMM(i) => return Err(format!("{}: Invalid source register.", op_name))
                }
            },
            None => return Err(format!("{}: missing parameters.", op_name))
        };

        if constraint.target_invalid_reg.contains(&target_register.name) {
            return Err(format!("target register can't be {}", target_register.name));
        }

        if constraint.source_0_invalid_reg.contains(&source_0_register.name) {
            return Err(format!("source 0 register can't be {}", source_0_register.name));
        }

        if constraint.source_1_invalid_reg.contains(&source_1_register.name) {
            return Err(format!("source 1 register can't be {}", source_1_register.name));
        }

        let bin_code = ((target_register.label as u32) << 16) | ((source_0_register.label as u32) << 10) | ((source_1_register.label as u32) << 4);
        return Ok(bin_code);
    }

    fn pars_tii(rast: Vec<Source>, constraint: Constraint, op_name: &str) -> Result<u32, String> {
        let target_register = match rast.get(0) {
            Some(s) => {
                match s {
                    Source::REG(r) => r,
                    Source::IMM(i) => return Err(format!("{}: Invalid target register.", op_name))
                }
            },
            None => return Err(format!("{}: missing parameters.", op_name))
        };

        let &immediate_0_number = match rast.get(1) {
            Some(s) => {
                match s {
                    Source::REG(_) => return Err(format!("{}: Invalid immediate number.", op_name)),
                    Source::IMM(i) => i
                }
            },
            None => return Err(format!("{}: missing parameters.", op_name))
        };

        let &immediate_1_number = match rast.get(2) {
            Some(s) => {
                match s {
                    Source::REG(_) => return Err(format!("{}: Invalid immediate number.", op_name)),
                    Source::IMM(i) => i
                }
            },
            None => return Err(format!("{}: missing parameters.", op_name))
        };

        if constraint.target_invalid_reg.contains(&target_register.name) {
            return Err(format!("target register can't be {}", target_register.name));
        }

        if immediate_0_number > constraint.immediate_0_number_max {
            return Err(format!("immediat number 0 is grater then {}", constraint.immediate_0_number_max));
        }

        if immediate_1_number > constraint.immediate_1_number_max {
            return Err(format!("immediat number 1 is grater then {}", constraint.immediate_1_number_max));
        }

        let bin_code = ((target_register.label as u32) << 16) | (immediate_0_number << 10) | (immediate_1_number << 4);
        return Ok(bin_code);
    }
}

struct InstPars {}

impl InstPars {
    fn pars_load8(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
        let op_name = "LOAD8";

        let rast = match generate_register_ast(register_info, labels) {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let constraint = Constraint {
            target_invalid_reg: vec![String::from("PC"), String::from("ZERO")],
            source_0_invalid_reg: vec![],
            source_1_invalid_reg: vec![],
            immediate_0_number_max: 0xFF,
            immediate_1_number_max: 0
        };

        match rast.len() {
            1 => return Err(format!("{}: Too few arguments!", op_name)),
            2 => {
                match InstDiffTypePars::pars_ti(rast.clone(), constraint.clone(), op_name) {
                    Ok(b) => return Ok((0b0000_0000_01 << 22) | b),
                    Err(e) => {
                        match InstDiffTypePars::pars_s(rast, constraint, op_name) {
                            Ok(b) => return Ok((0b1100_0000_10 << 22) | b),
                            Err(e) => return Err(e)
                        }
                    }
                }
            },
            3 => {
                match InstDiffTypePars::pars_ti(rast, constraint, op_name) {
                    Ok(b) => return Ok((0b0000_0000_10 << 22) | b),
                    Err(e) => return Err(e)
                }
            },
            _ => return Err(format!("{}: Too much arguments!", op_name))
        } 
    }

    fn pars_load16(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
        let op_name = "LOAD16";

        let rast = match generate_register_ast(register_info, labels) {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let constraint = Constraint {
            target_invalid_reg: vec![String::from("PC"), String::from("ZERO")],
            source_0_invalid_reg: vec![],
            source_1_invalid_reg: vec![],
            immediate_0_number_max: 0xFF,
            immediate_1_number_max: 0
        };

        match rast.len() {
            1 => return Err(format!("{}: Too few arguments!", op_name)),
            2 => {
                match InstDiffTypePars::pars_ti(rast.clone(), constraint.clone(), op_name) {
                    Ok(b) => return Ok((0b0000_0000_11 << 22) | b),
                    Err(e1) => {
                        match InstDiffTypePars::pars_s(rast, constraint, op_name) {
                            Ok(b) => return Ok((0b1100_0001_00 << 22) | b),
                            Err(e) => return Err(e1 + "\n" + &e)
                        }
                    }
                }
            },
            3 => {
                match InstDiffTypePars::pars_ti(rast, constraint, op_name) {
                    Ok(b) => return Ok((0b0000_0001_00 << 22) | b),
                    Err(e) => Err(e)
                }
            },
            _ => return Err(format!("{}: Too much arguments!", op_name))
        } 
    }

    fn pars_load32(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
        let op_name = "LOAD32";

        let rast = match generate_register_ast(register_info, labels) {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let constraint = Constraint {
            target_invalid_reg: vec![String::from("PC"), String::from("ZERO")],
            source_0_invalid_reg: vec![],
            source_1_invalid_reg: vec![],
            immediate_0_number_max: 0xFF,
            immediate_1_number_max: 0
        };

        match rast.len() {
            1 => return Err(format!("{}: Too few arguments!", op_name)),
            2 => {
                match InstDiffTypePars::pars_ti(rast.clone(), constraint.clone(), op_name) {
                    Ok(b) => return Ok((0b0000_0001_10 << 22) | b),
                    Err(e1) => {
                        match InstDiffTypePars::pars_s(rast, constraint, op_name) {
                            Ok(b) => return Ok((0b1100_0001_01 << 22) | b),
                            Err(e) => return Err(e1 + "\n" + &e)
                        }
                    }
                }
            },
            3 => {
                match InstDiffTypePars::pars_tss(rast, constraint, op_name) {
                    Ok(b) => return Ok((0b0000_0001_01 << 22) | b),
                    Err(e) => return Err(e)
                }
            },
            _ => return Err(format!("{}: Too much arguments!", op_name))
        } 
    }

    fn pars_store8(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
        let op_name = "STORE8";

        let rast = match generate_register_ast(register_info, labels) {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let constraint = Constraint {
            target_invalid_reg: vec![String::from("PC"), String::from("ZERO")],
            source_0_invalid_reg: vec![],
            source_1_invalid_reg: vec![],
            immediate_0_number_max: 0xFF,
            immediate_1_number_max: 0
        };

        match rast.len() {
            1 => return Err(format!("{}: Too few arguments!", op_name)),
            2 => {
                match InstDiffTypePars::pars_ts(rast, constraint, op_name) {
                    Ok(b) => return Ok((0b0000_0001_10 << 22) | b),
                    Err(e) => return Err(e)
                }
            },
            3 => {
                match InstDiffTypePars::pars_tss(rast, constraint, op_name) {
                    Ok(b) => return Ok((0b0000_0001_10 << 22) | b),
                    Err(e) => return Err(e)
                }
            },
            _ => return Err(format!("{}: Too much arguments!", op_name))
        } 
    }

    fn pars_store16(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
        let op_name = "STORE16";

        let rast = match generate_register_ast(register_info, labels) {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let constraint = Constraint {
            target_invalid_reg: vec![String::from("PC"), String::from("ZERO")],
            source_0_invalid_reg: vec![],
            source_1_invalid_reg: vec![],
            immediate_0_number_max: 0xFF,
            immediate_1_number_max: 0
        };

        match rast.len() {
            1 => return Err(format!("{}: Too few arguments!", op_name)),
            2 => {
                match InstDiffTypePars::pars_ts(rast, constraint, op_name) {
                    Ok(b) => return Ok((0b0000_0001_11 << 22) | b),
                    Err(e) => return Err(e)
                }
            },
            3 => {
                match InstDiffTypePars::pars_tss(rast, constraint, op_name) {
                    Ok(b) => return Ok((0b0000_0001_11 << 22) | b),
                    Err(e) => return Err(e)
                }
            },
            _ => return Err(format!("{}: Too much arguments!", op_name))
        } 
    }

    fn pars_store32(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
        let op_name = "STORE32";

        let rast = match generate_register_ast(register_info, labels) {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let constraint = Constraint {
            target_invalid_reg: vec![String::from("PC"), String::from("ZERO")],
            source_0_invalid_reg: vec![],
            source_1_invalid_reg: vec![],
            immediate_0_number_max: 0xFF,
            immediate_1_number_max: 0
        };

        match rast.len() {
            1 => return Err(format!("{}: Too few arguments!", op_name)),
            2 => {
                match InstDiffTypePars::pars_ts(rast, constraint, op_name) {
                    Ok(b) => return Ok((0b0000_0010_00 << 22) | b),
                    Err(e) => return Err(e)
                }
            },
            3 => {
                match InstDiffTypePars::pars_tss(rast, constraint, op_name) {
                    Ok(b) => return Ok((0b0000_0010_00 << 22) | b),
                    Err(e) => return Err(e)
                }
            },
            _ => return Err(format!("{}: Too much arguments!", op_name))
        } 
    }

    fn pars_move(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
        let op_name = "MOVE";

        let rast = match generate_register_ast(register_info, labels) {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let constraint = Constraint {
            target_invalid_reg: vec![String::from("PC"), String::from("ZERO")],
            source_0_invalid_reg: vec![],
            source_1_invalid_reg: vec![],
            immediate_0_number_max: 0xFF,
            immediate_1_number_max: 0
        };

        match rast.len() {
            1 => return Err(format!("{}: Too few arguments!", op_name)),
            2 => {
                match InstDiffTypePars::pars_ts(rast, constraint, op_name) {
                    Ok(b) => return Ok((0b0000_0010_01 << 22) | b),
                    Err(e) => return Err(e)
                }
            },
            _ => return Err(format!("{}: Too much arguments!", op_name))
        } 
    }

    fn pars_add(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
        let op_name = "ADD";

        let rast = match generate_register_ast(register_info, labels) {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let constraint = Constraint {
            target_invalid_reg: vec![String::from("PC"), String::from("ZERO")],
            source_0_invalid_reg: vec![],
            source_1_invalid_reg: vec![],
            immediate_0_number_max: 0xFF,
            immediate_1_number_max: 0
        };

        match rast.len() {
            1 | 2 => return Err(format!("{}: Too few arguments!", op_name)),
            3 => {
                match InstDiffTypePars::pars_tss(rast.clone(), constraint.clone(), op_name) {
                    Ok(b) => return Ok((0b1000_0000_01 << 22) | b),
                    Err(e1) => {
                        match InstDiffTypePars::pars_tsi(rast, constraint, op_name) {
                            Ok(b) => return Ok((0b1000_0000_00 << 22) | b),
                            Err(e) => return Err(e1 + "\n" + &e)
                        }
                    }
                }
            },
            _ => return Err(format!("{}: Too much arguments!", op_name))
        } 
    }

    fn pars_sub(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
        let op_name = "SUB";

        let rast = match generate_register_ast(register_info, labels) {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let constraint = Constraint {
            target_invalid_reg: vec![String::from("PC"), String::from("ZERO")],
            source_0_invalid_reg: vec![],
            source_1_invalid_reg: vec![],
            immediate_0_number_max: 0xFF,
            immediate_1_number_max: 0
        };

        match rast.len() {
            1 | 2 => return Err(format!("{}: Too few arguments!", op_name)),
            3 => {
                match InstDiffTypePars::pars_tss(rast.clone(), constraint.clone(), op_name) {
                    Ok(b) => return Ok((0b1000_0000_01 << 22) | b),
                    Err(e1) => {
                        match InstDiffTypePars::pars_tsi(rast, constraint, op_name) {
                            Ok(b) => return Ok((0b1000_0000_00 << 22) | b),
                            Err(e) => return Err(e1 + "\n" + &e)
                        }
                    }
                }
            },
            _ => return Err(format!("{}: Too much arguments!", op_name))
        }
    }

    fn pars_eq(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
        let op_name = "EQ";

        let rast = match generate_register_ast(register_info, labels) {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let constraint = Constraint {
            target_invalid_reg: vec![String::from("PC"), String::from("ZERO")],
            source_0_invalid_reg: vec![],
            source_1_invalid_reg: vec![],
            immediate_0_number_max: 0x3FF,
            immediate_1_number_max: 0
        };

        match rast.len() {
            1 | 2 => return Err(format!("{}: Too few arguments!", op_name)),
            3 => match InstDiffTypePars::pars_tss(rast.clone(), constraint.clone(), op_name) {
                Ok(b) => return Ok((0b1001_0000_11 << 22) | b),
                Err(e1) => {
                    match InstDiffTypePars::pars_tsi(rast, constraint, op_name) {
                        Ok(b) => return Ok((0b1001_0000_10 << 22) | b),
                        Err(e) => return Err(e1 + "\n" + &e)
                    }
                }
            }
            _ => return Err(format!("{}: Too much arguments!", op_name))
        } 
    }

    fn pars_jmp(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
        let op_name = "JMP";

        let rast = match generate_register_ast(register_info, labels) {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let constraint = Constraint {
            target_invalid_reg: vec![String::from("PC"), String::from("ZERO")],
            source_0_invalid_reg: vec![],
            source_1_invalid_reg: vec![],
            immediate_0_number_max: 0x3FFFFF,
            immediate_1_number_max: 0
        };

        match rast.len() {
            1 => {
                match InstDiffTypePars::pars_i(rast.clone(), constraint.clone(), op_name) {
                    Ok(b) => return Ok((0b1100_0000_00 << 22) | b),
                    Err(e1) => {
                        match InstDiffTypePars::pars_s(rast, constraint, op_name) {
                            Ok(b) => return Ok((0b1100_0000_01 << 22) | b),
                            Err(e) => return Err(e1 + "\n" + &e)
                        }
                    }
                }
            },
            2 => {
                match InstDiffTypePars::pars_ss(rast, constraint, op_name) {
                    Ok(b) => return Ok((0b1100_0000_01 << 22) | b),
                    Err(e) => return Err(e)
                }
            },
            _ => return Err(format!("{}: Too much arguments!", op_name))
        } 
    }

    fn pars_ojmp(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
        let op_name = "OJMP";

        let rast = match generate_register_ast(register_info, labels) {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let constraint = Constraint {
            target_invalid_reg: vec![String::from("PC"), String::from("ZERO")],
            source_0_invalid_reg: vec![],
            source_1_invalid_reg: vec![],
            immediate_0_number_max: 0x3FFFFF,
            immediate_1_number_max: 0
        };

        match rast.len() {
            1 => return Err(format!("{}: Too few arguments!", op_name)),
            2 => {
                match InstDiffTypePars::pars_ti(rast.clone(), constraint.clone(), op_name) {
                    Ok(b) => return Ok((0b1100_0000_10 << 22) | b),
                    Err(e1) => {
                        match InstDiffTypePars::pars_ts(rast, constraint, op_name) {
                            Ok(b) => return Ok((0b1100_0000_11 << 22) | b),
                            Err(e) => return Err(e1 + "\n" + &e)
                        }
                    }
                }
            },
            _ => return Err(format!("{}: Too much arguments!", op_name))
        } 
    }

    fn pars_zjmp(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
        let op_name = "ZJMP";

        let rast = match generate_register_ast(register_info, labels) {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let constraint = Constraint {
            target_invalid_reg: vec![String::from("PC"), String::from("ZERO")],
            source_0_invalid_reg: vec![],
            source_1_invalid_reg: vec![],
            immediate_0_number_max: 0x3FFFFF,
            immediate_1_number_max: 0
        };

        match rast.len() {
            1 => return Err(format!("{}: Too few arguments!", op_name)),
            2 => {
                match InstDiffTypePars::pars_ti(rast.clone(), constraint.clone(), op_name) {
                    Ok(b) => return Ok((0b1100_0001_00 << 22) | b),
                    Err(e1) => {
                        match InstDiffTypePars::pars_ts(rast, constraint, op_name) {
                            Ok(b) => return Ok((0b1100_0001_01 << 22) | b),
                            Err(e) => return Err(e1 + "\n" + &e)
                        }
                    }
                }
            },
            _ => return Err(format!("{}: Too much arguments!", op_name))
        } 
    }
}