use std::collections::HashMap;
use std::num::ParseIntError;

macro_rules! log {
    ($log_level: expr, $line_num: expr, $message: expr) => {
        {println!("[{}] Line {}: {}", $log_level, $line_num + 1, $message);}
    };
    ($log_level: expr, $message: expr) => (
        {println!("[{}] : {}", $log_level, $message);}
    )
}

const IDLE: u8                  = 0;
const INST: u8                  = 10;
const GET_ARG_1_FIRST_CHAR: u8  = 20;
const GET_ARG_1: u8             = 30;
const GET_ARG_2_FIRST_CHAR: u8  = 40;
const GET_ARG_2: u8             = 50;
const GET_ARG_2_STRING: u8      = 60;
const FINISH: u8                = 70;

#[derive(Debug)]
#[derive(Clone)]
struct AST {
    inst: String,
    arg_1: String,
    arg_2: setting_items
}

/// The Dot Instrction e.g. ProProcess Instrction in this assemblyer must abide by this syntax format:
/// .INSTNAME[SPACE]VARIABLENAME[SPACE]VALUE[SPACE or TAB][\n]
/// or
/// .INSTNAME[SPACE]SETTINGS[SPACE or TAB][\n]
/// or
/// .INSTNAME[SPACE]SETTINGSNAME[SPACE]VALUE[SPACE or TAB][\n]
/// So when check the syntex,
/// I decided to follow this process:
/// 1. Traverse each line, find all the line start with ".", push them into a vector, and delate them in
/// orignal vector
/// 2. For each line start with ".", check them if they are start with an available instruction, if not
/// then throw an Error, collect all the Errors
/// 3. If find avaliable instruction, create a thread to process this line, use the state machine parse
/// and check
/// 4. Throw Error if it does not conform to the syntax

pub struct DotInstrctionsProcessor {
    dot_instrctions: Vec<(usize, String)>,
    set_info: HashMap<String, setting_items>,
    data_buffer: Vec<u16>,
    data_info: HashMap<String, u16>,
    define_info: HashMap<String, String>,
    ast_buffer: Vec<(usize, AST)>
}

#[derive(Clone)]
#[derive(Debug)]
pub enum setting_items {
    sibool(bool),
    sinum(u16),
    sistr(String),
    siarr(Vec<u16>),
    sinull()
}

impl DotInstrctionsProcessor {
    pub fn new(orignal_data_in_line: Vec<(usize, String)>) -> (DotInstrctionsProcessor, Vec<(usize, String)>) {
        let mut dot_instrctions = vec![];
        let mut instrcutions = vec![];

        for (line_num, line) in orignal_data_in_line {
            if line.starts_with(".") {
                dot_instrctions.push((line_num, line));
            } else if line.eq("") {
                continue;
            } else {
                instrcutions.push((line_num, line))
            }
        }
        (DotInstrctionsProcessor {
            dot_instrctions,
            set_info: HashMap::from([
                // default settings
                (String::from("CODE_START_ADDRESS"), setting_items::sinum(0x0)),
                (String::from("DATA_START_ADDRESS"), setting_items::sinum(0x1000)),
                (String::from("STACK_START_ADDRESS"), setting_items::sinum(0x2000)),
            ]),
            data_buffer: vec![],
            data_info: HashMap::new(),
            define_info: HashMap::new(),
            ast_buffer: vec![]
        }, instrcutions)
    }

    pub fn lexical_check(&mut self) -> bool {
        let mut no_errors = true;

        for (line_num, line) in self.dot_instrctions.clone() {
            let mut curr_state = IDLE;

            let mut inst = String::new();
            let mut arg_1 = String::new();
            let mut arg_2 = String::new();

            let mut arg_2_is_string = false;

            for c in line.chars() {
                match curr_state {
                    IDLE => {
                        if c == '.' {
                            curr_state = INST;
                        }
                    },
                    INST => {
                        if (c == ' ' || c == '\t') && c != '\n' {
                            curr_state = GET_ARG_1_FIRST_CHAR;
                        } else if c == '\n' {
                            break;
                        } else {
                            inst.push(c);
                        }
                    },
                    GET_ARG_1_FIRST_CHAR => {
                        if c != ' ' && c != '\t' {
                            if c.is_ascii_digit() || (c.is_ascii_punctuation() && c != '_') {
                                log!("ERROR", line_num, "The argument can not start with number or punctuation but \"_\"");
                                no_errors = false;
                            } else {
                                arg_1.push(c);
                                curr_state = GET_ARG_1;
                            }
                        }
                    },
                    GET_ARG_1 => {
                        if (c == ' ' || c == '\t') && c != '\n' {
                            curr_state = GET_ARG_2_FIRST_CHAR;
                        } else if c == '\n' {
                            break;
                        } else {
                            arg_1.push(c);
                        }
                    },
                    GET_ARG_2_FIRST_CHAR => {
                        if (c == ' ' || c == '\t' ) && c != '\n' {
                            continue;
                        } else if c == '\n' {
                            break;
                        } else if (c != ' ' && c != '\t' && c != '\n') {
                            if c == '\"' {
                                curr_state = GET_ARG_2_STRING;
                            } else {
                                curr_state = GET_ARG_2;
                                arg_2.push(c);
                            }
                        } else {
                            log!("FATAL", line_num, "State machine error");
                            no_errors = false;
                        }
                    },
                    GET_ARG_2_STRING => {
                        arg_2_is_string = true;
                        if c == '\"' {
                            curr_state = FINISH;
                        } else if c == '\n' {
                            log!("ERROR", line_num, "Argument finished with out \"");
                            no_errors = false;
                        } else {
                            arg_2.push(c);
                        }
                    },
                    GET_ARG_2 => {
                        if c != '\n' {
                            arg_2.push(c);
                        } else {
                            break;
                        }
                    },
                    FINISH => {
                        if (c != '\n' && c != ' ' && c != '\t') {
                            log!("ERROR", line_num, "Too many arguments");
                            no_errors = false;
                        } else if c == '\n' {
                            break;
                        } else {
                            continue;
                        }
                    },
                    _ => {
                        log!("FATAL", line_num, "State machine Error");
                        no_errors = false;
                    }
                }
            }

            let inst = inst.trim();
            let arg_1 = arg_1.trim();
            let arg_2 = arg_2.trim();

            log!("DEBUG", line_num, format!("inst: {}, arg_1: {}, arg_2: {}", inst, arg_1, arg_2));

            if inst.is_empty() {
                log!("ERROR", line_num, "No dot instruction in this line");
                no_errors = false;
            } else {
                if no_errors {
                    let mut ast = AST {
                        inst: String::new(),
                        arg_1: String::new(),
                        arg_2: setting_items::sinull()
                    };

                    if arg_2_is_string {
                        ast = AST {
                            inst: String::from(inst),
                            arg_1: String::from(arg_1),
                            arg_2: setting_items::sistr(String::from(arg_2))
                        };

                    } else {
                        if arg_2.is_empty() {

                        } else if arg_2.contains(',') {
                            let mut array = vec![];
                            for item in arg_2.split(',').map(|x| x.trim()) {
                                match self.str2num(item) {
                                    Ok(i) => array.push(i),
                                    Err(e) => {
                                        log!("ERROR", line_num, e);
                                        no_errors = false;
                                    }
                                }
                            }
                            ast = AST {
                                inst: String::from(inst),
                                arg_1: String::from(arg_1),
                                arg_2: setting_items::siarr(array)
                            };
                        } else {
                            match self.str2num(arg_2) {
                                Ok(a) => {
                                    ast = AST {
                                        inst: String::from(inst),
                                        arg_1: String::from(arg_1),
                                        arg_2: setting_items::sinum(a)
                                    };
                                },
                                Err(e) => {
                                    if arg_2.to_uppercase() == "TRUE" {
                                        ast = AST {
                                            inst: String::from(inst),
                                            arg_1: String::from(arg_1),
                                            arg_2: setting_items::sibool(true)
                                        };
                                    } else if arg_2.to_uppercase() == "FALSE" {
                                        ast = AST {
                                            inst: String::from(inst),
                                            arg_1: String::from(arg_1),
                                            arg_2: setting_items::sibool(false)
                                        };
                                    } else {
                                        log!("ERROR", line_num, e);
                                        no_errors = false;
                                    }
                                }
                            }
                        }
                    }
                    self.ast_buffer.push((line_num, ast));
                } else {
                    continue;
                }
            }
        }
        log!("DEBUG", 0xdeb, format!("{:?}", self.ast_buffer));
        no_errors
    }

    pub fn syntax_check(&mut self) -> bool {
        let mut no_errors = true;
        for (line_num, ast) in self.ast_buffer.clone() {
            let mut data_pointer = self.data_buffer.len() as u16;
            if self.data_buffer.len() <= u16::MAX as usize {
                data_pointer = self.data_buffer.len() as u16;
            } else {
                log!("ERROR", line_num - 1, "The data volume is greater than the addressable maximun space");
                no_errors = false;
            }
            if ast.inst == "SET" {
                if ast.arg_1.is_empty() {
                    log!("ERROR", line_num, "\".SET\" instrcution need at least one argument");
                    no_errors = false;
                } else {
                    match ast.arg_2 {
                        setting_items::sinull() => {
                            log!("WARNING", line_num, "Not supported");
                        },
                        _ => {
                            if self.set_info.contains_key(&ast.arg_1) {
                                self.set_info.insert(ast.arg_1, ast.arg_2);
                            } else {
                                log!("ERROR", line_num, "Unknown setting items");
                                no_errors = false;
                            }
                        }
                    }
                }
            } else if ast.inst == "DATA" {
                if ast.arg_1.is_empty() {
                    log!("ERROR", line_num, "\".DATA\" instrcution need at least two argument");
                    no_errors = false;
                } else {
                    match ast.arg_2 {
                        setting_items::sinull() => {
                            log!("ERROR", line_num, "\".DATA\" instrcution need at least two argument");
                            no_errors = false;
                        },
                        setting_items::sinum(n) => {
                            self.data_info.insert(ast.arg_1, self.data_buffer.len() as u16);
                            self.data_buffer.push(n);
                        },
                        _ => {
                            log!("ERROR", line_num, "\".DATA\" instrcution does not support defining as array or bool");
                            no_errors = false;
                        }
                    }
                }
            } else if ast.inst == "ARRAY" {
                if ast.arg_1.is_empty() {
                    log!("ERROR", line_num, "\".ARRAY\" instrcution need at least two argument");
                    no_errors = false;
                } else {
                    match ast.arg_2 {
                        setting_items::sinull() => {
                            log!("ERROR", line_num, "\".ARRAY\" instrcution need at least two argument");
                            no_errors = false;
                        },
                        setting_items::siarr(a) => {
                            if self.data_info.contains_key(&ast.arg_1) {
                                log!("ERROR", line_num, "Duplicate definition");
                                no_errors = false;
                            } else {
                                self.data_info.insert(ast.arg_1, data_pointer);
                            }
                            for i in a {
                                self.data_buffer.push(i);
                            }
                        },
                        setting_items::sistr(s) => {
                            if self.data_info.contains_key(&ast.arg_1) {
                                log!("ERROR", line_num, "Duplicate definition");
                                no_errors = false;
                            } else {
                                self.data_info.insert(ast.arg_1, data_pointer);
                            }
                            for i in s.chars() {
                                self.data_buffer.push(i as u16);
                            }
                        },
                        _ => {
                            log!("ERROR", line_num, "\".ARRAY\" instrcution does not support defining as number or bool");
                            no_errors = false;
                        }
                    }
                }
            } else if ast.inst == "DEFINE" {
                if ast.arg_1.is_empty() {
                    log!("ERROR", line_num, "\".ARRAY\" instrcution need at least two argument");
                    no_errors = false;
                } else {
                    match ast.arg_2 {
                        setting_items::sistr(s) => {
                            if self.define_info.contains_key(&ast.arg_1) {
                                log!("ERROR", line_num, "Duplicate definition");
                                no_errors = false;
                            } else {
                                self.define_info.insert(ast.arg_1, s);
                            }
                        },
                        _ => {
                            log!("ERROR", line_num, "\".DEFINE\" instrcution does not support defining as number or bool or array");
                            no_errors = false;
                        }
                    }
                }
            } else {
                log!("ERROR", line_num, "Unknown dot instuction");
                no_errors = false;
            }
        }
        no_errors
    }

    pub fn data_overflow_ckeck (&self) -> bool {
        let mut no_errors = true;
        let code = match self.set_info.get("CODE_START_ADDRESS") {
            Some(a) => {
                match a {
                    setting_items::sinum(n) => n,
                    _ => {
                        log!("ERROR", "\"CODE_START_ADDRESS\" setting item is not a number");
                        no_errors = false;
                        &0_u16
                    }
                }
            },
            None => {
                log!("FATAL", "Can't get \"CODE_START_ADDRESS\" setting item");
                no_errors = false;
                &0_u16
            }
        };

        let data = match self.set_info.get("DATA_START_ADDRESS") {
            Some(a) => {
                match a {
                    setting_items::sinum(n) => n,
                    _ => {
                        log!("ERROR", "\"DATA_START_ADDRESS\" setting item is not a number");
                        no_errors = false;
                        &0_u16
                    }
                }
            },
            None => {
                log!("FATAL", "Can't get \"DATA_START_ADDRESS\" setting item");
                no_errors = false;
                &0_u16
            }
        };

        let stack = match self.set_info.get("STACK_START_ADDRESS") {
            Some(a) => {
                match a {
                    setting_items::sinum(n) => n,
                    _ => {
                        log!("ERROR", "\"STACK_START_ADDRESS\" setting item is not a number");
                        no_errors = false;
                        &0_u16
                    }
                }
            },
            None => {
                log!("FATAL", "Can't get \"STACK_START_ADDRESS\" setting item");
                no_errors = false;
                &0_u16
            }
        };

        if no_errors {
            if stack > data {
                if data < code {
                    log!("ERROR", "Data segment start address must greater then code segment start address");
                    no_errors = false;
                } else {
                    let data_segment_start_address = stack - data + 1;
                    if data_segment_start_address <= self.data_buffer.len() as u16 {
                        log!("ERROR", "The data number in data segment is greater than the preset length");
                        no_errors = false;
                    }
                }
            } else {
                log!("ERROR", "Stack segment start address must greater then data segment start address");
                no_errors = false;
            }
        }
        no_errors
    }
    pub fn getinfo(&self) -> (HashMap<String, setting_items>, HashMap<String, u16>, HashMap<String, String>, Vec<u16>) {
        (self.set_info.clone(), self.data_info.clone(), self.define_info.clone(), self.data_buffer.clone())
    }
    /// Instructions that need to remove the beginning of a line

    fn str2num(&self, org_str: &str) -> Result<u16, ParseIntError> {
        if org_str.ends_with("H") {
            match u16::from_str_radix(org_str.trim_end_matches("H"), 16) {
                Ok(a) => return Ok(a),
                Err(e) => return Err(e)
            };
        } else if org_str.ends_with("O") {
            match u16::from_str_radix(org_str.trim_end_matches("O"), 8) {
                Ok(a) => return Ok(a),
                Err(e) => return Err(e)
            };
        } else if org_str.ends_with("B") {
            match u16::from_str_radix(org_str.trim_end_matches("B"), 2) {
                Ok(a) => return Ok(a),
                Err(e) => return Err(e)
            };
        } else {
            match u16::from_str_radix(org_str, 10) {
                Ok(a) => return Ok(a),
                Err(e) => return Err(e)
            };
        }
    }
}
