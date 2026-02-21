use std::{collections::HashMap, num::ParseIntError};
use crate::FileParser::Instr;

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
            bin = match pars_load8(line.data.trim_start_matches("LOAD8").trim_start_matches("load8").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>(), labels.clone()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    panic!();
                }
            };
        } else if line.data.starts_with("LOAD16") || line.data.starts_with("load16") {
            bin = match pars_load16(line.data.trim_start_matches("LOAD16").trim_start_matches("load16").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>(), labels.clone()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    panic!();
                }
            };
        } else if line.data.starts_with("LOAD32") || line.data.starts_with("load32") {
            bin = match pars_load32(line.data.trim_start_matches("LOAD32").trim_start_matches("load32").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>(), labels.clone()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    panic!();
                }
            };
        } else if line.data.starts_with("STORE8") || line.data.starts_with("store8") {
            bin = match pars_store8(line.data.trim_start_matches("STORE8").trim_start_matches("store8").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>(), labels.clone()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    panic!();
                }
            };
        } else if line.data.starts_with("STORE16") || line.data.starts_with("store16") {
            bin = match pars_store16(line.data.trim_start_matches("STORE16").trim_start_matches("store16").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>(), labels.clone()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    panic!();
                }
            };
        } else if line.data.starts_with("STORE32") || line.data.starts_with("store32") {
            bin = match pars_store32(line.data.trim_start_matches("STORE32").trim_start_matches("store32").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>(), labels.clone()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    panic!();
                }
            };
        } else if line.data.starts_with("JMP") || line.data.starts_with("jmp") {
            bin = match pars_jmp(line.data.trim_start_matches("JMP").trim_start_matches("jmp").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>(), labels.clone()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
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

struct Register {
    name: String,
    label: u8
}

enum Source {
    REG(Register),
    IMM(u32)
}

struct RAST {
    target: Option<Register>,

    source_0: Option<Source>,
    source_1: Option<Register>
}

fn generate_register_ast(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<RAST, String> {
    if register_info[0].starts_with('%') {
        let target_register = register_info[0].trim_start_matches("%");
        if register_info.len() == 1 {
            return Ok(
                RAST {
                    target: Some(
                        Register {
                            name: target_register.to_string(),
                            label: match get_register_label(target_register) {
                                Ok(v) => v,
                                Err(e) => {
                                    println!("{}", e);
                                    panic!();
                                }
                            }
                        }
                    ),
                    source_0: None,
                    source_1: None
                }
            );
        } else if register_info.len() == 2 {
            return Ok(
                RAST {
                    target: Some(
                        Register {
                            name: target_register.to_string(),
                            label: match get_register_label(target_register) {
                                Ok(v) => v,
                                Err(e) => {
                                    println!("{}", e);
                                    panic!();
                                }
                            }
                        }
                    ),

                    source_0: Some(
                        if register_info[1].starts_with('[') && register_info[1].ends_with(']') {
                            Source::IMM(match para_immediate_num(register_info[1].trim_start_matches('[').trim_end_matches(']')) {
                                Ok(v) => v,
                                Err(e) => {
                                    println!("{}", e);
                                    panic!();
                                }
                            })
                        } else if register_info[1].starts_with('%') {
                            let register = register_info[1].trim_start_matches('%');
                            Source::REG(Register {
                                name: register.to_string(),
                                label: match get_register_label(register) {
                                    Ok(v) => v,
                                    Err(e) => {
                                        println!("{}", e);
                                        panic!();
                                    }
                                }
                            })
                        } else {
                            if labels.contains_key(register_info[0]) {
                                Source::IMM(labels[register_info[0]] as u32)
                            } else {
                                return Err(String::from("Unknown argument."));
                            }
                        }
                    ),
                    source_1: None
                }
            );
        } else if register_info.len() == 3 {
            return Ok(
                RAST {
                    target: Some(
                        Register {
                            name: target_register.to_string(),
                            label: match get_register_label(target_register) {
                                Ok(v) => v,
                                Err(e) => {
                                    println!("{}", e);
                                    panic!();
                                }
                            }
                        }
                    ),

                    source_0: Some(
                        if register_info[1].starts_with('%') {
                            let register = register_info[1].trim_start_matches('%');
                            Source::REG(Register {
                                name: register.to_string(),
                                label: match get_register_label(register) {
                                    Ok(v) => v,
                                    Err(e) => {
                                        println!("{}", e);
                                        panic!();
                                    }
                                }
                            })
                        } else {
                            if labels.contains_key(register_info[0]) {
                                Source::IMM(labels[register_info[0]] as u32)
                            } else {
                                return Err(String::from("Unknown argument."));
                            }
                        }
                    ),
                    source_1: Some(
                        if register_info[2].starts_with('%') {
                            let register = register_info[2].trim_start_matches('%');
                            Register {
                                name: register.to_string(),
                                label: match get_register_label(register) {
                                    Ok(v) => v,
                                    Err(e) => {
                                        println!("{}", e);
                                        panic!();
                                    }
                                }
                            }
                        } else {
                            return Err(String::from("Unknown argument."));
                        }
                    )
                }
            );
        } else {
            return Err(String::from("Too much arguments."));
        }
    } else if register_info[0].starts_with('[') {
        let immediate_number = register_info[0].trim_start_matches("[").trim_end_matches(']');
        return Ok(
            RAST {
                target: None,
                source_0: Some(
                    Source::IMM(match para_immediate_num(immediate_number) {
                        Ok(v) => v,
                        Err(e) => {
                            println!("{}", e);
                            panic!();
                        }
                    })
                ),
                source_1: None
            }
        );
    } else {
        if labels.contains_key(register_info[0]) {
        return Ok(
            RAST {
                target: None,
                source_0: Some(Source::IMM(labels[register_info[0]] as u32)),
                source_1: None
            }
        );
        } else {
            return Err(String::from("operation target must be a register."));
        }
    }
}

struct InstDiffTypePars {}

impl InstDiffTypePars {
    fn pars_ti(target_register: Register, immediate_number: u32, constraint: Constraint) -> Result<u32, String> {
        if constraint.target_invalid_reg.contains(&target_register.name) {
            return Err(format!("target register can't be {}", target_register.name));
        }

        if immediate_number > constraint.immediate_0_number_max {
            return Err(format!("immediat number is grater then {}", constraint.immediate_0_number_max));
        }

        let bin_code = ((target_register.label as u32) << 16) | immediate_number;
        return Ok(bin_code);
    }

    fn pars_i(immediate_number: u32, constraint: Constraint) -> Result<u32, String> {
        if immediate_number > constraint.immediate_0_number_max {
            return Err(format!("immediat number is grater then {}", constraint.immediate_0_number_max));
        }

        return Ok(immediate_number);
    }

    fn pars_ss(source_0_register: Register, source_1_register: Register, constraint: Constraint) -> Result<u32, String> {
        if constraint.source_0_invalid_reg.contains(&source_0_register.name) {
            return Err(format!("source 0 register can't be {}", source_0_register.name));
        }

        if constraint.source_1_invalid_reg.contains(&source_1_register.name) {
            return Err(format!("source 1 register can't be {}", source_1_register.name));
        }

        let bin_code = ((source_0_register.label as u32) << 10) | ((source_1_register.label as u32) << 4);
        return Ok(bin_code);
    }

    fn pars_ts(target_register: Register, source_register: Register, constraint: Constraint) -> Result<u32, String> {
        if constraint.target_invalid_reg.contains(&target_register.name) {
            return Err(format!("target register can't be {}", target_register.name));
        }

        if constraint.source_0_invalid_reg.contains(&source_register.name) {
            return Err(format!("source register can't be {}", source_register.name));
        }

        let bin_code = ((target_register.label as u32) << 16) | ((source_register.label as u32) << 10);
        return Ok(bin_code);
    }

    fn pars_tsi(target_register: Register, source_register: Register, immediate_number: u32, constraint: Constraint) -> Result<u32, String> {
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

    fn pars_tss(target_register: Register, source_0_register: Register, source_1_register: Register, constraint: Constraint) -> Result<u32, String> {
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

    fn pars_tii(target_register: Register, immediate_0_number: u32, immediate_1_number: u32, constraint: Constraint) -> Result<u32, String> {
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

fn pars_load8(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
    let rast = match generate_register_ast(register_info, labels) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            panic!();
        }
    };

    let target_register = match rast.target {
        Some(r) => r,
        None => {
            return Err(String::from("No target register when use LOAD8."))
        }
    };

    let constraint = Constraint {
        target_invalid_reg: vec![String::from("PC"), String::from("ZERO")],
        source_0_invalid_reg: vec![],
        source_1_invalid_reg: vec![],
        immediate_0_number_max: 0xFF,
        immediate_1_number_max: 0
    };

    match rast.source_0 {
        Some(v) => {
            match v {
                Source::IMM(i) => {
                    let opcode: u32 = 0b0000_0000_01 << 22;
                    match InstDiffTypePars::pars_ti(target_register, i, constraint) {
                        Ok(v) => return Ok(opcode | v),
                        Err(e) => return Err(e)
                    };
                },
                Source::REG(r) => {
                    let opcode: u32 = 0b0000_0000_10 << 22;
                    match rast.source_1 {
                        None => {
                            match InstDiffTypePars::pars_tss(target_register, r, Register { name: String::from("ZERO"), label: 0 }, constraint) {
                                Ok(v) => return Ok(opcode | v),
                                Err(e) => return Err(e)
                            };
                        },
                        Some(s) => {
                            match InstDiffTypePars::pars_tss(target_register, r, s, constraint) {
                                Ok(v) => return Ok(opcode | v),
                                Err(e) => return Err(e)
                            };
                        }
                    }
                }
            }
        },
        None => {
            return Err(String::from("No operation argument when use LOAD8."));
        }
    }
}

fn pars_load16(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
    let rast = match generate_register_ast(register_info, labels) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            panic!();
        }
    };

    let target_register = match rast.target {
        Some(r) => r,
        None => {
            return Err(String::from("No target register when use LOAD8."))
        }
    };

    let constraint = Constraint {
        target_invalid_reg: vec![String::from("PC"), String::from("ZERO")],
        source_0_invalid_reg: vec![],
        source_1_invalid_reg: vec![],
        immediate_0_number_max: 0xFF,
        immediate_1_number_max: 0
    };

    match rast.source_0 {
        Some(v) => {
            match v {
                Source::IMM(i) => {
                    let opcode: u32 = 0b0000_0000_11 << 22;
                    match InstDiffTypePars::pars_ti(target_register, i, constraint) {
                        Ok(v) => return Ok(opcode | v),
                        Err(e) => return Err(e)
                    };
                },
                Source::REG(r) => {
                    let opcode: u32 = 0b0000_0001_00 << 22;
                    match rast.source_1 {
                        None => {
                            match InstDiffTypePars::pars_tss(target_register, r, Register { name: String::from("ZERO"), label: 0 }, constraint) {
                                Ok(v) => return Ok(opcode | v),
                                Err(e) => return Err(e)
                            };
                        },
                        Some(s) => {
                            match InstDiffTypePars::pars_tss(target_register, r, s, constraint) {
                                Ok(v) => return Ok(opcode | v),
                                Err(e) => return Err(e)
                            };
                        }
                    }
                }
            }
        },
        None => {
            return Err(String::from("No operation argument when use LOAD8."));
        }
    }
}

fn pars_load32(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
    let rast = match generate_register_ast(register_info, labels) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            panic!();
        }
    };

    let target_register = match rast.target {
        Some(r) => r,
        None => {
            return Err(String::from("No target register when use LOAD8."))
        }
    };

    let constraint = Constraint {
        target_invalid_reg: vec![String::from("PC"), String::from("ZERO")],
        source_0_invalid_reg: vec![],
        source_1_invalid_reg: vec![],
        immediate_0_number_max: 0xFF,
        immediate_1_number_max: 0
    };

    match rast.source_0 {
        Some(v) => {
            match v {
                Source::IMM(i) => {
                    return Err(String::from("LOAD32 Can't load 32bit immediate number"))
                },
                Source::REG(r) => {
                    let opcode: u32 = 0b0000_0001_01 << 22;
                    match rast.source_1 {
                        None => {
                            match InstDiffTypePars::pars_tss(target_register, r, Register { name: String::from("ZERO"), label: 0 }, constraint) {
                                Ok(v) => return Ok(opcode | v),
                                Err(e) => return Err(e)
                            };
                        },
                        Some(s) => {
                            match InstDiffTypePars::pars_tss(target_register, r, s, constraint) {
                                Ok(v) => return Ok(opcode | v),
                                Err(e) => return Err(e)
                            };
                        }
                    }
                }
            }
        },
        None => {
            return Err(String::from("No operation argument when use LOAD8."));
        }
    }
}

fn pars_store8(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
    let rast = match generate_register_ast(register_info, labels) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            panic!();
        }
    };

    let target_register = match rast.target {
        Some(r) => r,
        None => {
            return Err(String::from("No target register when use STORE8."))
        }
    };

    let constraint = Constraint {
        target_invalid_reg: vec![String::from("PC"), String::from("ZERO")],
        source_0_invalid_reg: vec![],
        source_1_invalid_reg: vec![],
        immediate_0_number_max: 0xFF,
        immediate_1_number_max: 0
    };

    match rast.source_0 {
        Some(v) => {
            match v {
                Source::IMM(i) => {
                    return Err(String::from("STORE8 Can't use immediate number as sourece."))
                },
                Source::REG(r) => {
                    let opcode: u32 = 0b0000_0001_10 << 22;
                    match rast.source_1 {
                        None => {
                            match InstDiffTypePars::pars_tss(target_register, r, Register { name: String::from("ZERO"), label: 0 }, constraint) {
                                Ok(v) => return Ok(opcode | v),
                                Err(e) => return Err(e)
                            };
                        },
                        Some(s) => {
                            match InstDiffTypePars::pars_tss(target_register, r, s, constraint) {
                                Ok(v) => return Ok(opcode | v),
                                Err(e) => return Err(e)
                            };
                        }
                    }
                }
            }
        },
        None => {
            return Err(String::from("No operation argument when use STORE8."));
        }
    }
}

fn pars_store16(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
    let rast = match generate_register_ast(register_info, labels) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            panic!();
        }
    };

    let target_register = match rast.target {
        Some(r) => r,
        None => {
            return Err(String::from("No target register when use STORE16."))
        }
    };

    let constraint = Constraint {
        target_invalid_reg: vec![String::from("PC"), String::from("ZERO")],
        source_0_invalid_reg: vec![],
        source_1_invalid_reg: vec![],
        immediate_0_number_max: 0xFF,
        immediate_1_number_max: 0
    };

    match rast.source_0 {
        Some(v) => {
            match v {
                Source::IMM(i) => {
                    return Err(String::from("STORE16 Can't use immediate number as sourece."))
                },
                Source::REG(r) => {
                    let opcode: u32 = 0b0000_0001_11 << 22;
                    match rast.source_1 {
                        None => {
                            match InstDiffTypePars::pars_tss(target_register, r, Register { name: String::from("ZERO"), label: 0 }, constraint) {
                                Ok(v) => return Ok(opcode | v),
                                Err(e) => return Err(e)
                            };
                        },
                        Some(s) => {
                            match InstDiffTypePars::pars_tss(target_register, r, s, constraint) {
                                Ok(v) => return Ok(opcode | v),
                                Err(e) => return Err(e)
                            };
                        }
                    }
                }
            }
        },
        None => {
            return Err(String::from("No operation argument when use STORE16."));
        }
    }
}

fn pars_store32(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
    let rast = match generate_register_ast(register_info, labels) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            panic!();
        }
    };

    let target_register = match rast.target {
        Some(r) => r,
        None => {
            return Err(String::from("No target register when use STORE32."))
        }
    };

    let constraint = Constraint {
        target_invalid_reg: vec![String::from("PC"), String::from("ZERO")],
        source_0_invalid_reg: vec![],
        source_1_invalid_reg: vec![],
        immediate_0_number_max: 0xFF,
        immediate_1_number_max: 0
    };

    match rast.source_0 {
        Some(v) => {
            match v {
                Source::IMM(i) => {
                    return Err(String::from("STORE32 Can't use immediate number as sourece."))
                },
                Source::REG(r) => {
                    let opcode: u32 = 0b0000_0010_00 << 22;
                    match rast.source_1 {
                        None => {
                            match InstDiffTypePars::pars_tss(target_register, r, Register { name: String::from("ZERO"), label: 0 }, constraint) {
                                Ok(v) => return Ok(opcode | v),
                                Err(e) => return Err(e)
                            };
                        },
                        Some(s) => {
                            match InstDiffTypePars::pars_tss(target_register, r, s, constraint) {
                                Ok(v) => return Ok(opcode | v),
                                Err(e) => return Err(e)
                            };
                        }
                    }
                }
            }
        },
        None => {
            return Err(String::from("No operation argument when use STORE32."));
        }
    }
}

fn pars_move(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
    let rast = match generate_register_ast(register_info, labels) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            panic!();
        }
    };

    let target_register = match rast.target {
        Some(r) => r,
        None => {
            return Err(String::from("No target register when use MOVE."))
        }
    };

    let constraint = Constraint {
        target_invalid_reg: vec![String::from("PC"), String::from("ZERO")],
        source_0_invalid_reg: vec![],
        source_1_invalid_reg: vec![],
        immediate_0_number_max: 0xFF,
        immediate_1_number_max: 0
    };

    match rast.source_0 {
        Some(v) => {
            match v {
                Source::IMM(i) => {
                    return Err(String::from("MOVE Can't use immediate number as sourece."))
                },
                Source::REG(r) => {
                    let opcode: u32 = 0b0000_0010_01 << 22;
                    match rast.source_1 {
                        None => {
                            match InstDiffTypePars::pars_tss(target_register, r, Register { name: String::from("ZERO"), label: 0 }, constraint) {
                                Ok(v) => return Ok(opcode | v),
                                Err(e) => return Err(e)
                            };
                        },
                        Some(s) => {
                            return Err(String::from("MOVE Can't use two register as source."))
                        }
                    }
                }
            }
        },
        None => {
            return Err(String::from("No operation argument when use MOVE."));
        }
    }
}

fn pars_jmp(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
    let rast = match generate_register_ast(register_info, labels) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            panic!();
        }
    };

    let constraint = Constraint {
        target_invalid_reg: vec![],
        source_0_invalid_reg: vec![],
        source_1_invalid_reg: vec![],
        immediate_0_number_max: 0xFF,
        immediate_1_number_max: 0
    };

    match rast.source_0 {
        Some(v) => {
            match v {
                Source::IMM(i) => {
                    let opcode = 0b1100_0000_00 << 22;
                    match InstDiffTypePars::pars_i(i, constraint) {
                        Ok(v) => return Ok(opcode | v),
                        Err(e) => return Err(e)
                    }
                },
                Source::REG(r) => {
                    let opcode = 0b1100_0000_01 << 22;
                    match rast.source_1 {
                        None => {
                            match InstDiffTypePars::pars_ss(r, Register { name: String::from("ZERO"), label: 0 }, constraint) {
                                Ok(v) => return Ok(opcode | v),
                                Err(e) => return Err(e)
                            };
                        },
                        Some(s) => {
                            match InstDiffTypePars::pars_ss(r, s, constraint) {
                                Ok(v) => return Ok(opcode | v),
                                Err(e) => return Err(e)
                            };
                        }
                    }
                }
            }
        },
        None => {
            return Err(String::from("No operation argument when use MOVE."));
        }
    }
}