use std::collections::HashMap;
use std::hash::Hash;
use std::num::ParseIntError;

macro_rules! log {
    ($log_level: expr, $line_num: expr, $message: expr) => {
        {println!("[{}] Line {}: {}", $log_level, $line_num, $message);}
    };
}

const IDLE: u16                 = 0b0;
const GET_ARG_1_FIRST_CHAR: u16 = 0b1;
const GET_ARG_1: u16            = 0b10;
const ARG_1_FINISH: u16         = 0b100;
const GET_ARG_2: u16            = 0b1000;
const FINISH: u16               = 0b10000;

enum ParseError {
    ArgumentStartWithNumberError(ASWNError),
    ArgumentStartWithNonUnderlineError(ASWNUError),
    TooManyArgmumentError(TMAError),
    StateMachineError(SMError),
    ParseNumberError(ParseIntError)
}

#[derive(Debug)]
struct ASWNError;

impl std::fmt::Display for ASWNError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "First argument can not start with number")
    }
}

impl std::error::Error for ASWNError {}

#[derive(Debug)]
struct ASWNUError;

impl std::fmt::Display for ASWNUError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Argument can not start with a symbol other than '_'")
    }
}

impl std::error::Error for ASWNUError {}

#[derive(Debug)]
struct TMAError;

impl std::fmt::Display for TMAError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Only need one argmument")
    }
}

impl std::error::Error for TMAError {}

#[derive(Debug)]
struct SMError;

impl std::fmt::Display for SMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "State machine error")
    }
}

impl std::error::Error for SMError {}

#[derive(Debug)]
struct PNError;

impl std::fmt::Display for PNError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Illegal character while parsing number")
    }
}

impl std::error::Error for PNError {}

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
    define_info: HashMap<String, String>
}

#[derive(Clone)]
pub enum setting_items {
    sibool(bool),
    sinum(u16),
    sistr(String),
    siarr(Vec<u16>)
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
            define_info: HashMap::new()
        }, instrcutions)
    }

    pub fn generate(&mut self) {
        // error?
        for (line_num, line) in self.dot_instrctions.clone() {
            if line.starts_with(".SET") {
                match self.two_args(line.trim_start_matches(".SET")) {
                    Ok((si, sv)) => {
                        if self.set_info.contains_key(&si) {
                            match sv.clone() {
                                setting_items::siarr(a) => log!("ERROR", line_num, "\".SET\" type cannot be array"),
                                setting_items::sibool(_) => (),
                                setting_items::sinum(_) => (),
                                setting_items::sistr(_) => ()
                            }
                            self.set_info.insert(si, sv);
                        } else {
                            log!("ERROR", line_num, "Unkonwn set arg");
                        }
                    },
                    Err(pe) => {
                        match pe {
                            ParseError::ArgumentStartWithNonUnderlineError(e) => {
                                log!("ERROR", line_num, e);
                            },
                            ParseError::ArgumentStartWithNumberError(e) => {
                                log!("ERROR", line_num, e);
                            },
                            ParseError::ParseNumberError(e) => {
                                log!("ERROR", line_num, e);
                            },
                            ParseError::StateMachineError(e) => {
                                log!("FATAL", line_num, e);
                            },
                            ParseError::TooManyArgmumentError(e) => {
                                log!("ERROR", line_num, e);
                            }
                        }
                    }
                }
            } else if line.starts_with(".DATA") {
                match self.two_args(line.trim_start_matches(".DATA")) {
                    Ok((di, dv)) => {
                        let pointer = self.data_buffer.len();
                        if pointer > u16::MAX as usize {
                            log!("FATAL", line_num, "Too many data has exceeded the addressing range");
                        } else {
                            self.data_info.insert(di, self.data_buffer.len() as u16);
                        }
                        match dv {
                            setting_items::sibool(b) => {
                                self.data_buffer.push(if b {1_u16} else {0_u16})
                            },
                            setting_items::sinum(n) => {
                                self.data_buffer.push(n)
                            },
                            setting_items::sistr(s) => {
                                log!("ERROR", line_num, "\".DATA\" type cannot be a string")
                            },
                            setting_items::siarr(a) => {
                                log!("ERROR", line_num, "\".DATA\" type cannot be a array")
                            }
                        }
                    },
                    Err(pe) => {
                        match pe {
                            ParseError::ArgumentStartWithNonUnderlineError(e) => {
                                log!("ERROR", line_num, e);
                            },
                            ParseError::ArgumentStartWithNumberError(e) => {
                                log!("ERROR", line_num, e);
                            },
                            ParseError::ParseNumberError(e) => {
                                log!("ERROR", line_num, e);
                            },
                            ParseError::StateMachineError(e) => {
                                log!("FATAL", line_num, e);
                            },
                            ParseError::TooManyArgmumentError(e) => {
                                log!("ERROR", line_num, e);
                            }
                        }
                    }
                }
            } else if line.starts_with(".ARRAY") {
                match self.two_args(line.trim_start_matches(".ARRAY")) {
                    Ok((ai, av)) => {
                        let pointer = self.data_buffer.len();
                        if pointer > u16::MAX as usize {
                            log!("FATAL", line_num, "Too many data has exceeded the addressing range");
                        } else {
                            self.data_info.insert(ai, self.data_buffer.len() as u16);
                        }
                        match av {
                            setting_items::sibool(_) => {
                                log!("ERROR", line_num, "\".ARRAY\" type cannot be a bool value")
                            },
                            setting_items::sinum(n) => {
                                self.data_buffer.push(n);
                            },
                            setting_items::sistr(s) => {
                                for c in s.chars() {
                                    self.data_buffer.push(c as u16);
                                }
                                self.data_buffer.push('\0' as u16);
                            },
                            setting_items::siarr(a) => {
                                for n in a {
                                    self.data_buffer.push(n);
                                }
                            }
                        }
                    },
                    Err(pe) => {
                        match pe {
                            ParseError::ArgumentStartWithNonUnderlineError(e) => {
                                log!("ERROR", line_num, e);
                            },
                            ParseError::ArgumentStartWithNumberError(e) => {
                                log!("ERROR", line_num, e);
                            },
                            ParseError::ParseNumberError(e) => {
                                log!("ERROR", line_num, e);
                            },
                            ParseError::StateMachineError(e) => {
                                log!("FATAL", line_num, e);
                            },
                            ParseError::TooManyArgmumentError(e) => {
                                log!("ERROR", line_num, e);
                            }
                        }
                    }
                }
            } else if line.starts_with(".DEFINE") {
                match self.two_args(line.trim_start_matches(".DEFINE")) {
                    Ok((di, dv)) => {
                        match dv {
                            setting_items::siarr(_) => log!("ERROR", line_num, "\".DEFINE\" type cannot be array"),
                            setting_items::sibool(_) => log!("ERROR", line_num, "\".DEFINE\" type cannot be bool"),
                            setting_items::sinum(_) => log!("ERROR", line_num, "\".DEFINE\" type cannot be number"),
                            setting_items::sistr(s) => {self.define_info.insert(di, s);}
                        }
                    },
                    Err(pe) => {
                        match pe {
                            ParseError::ArgumentStartWithNonUnderlineError(e) => {
                                log!("ERROR", line_num, e);
                            },
                            ParseError::ArgumentStartWithNumberError(e) => {
                                log!("ERROR", line_num, e);
                            },
                            ParseError::ParseNumberError(e) => {
                                log!("ERROR", line_num, e);
                            },
                            ParseError::StateMachineError(e) => {
                                log!("FATAL", line_num, e);
                            },
                            ParseError::TooManyArgmumentError(e) => {
                                log!("ERROR", line_num, e);
                            }
                        }
                    }
                }
            } else {
                log!("ERROR", line_num, "Unknown Instruction");
            }
        }
    }

    pub fn getinfo(&self) -> (HashMap<String, setting_items>, HashMap<String, u16>, HashMap<String, String>, Vec<u16>) {
        (self.set_info.clone(), self.data_info.clone(), self.define_info.clone(), self.data_buffer.clone())
    }
    /// Instructions that need to remove the beginning of a line
    fn one_arg(&self, line: &str) -> Result<String, ParseError> {
        let mut curr_state = IDLE;

        let mut arg = String::new();

        for c in line.chars() {
            match curr_state {
                IDLE => {
                    if c == ' ' && c == '\t' {
                        curr_state = GET_ARG_1_FIRST_CHAR;
                    }
                },
                GET_ARG_FIRST_CHAR => {
                    if c.is_ascii_digit() {
                        return Err(ParseError::ArgumentStartWithNumberError(ASWNError));
                    } else if c.is_ascii_punctuation() && c != '_' {
                        return Err(ParseError::ArgumentStartWithNonUnderlineError(ASWNUError));
                    } else {
                        arg.push(c);
                        curr_state = GET_ARG_1;
                    }
                },
                GET_ARG => {
                    if c != '\n' && c != ' ' && c != '\t' {
                        arg.push(c)
                    } else {
                        curr_state = ARG_1_FINISH;
                    }
                },
                ARG_1_FINISH => {
                    if c != ' ' && c != '\t' && c != '\n' {
                        return Err(ParseError::TooManyArgmumentError(TMAError));
                    }
                },
                _ => {
                    curr_state = IDLE;
                    return Err(ParseError::StateMachineError(SMError));
                }
            }
        }
        return Ok(arg);
    }

    /// Instructions that need to remove the beginning of a line
    fn two_args(&self, line: &str) -> Result<(String, setting_items), ParseError> {
        let arg_str: u8 = 0b0;
        let arg_not_str: u8 = 0b1;

        let mut curr_state = IDLE;

        let mut arg_2_type = arg_not_str;

        let mut arg_1 = String::new();
        let mut arg_2 = String::new();

        for c in line.chars() {
            match curr_state {
                IDLE => {
                    if c == ' ' && c == '\t' {
                        curr_state = GET_ARG_1_FIRST_CHAR;
                    }
                },
                GET_ARG_1_FIRST_CHAR => {
                    if c.is_ascii_digit() {
                        return Err(ParseError::ArgumentStartWithNumberError(ASWNError));
                    } else if c.is_ascii_punctuation() && c != '_' {
                        return Err(ParseError::ArgumentStartWithNonUnderlineError(ASWNUError));
                    } else {
                        arg_1.push(c);
                        curr_state = GET_ARG_1;
                    }
                },
                GET_ARG_1 => {
                    if c != ' ' && c != '\t' {
                        arg_1.push(c)
                    } else {
                        curr_state = ARG_1_FINISH;
                    }
                },
                ARG_1_FINISH => {
                    if c != ' ' && c != '\t' {
                        if c == '\"' {
                            arg_2_type = arg_str;
                            curr_state = GET_ARG_2;
                        } else {
                            arg_2.push(c);
                            curr_state = GET_ARG_2;
                        }
                    }
                },
                GET_ARG_2 => {
                    if arg_2_type == arg_not_str {
                        if c != ' ' && c != '\t' && c != '\n' {
                            arg_2.push(c);
                        } else {
                            curr_state = FINISH;
                        }
                    } else if arg_2_type == arg_str {
                        if c != '\"' {
                            arg_2.push(c);
                        } else {
                            curr_state = FINISH;
                        }
                    } else {
                        return Err(ParseError::StateMachineError(SMError));
                    }
                },
                FINISH => {
                    if c != ' ' && c != '\t' && c != '\n' {
                        return Err(ParseError::TooManyArgmumentError(TMAError));
                    }
                }
                _ => {
                    curr_state = IDLE;
                    return Err(ParseError::StateMachineError(SMError));
                }
            }
        }

        if arg_2_type == arg_not_str {
            if arg_2.contains(",") {
                let data_arr_buffer = arg_2.split(",").collect::<Vec<_>>();
                let mut data_arr = vec![];
                for data in data_arr_buffer {
                    match self.str2num(data) {
                        Ok(a) => data_arr.push(a),
                        Err(e) => return Err(ParseError::ParseNumberError(e))
                    }
                }
                return Ok((arg_1, setting_items::siarr(data_arr)));
            } else {
                match self.str2num(arg_2.as_str()) {
                    Ok(n) => return Ok((arg_1, setting_items::sinum(n))),
                    Err(e) => return Err(ParseError::ParseNumberError(e))
                }
            }
        } else if arg_2_type == arg_str {
            if arg_2.to_ascii_uppercase() == "TRUE" {
                return Ok((arg_1, setting_items::sibool(true)));
            } else if arg_2.to_ascii_uppercase() == "FALSE" {
                return Ok((arg_1, setting_items::sibool(false)));
            } else {
                return Ok((arg_1, setting_items::sistr(arg_2)));
            }
        } else {
            return Err(ParseError::StateMachineError(SMError));
        }
    }

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
