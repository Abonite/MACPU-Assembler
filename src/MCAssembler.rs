extern crate taplo;

use std::collections::HashMap;
use std::io::Write;
use std::{fs::File, io::Read};
use taplo::parser::parse;
use taplo::dom::node::Node;
use taplo::dom::Error;
use taplo::dom::error;

pub struct Assembler {
    toml: Node,
    code_start_address: u16,
    stack_start_address: u16,
    data_start_address: u16,
    mode: String
}

impl Assembler {
    pub fn new(instructions_file_path: &str) -> Assembler {
        let mut instructions_file = File::open(instructions_file_path).unwrap();
        let mut toml = String::new();
        instructions_file.read_to_string(&mut toml).unwrap();

        let result = parse(toml.as_str());
        let result = result.into_dom();

        Assembler {
            toml: result,
            code_start_address: 0x0,
            data_start_address: 0x1000,
            stack_start_address: 0x2000,
            mode: String::new()
        }
    }

    pub fn set(&mut self, csa: Option<u16>, dsa: Option<u16>, ssa: Option<u16>, mode: String) {
        self.code_start_address = match csa {
            None => 0x0,
            Some(addr) => addr
        };
        self.data_start_address = match csa {
            None => 0x1000,
            Some(addr) => addr
        };
        self.stack_start_address = match csa {
            None => 0x2000,
            Some(addr) => addr
        };
        self.mode = mode;
    }

    pub fn generate_bcode(&mut self, asm_file_path: &str, output_file_path: &str) {
        let mut valid_code = vec![];
        let mut bcode = vec![];

        let mut define_table: HashMap<&str, &str> = HashMap::new();
        let mut datas = vec![];
        let mut data_table: HashMap<&str, u16> = HashMap::new();
        let mut data_ptr: u16 = 0;

        let mut asm_file = File::open(asm_file_path).unwrap();
        let mut asm = String::new();
        asm_file.read_to_string(&mut asm).unwrap();

        let asm = asm.trim().lines();
        for (line_num, line) in asm.enumerate() {
            let line_num = line_num + 1;
            let line = line.split(";").collect::<Vec<_>>()[0];
            let line = line.trim();
            if line.eq("") || line.eq("\t") || line.eq("\t") {
                continue;
            } else if line.starts_with(".") {
                let line = line.split(" ").collect::<Vec<_>>();
                if line[0] == ".DEFINE" {
                    if line[1].chars().collect::<Vec<_>>()[0].is_ascii_digit() {
                        panic!("[Syntex Error] line{}:", line_num);
                    } else {
                        define_table.insert(line[1].trim(), line[2].trim());
                    }
                } else if line[0] == ".STRING" {
                    if line[1].chars().collect::<Vec<_>>()[0].is_ascii_digit() {
                        panic!("[Syntex Error] line{}:", line_num);
                    } else {
                        let string = line[2..line.len()].join(" ");
                        let mut string = String::from(string.trim_start_matches("\"").trim_end_matches("\""));
                        if (string.len() & 1) == 1 {
                            string += &String::from("\0\0");
                        } else {
                            string += &String::from("\0")
                        }
                        data_table.insert(line[1], data_ptr);
                        let mut data_d: u16 = 0;
                        for (index, c) in string.chars().enumerate() {
                            if index != 0 && index % 2 == 0 {
                                datas.push(data_d);
                                data_d = 0;
                                data_d += (c as u16) << 8;
                                data_ptr += 1;
                            } else {
                                data_d |= (c as u16) << (8 * (1 - (index % 2)));
                            }
                        }
                    }
                } else if line[0] == ".DATA" {
                    data_table.insert(line[1], data_ptr);
                    if line[2].starts_with("\"") && line[2].ends_with("\"") {
                        datas.push(line[2].trim_start_matches("\"").trim_end_matches("\"").chars().collect::<Vec<_>>()[0] as u16);
                    } else {
                        if line[2].ends_with("H") {
                            datas.push(u16::from_str_radix(line[2].trim_end_matches("H"), 16).unwrap());
                        } else if line[2].ends_with("O") {
                            datas.push(u16::from_str_radix(line[2].trim_end_matches("O"), 8).unwrap());
                        } else if line[2].ends_with("B") {
                            datas.push(u16::from_str_radix(line[2].trim_end_matches("B"), 2).unwrap());
                        } else {
                            datas.push(u16::from_str_radix(line[2], 10).unwrap());
                        }
                    }
                    data_ptr += 1;
                } else if line[0] == ".SET" {
                    let mut c = 0;
                    if line[2].ends_with("H") {
                        c = u16::from_str_radix(line[2].trim_end_matches("H"), 16).unwrap();
                    } else if line[2].ends_with("O") {
                        c = u16::from_str_radix(line[2].trim_end_matches("O"), 8).unwrap();
                    } else if line[2].ends_with("B") {
                        c = u16::from_str_radix(line[2].trim_end_matches("B"), 2).unwrap();
                    } else {
                        c = u16::from_str_radix(line[2], 10).unwrap();
                    }
                    match line[1] {
                        "CODE" => {self.code_start_address = c},
                        "STACK" => {self.stack_start_address = c},
                        "DATA" => {self.data_start_address = c},
                        _ => panic!("[Syntex Error] Line{}:Does not spport this setting item.", line_num)
                    }
                }
            } else if line.starts_with(";"){
                continue;
            } else {
                let code = line.split(" ").collect::<Vec<_>>();

                if code.len() == 0 {
                    continue;
                } else if code.len() == 1 {
                    valid_code.push((code[0], None, line_num));
                } else if code.len() == 2 {
                    valid_code.push((code[0], Some(code[1].trim().split(",").collect::<Vec<_>>()), line_num));
                } else {
                    println!("[WARNING] Line{}:Unknown line contnt", line_num);
                }
            }
        }

        // insert compile pre operation
        let c_s  = self.stack_start_address.to_string();
        if self.mode.eq("bin") {
            valid_code.insert(0, ("LOAD", Some(vec![c_s.as_str(), "%SS"],), 0));
        } else if self.mode.eq("lib") {
            // not supported
        } else {
            panic!("[ERROR] Unknown mode")
        }

        // to bcode
        let mut addr: u16 = self.code_start_address;
        let mut label_table: HashMap<&str, u16> = HashMap::new();
        for (op, args, line_num) in valid_code {
            let mut new_args = vec![];
            let args_num = match args {
                None => 0,
                Some(args) => {
                    for arg in args.clone() {
                        match define_table.get(arg) {
                            Some(e) => new_args.push(*e),
                            None => new_args.push(arg)
                        }
                    }
                    args.len()
                }
            };

            let op = match define_table.get(op) {
                Some(s) => s,
                None => op
            };

            if op.ends_with(":") {
                if args_num == 0 {
                    label_table.insert(op.trim_end_matches(":"), addr);
                } else {
                    panic!("[Syntex Error] Line{}:Labels need to be on separate lines.", line_num);
                }
                addr += 1 + args_num as u16;
                continue;
            }

            let op_info = match self.toml.try_get(op) {
                Ok(info) => info,
                Err(Error::Query(error::QueryError::NotFound)) => panic!("[Syntex Error] Line{}:Undefined instruction {}", line_num, op),
                Err(e) => panic!("[ERROR] {}", e)
            };
            let bcode_temp = match op_info.get("bcode").as_integer() {
                Some(i) => {
                    match i.value().as_positive() {
                        Some(b) => b as u16,
                        None => panic!("[ERROR] Premiter {} info \"bcode\" can't be parsed, please check", op)
                    }
                }
                None => panic!("[ERROR] Premiter {} info dose not have \"bcode\", please check", op)
            };

            let request_arg_num = match op_info.get("arg_num").as_integer() {
                Some(i) => {
                    match i.value().as_positive() {
                        Some(b) => b,
                        None => panic!("[ERROR] Premiter {} info \"arg_num\" can't be parsed, please check", op)
                    }
                }
                None => panic!("[ERROR] Premiter {} info dose not have \"arg_num\", please check", op)
            };

            if request_arg_num != args_num as u64 {
                panic!("[Syntex Error] Line{}:{} need {} arguments but give {}", line_num, op, request_arg_num, args_num);
            }

            if args_num != 0 {
                match op_info.get("arg_kinds").as_array() {
                    Some(a) => {
                        let a = a.items();
                        let mut finded = false;
                        for i in a.get().iter() {
                            match i.as_array() {
                                Some(i) => {
                                    let i = i.items().get();
                                    let mut equal = true;
                                    let mut temp_bcode = vec![];
                                    let rarg = i.iter().map(|x|
                                            match x.as_str() {
                                                Some(j) => j.value(),
                                                None => panic!("[ERROR] Premiter {} info argument(s) in \"arg_kinds\"can't be parsed, please check", op)
                                            }
                                        ).collect::<Vec<_>>();
                                    let rarg_len = rarg.len();

                                    for (p, a) in new_args.iter().enumerate() {
                                        match self.arg_kind_eq(a, rarg[p], &label_table, &data_table) {
                                            Ok((e, b)) => {equal &= e; temp_bcode.push(b)},
                                            Err(e) => panic!("[Syntex Error] Line{}:{}", line_num, e)
                                        }
                                    }
                                    if equal {
                                        if rarg_len as u64 == request_arg_num + 1 {
                                            bcode.push(bcode_temp + rarg[rarg_len - 1].parse::<u16>().unwrap());
                                        } else if rarg_len as u64 != request_arg_num {
                                            panic!("[ERROR] request arguments number iligal");
                                        } else {
                                            bcode.push(bcode_temp);
                                        }
                                        bcode.append(&mut temp_bcode);
                                        finded = true;
                                        break;
                                    }
                                },
                                None => panic!("[ERROR] Premiter {} info dose not have argument(s) in \"arg_kinds\", please check", op)
                            }
                        }
                        if !finded {
                            panic!("[Syntex Error] Line{}:The {} directive does not support this type of parameter(s)", line_num, op);
                        }
                    },
                    None => panic!("[ERROR] Premiter {} info dose not have \"arg_kinds\", please check", op)
                }
            }
        }
        let mut zeros = vec![0u16; self.data_start_address as usize - bcode.len()];
        let mut output_file = File::create(output_file_path).unwrap();
        let mut output_buf = vec![];
        bcode.append(&mut zeros);
        bcode.append(&mut datas);
        let mut zeros = vec![0u16; self.stack_start_address as usize - datas.len() + 32];
        bcode.append(&mut zeros);

        for i in bcode {
            output_buf.push((i >> 8) as u8);
            output_buf.push((i & 0xFF) as u8);
        }

        output_file.write(&output_buf).unwrap();
    }

    fn arg_kind_eq(&self, arg: &str, request_kind: &str, label_table: &HashMap<&str, u16>, data_table: &HashMap<&str, u16>) -> Result<(bool, u16), &str> {
        let mut equal = false;
        let mut arg_kind = "unknow";
        let mut bcode: u16 = 0;
        let new_arg = match data_table.get(arg.trim_start_matches("$")) {
            Some(a) => format!("[{}]", a + self.data_start_address),
            None => {
                if arg.starts_with("$") {
                    return Err("Undefined variable");
                } else {
                    String::from(arg)
                }
            }
        };

        let new_arg = new_arg.as_str();
        if new_arg.starts_with("%") {
            arg_kind = "regs";
            bcode = match new_arg.trim_start_matches("%") {
                "A" => 0b1000_0000_0000_0000,
                "B" => 0b1000_0000_0000_0001,
                "C" => 0b1000_0000_0000_0011,
                "D" => 0b1000_0000_0000_0010,
                "E" => 0b1000_0000_0000_0110,
                "F" => 0b1000_0000_0000_0100,
                "SS" => 0b1000_0000_0000_0101,
                "SP" => 0b1000_0000_0000_0111,
                _ => return Err("Undefined register")
            };
        } else if new_arg.starts_with("[") && new_arg.ends_with("]") {
            arg_kind = "addr";
            let new_arg = new_arg.trim_start_matches("[").trim_end_matches("]");

            if new_arg.ends_with("H") {
                bcode = match u16::from_str_radix(new_arg.trim_end_matches("H"), 16) {
                    Ok(c) => c,
                    Err(e) => return Err("Bad base or base mark")
                };
            } else if new_arg.ends_with("O") {
                bcode = match u16::from_str_radix(new_arg.trim_end_matches("O"), 8) {
                    Ok(c) => c,
                    Err(e) => return Err("Bad base or base mark")
                };
            } else if new_arg.ends_with("B") {
                bcode = match u16::from_str_radix(new_arg.trim_end_matches("B"), 2) {
                    Ok(c) => c,
                    Err(e) => return Err("Bad base or base mark")
                };
            } else {
                if new_arg.starts_with("%") {
                    bcode = match new_arg.trim_start_matches("%") {
                        "A" => 0b1000_0000_0000_0000,
                        "B" => 0b1000_0000_0000_0001,
                        "C" => 0b1000_0000_0000_0011,
                        "D" => 0b1000_0000_0000_0010,
                        "E" => 0b1000_0000_0000_0110,
                        "F" => 0b1000_0000_0000_0100,
                        "SS" => 0b1000_0000_0000_0101,
                        "SP" => 0b1000_0000_0000_0111,
                        _ => return Err("Undefined register")
                    };
                } else {
                    bcode = match new_arg.parse::<u16>() {
                        Ok(c) => c,
                        Err(e) => return Err("Bad base or base mark")
                    };
                }
            }
        } else if new_arg.chars().collect::<Vec<_>>()[0].is_ascii_digit() {
            arg_kind = "imdn";
            if new_arg.ends_with("H") {
                bcode = match u16::from_str_radix(new_arg.trim_end_matches("H"), 16) {
                    Ok(c) => c,
                    Err(e) => return Err("Bad base or base mark")
                };
            } else if new_arg.ends_with("O") {
                bcode = match u16::from_str_radix(new_arg.trim_end_matches("O"), 8) {
                    Ok(c) => c,
                    Err(e) => return Err("Bad base or base mark")
                };
            } else if new_arg.ends_with("B") {
                bcode = match u16::from_str_radix(new_arg.trim_end_matches("B"), 2) {
                    Ok(c) => c,
                    Err(e) => return Err("Bad base or base mark")
                };
            } else {
                bcode = match new_arg.parse::<u16>() {
                    Ok(c) => c,
                    Err(e) => return Err("Bad base or base mark")
                };
            }
        } else {
            arg_kind = "label";
            bcode = match label_table.get(new_arg) {
                Some(b) => b.to_owned(),
                None => return Err("Unknown string")
            };
        }

        if arg_kind == request_kind {
            equal = true;
        }

        if equal {
            Ok((true, bcode))
        } else {
            Ok((false, 0))
        }
    }
}