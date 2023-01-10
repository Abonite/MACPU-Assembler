const IDLE: u8                  = 0;
const INST: u8                  = 10;
const GET_ARG_1_FIRST_CHAR: u8  = 20;
const GET_ARG_1: u8             = 30;
const GET_ARG_2_FISRT_CHAR: u8  = 40;
const GET_ARG_2: u8             = 50;
const FINISH: u8                = 60;

pub struct InstructionProcessor {
    file_in_line: Vec<(usize, String)>,
    code_ast_buffer: Vec<AST>
}

enum arg_type {
    addr(u16),
    regs(u16),
    imdn(u16),
    label(String),
    null()
}

enum inst_type {
    label(String),
    inst(String)
}

struct AST {
    inst: inst_type,
    arg_1: arg_type,
    arg_2: arg_type
}

impl InstructionProcessor {
    pub fn new(file_in_line: Vec<(usize, String)>) -> InstructionProcessor {
        InstructionProcessor {
            file_in_line,
            code_ast_buffer: vec![]
        }
    }

    pub fn lexical_check(&mut self) -> bool {
        let mut no_errors = true;

        for (line_num, line) in self.file_in_line {
            let mut curr_sate = IDLE;

            let mut inst = String::new();
            let mut arg_1 = String::new();
            let mut arg_2 = String::new();

            for c in line.chars() {
                match curr_sate {
                    IDLE => {
                        if c != ' ' && c != '\t' && c != '\n' {
                            curr_sate = INST;
                            inst.push(c);
                        } else if c == '\n' {
                            break;
                        } else {
                            continue;
                        }
                    },
                    INST => {
                        if (c == ' ' || c == '\t') && c != '\n' {
                            curr_sate = GET_ARG_1_FIRST_CHAR;
                        } else if c == '\n' {
                            break;
                        } else {
                            inst.push(c);
                        }
                    },
                    GET_ARG_1_FIRST_CHAR => {
                        if (c == ' ' || c == '\t') && c != '\n' {
                            continue;
                        } else if c == '\n' {
                            break;
                        } else if c.is_alphanumeric() || c == '_' {
                            curr_sate = GET_ARG_1;
                            arg_1.push(c)
                        } else {
                            continue;
                        }
                    },
                    GET_ARG_1 => {
                        if (c == ' ' || c == '\t') && c != '\n' {
                            curr_sate = GET_ARG_2_FISRT_CHAR;
                        } else if c == '\n' {
                            break;
                        } else {
                            arg_2.push(c);
                        }
                    },
                    GET_ARG_2_FISRT_CHAR => {
                        if (c == ' ' || c == '\t') && c != '\n' {
                            continue;
                        } else if c == '\n' {
                            break;
                        } else if c.is_alphanumeric() || c == '_' {
                            curr_sate = GET_ARG_2;
                            arg_2.push(c);
                        } else {
                            continue;
                        }
                    },
                    GET_ARG_2 => {
                        if (c == ' ' || c == '\t') && c != '\n' {
                            curr_sate = FINISH;
                        } else if c == '\n' {
                            break;
                        } else if c.is_alphanumeric() || c == '_' {
                            arg_2.push(c);
                        } else {
                            continue;
                        }
                    },
                    FINISH => {
                        if (c == ' ' || c == '\t') && c != '\n' {
                            continue;
                        } else if c == '\n' {
                            break;
                        } else {
                            // ERROR
                        }
                    }
                }
            }
            let inst = inst.trim();
            let arg_1 = arg_1.trim();
            let arg_2 = arg_2.trim();

            if inst.is_empty() {
                log!("ERROR", line_num, "No dot instruction in this line");
                no_errors = false;
            } else {
                if no_errors {
                    let mut ast = AST {
                        inst: inst_type::inst(String::from(inst)),
                        arg_1: arg_type::null(),
                        arg_2: arg_type::null()
                    };

                    self.ast_buffer.push((line_num, ast));
                } else {
                    continue;
                }
            }
        }

        no_errors
    }

}