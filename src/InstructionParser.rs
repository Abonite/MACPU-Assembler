use std::{collections::HashMap, num::ParseIntError};
use crate::FileParser::Instr;

pub fn pars_instructions(instructions: Vec<Instr>, labels: HashMap<String, u64>) -> Vec<u8> {
    let mut result = vec![];
    for line in instructions {
        let mut bin = 0;

        if line.data.starts_with("LOAD8") || line.data.starts_with("load8") {
            bin = match pars_load8(line.data.trim_start_matches("LOAD8").trim_start_matches("load8").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    panic!();
                }
            };
        } else if line.data.starts_with("LOAD16") || line.data.starts_with("load16") {
            bin = match pars_load16(line.data.trim_start_matches("LOAD16").trim_start_matches("load16").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    panic!();
                }
            };
        } else if line.data.starts_with("LOAD32") || line.data.starts_with("load32") {
            bin = match pars_load32(line.data.trim_start_matches("LOAD32").trim_start_matches("load32").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    panic!();
                }
            };
        } else if line.data.starts_with("STORE8") || line.data.starts_with("store8") {
            bin = match pars_store8(line.data.trim_start_matches("STORE8").trim_start_matches("store8").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    panic!();
                }
            };
        } else if line.data.starts_with("STORE16") || line.data.starts_with("store16") {
            bin = match pars_store8(line.data.trim_start_matches("STORE16").trim_start_matches("store16").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    panic!();
                }
            };
        }else if line.data.starts_with("STORE32") || line.data.starts_with("store32") {
            bin = match pars_store8(line.data.trim_start_matches("STORE32").trim_start_matches("store32").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>()) {
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

fn para_immediate_num(immediate_number: &str) -> Result<u16, ParseIntError>{
    if immediate_number.starts_with("0x") {
        let immediate_number = immediate_number.trim_start_matches("0x");
        return u16::from_str_radix(immediate_number, 16);
    } else if immediate_number.starts_with("0o") {
        let immediate_number = immediate_number.trim_start_matches("0o");
        return u16::from_str_radix(immediate_number, 8);
    } else if immediate_number.starts_with("0b") {
        let immediate_number = immediate_number.trim_start_matches("0b");
        return u16::from_str_radix(immediate_number, 2);
    } else {
        return u16::from_str_radix(immediate_number, 10);
    }
}

struct Register {
    name: String,
    label: u8
}

enum Source {
    REG(Register),
    IMM(u16)
}

struct RAST {
    target: Register,

    source_0: Option<Source>,
    source_1: Option<Register>
}

fn generate_register_ast(register_info: Vec<&str>) -> Result<RAST, String> {
    let target_register;
    if register_info[0].starts_with('%') {
        target_register = register_info[0].trim_start_matches("%");
    } else {
        return Err(String::from("operation target must be a register."));
    }

    if register_info.len() == 1 {
        return Ok(
            RAST {
                target: Register {
                    name: target_register.to_string(),
                    label: match get_register_label(target_register) {
                        Ok(v) => v,
                        Err(e) => {
                            println!("{}", e);
                            panic!();
                        }
                    }
                },

                source_0: None,
                source_1: None
            }
        );
    } else if register_info.len() == 2 {
        return Ok(
            RAST {
                target: Register {
                    name: target_register.to_string(),
                    label: match get_register_label(target_register) {
                        Ok(v) => v,
                        Err(e) => {
                            println!("{}", e);
                            panic!();
                        }
                    }
                },

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
                        return Err(String::from("Unknown argument."));
                    }
                ),
                source_1: None
            }
        );
    } else if register_info.len() == 3 {
        return Ok(
            RAST {
                target: Register {
                    name: target_register.to_string(),
                    label: match get_register_label(target_register) {
                        Ok(v) => v,
                        Err(e) => {
                            println!("{}", e);
                            panic!();
                        }
                    }
                },

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
                        return Err(String::from("Unknown argument."));
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
}

fn pars_load8(register_info: Vec<&str>) -> Result<u32, String> {
    let rast = match generate_register_ast(register_info) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            panic!();
        }
    };

    match rast.source_0 {
        Some(v) => {
            match v {
                Source::IMM(i) => {
                    if i > 0b11111111 {
                        return Err(String::from("Can't load width then 8bit number to register when use load8."))
                    } else {
                        return Ok(0b0000_0000_0100_0000_0000_0000_0000_0000_u32 + i as u32)
                    }
                },
                Source::REG(r) => {
                    match rast.source_1 {
                        None => {
                            return Ok(0b0000_0000_1000_0000_0000_0000_0000_0000_u32 + r.label as u32);
                        },
                        Some(s) => {
                            return Ok(0b0000_0000_1000_0000_0000_0000_0000_0000_u32 + r.label as u32 + ((s.label as u32) << 6));
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

fn pars_load16(register_info: Vec<&str>) -> Result<u32, String> {
    let rast = match generate_register_ast(register_info) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            panic!();
        }
    };

    match rast.source_0 {
        Some(v) => {
            match v {
                Source::IMM(i) => {
                    return Ok(0b0000_0000_1100_0000_0000_0000_0000_0000_u32 + i as u32)
                },
                Source::REG(r) => {
                    match rast.source_1 {
                        None => {
                            return Ok(0b0000_0001_0000_0000_0000_0000_0000_0000_u32 + r.label as u32);
                        },
                        Some(s) => {
                            return Ok(0b0000_0001_0000_0000_0000_0000_0000_0000_u32 + r.label as u32 + ((s.label as u32) << 6));
                        }
                    }
                }
            }
        },
        None => {
            return Err(String::from("No operation argument when use LOAD16."));
        }
    }
}

fn pars_load32(register_info: Vec<&str>) -> Result<u32, String> {
    let rast = match generate_register_ast(register_info) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            panic!();
        }
    };

    match rast.source_0 {
        Some(v) => {
            match v {
                Source::IMM(i) => {
                    return Err(String::from("LOAD32 operation dose not suppoert use immediate number."))
                },
                Source::REG(r) => {
                    match rast.source_1 {
                        None => {
                            return Ok(0b0000_0001_0100_0000_0000_0000_0000_0000_u32 + r.label as u32);
                        },
                        Some(s) => {
                            return Ok(0b0000_0001_0100_0000_0000_0000_0000_0000_u32 + r.label as u32 + ((s.label as u32) << 6));
                        }
                    }
                }
            }
        },
        None => {
            return Err(String::from("No operation argument when use LOAD32."));
        }
    }
}

fn pars_store8(register_info: Vec<&str>) -> Result<u32, String> {
    let rast = match generate_register_ast(register_info) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            panic!();
        }
    };

    match rast.source_0 {
        Some(v) => {
            match v {
                Source::IMM(i) => {
                    return Err(String::from("STORE8 operation dose not suppoert use immediate number."))
                },
                Source::REG(r) => {
                    match rast.source_1 {
                        None => {
                            return Ok(0b0000_0001_1000_0000_0000_0000_0000_0000_u32 + r.label as u32);
                        },
                        Some(s) => {
                            return Ok(0b0000_0001_1000_0000_0000_0000_0000_0000_u32 + r.label as u32 + ((s.label as u32) << 6));
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

fn pars_store16(register_info: Vec<&str>) -> Result<u32, String> {
    let rast = match generate_register_ast(register_info) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            panic!();
        }
    };

    match rast.source_0 {
        Some(v) => {
            match v {
                Source::IMM(i) => {
                    return Err(String::from("STORE16 operation dose not suppoert use immediate number."))
                },
                Source::REG(r) => {
                    match rast.source_1 {
                        None => {
                            return Ok(0b0000_0001_1100_0000_0000_0000_0000_0000_u32 + r.label as u32);
                        },
                        Some(s) => {
                            return Ok(0b0000_0001_1100_0000_0000_0000_0000_0000_u32 + r.label as u32 + ((s.label as u32) << 6));
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

fn pars_store32(register_info: Vec<&str>) -> Result<u32, String> {
    let rast = match generate_register_ast(register_info) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            panic!();
        }
    };

    match rast.source_0 {
        Some(v) => {
            match v {
                Source::IMM(i) => {
                    return Err(String::from("STORE32 operation dose not suppoert use immediate number."))
                },
                Source::REG(r) => {
                    match rast.source_1 {
                        None => {
                            return Ok(0b0000_0010_0000_0000_0000_0000_0000_0000_u32 + r.label as u32);
                        },
                        Some(s) => {
                            return Ok(0b0000_0010_0000_0000_0000_0000_0000_0000_u32 + r.label as u32 + ((s.label as u32) << 6));
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