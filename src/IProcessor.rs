const IDLE: u16                 = 0b0;
const INST: u16                 = 0b1;
const GET_ARG_1_FIRST_CHAR: u16 = 0b10;
const GET_ARG_1: u16            = 0b100;
const GET_ARG_2_FISRT_CHAR: u16 = 0b1000;
const GET_ARG_2: u16            = 0b10000;

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

    pub fn syntax_check(&mut self) {
        for (line_num, line) in self.file_in_line.clone() {
            
        }
    }

    fn ast_generate(&mut self, line: String) {
        let mut current_state = IDLE;

        let mut inst = String::new();
        let mut arg_1 = String::new();
        let mut arg_2 = String::new();

        let mut ast = AST {
            inst: inst_type::inst(String::new()),
            arg_1: arg_type::null(),
            arg_2: arg_type::null()
        };

        for c in line.chars() {
            match current_state {
                IDLE => {
                    if c != ' ' && c != '\t' {
                        current_state = INST;
                        inst.push(c);
                    }
                },
                INST => {
                    if c != ' ' && c != '\t' && c != '\n' {
                        inst.push(c)
                    } else if c == '\n' {
                        ast.inst = inst_type::inst(inst);
                        break;
                    } else if c == ':'{
                        ast.inst = inst_type::label(inst);
                        break;
                    } else if c == ' ' || c == '\t' {
                        current_state = GET_ARG_1_FIRST_CHAR;
                    }
                },
                GET_ARG_1_FIRST_CHAR => {
                    if c != ' ' && c != '\t' && c != '\n' {
                        if c.is_ascii_digit() || (c.is_ascii_punctuation() && (c != '_' && c != '%' && c != '[')) {
                            arg_1.push(c);
                            current_state = GET_ARG_1;
                        } else {
                            // ERROR
                        }
                    } else if c =='\n' {
                        break;
                    }
                },
                GET_ARG_1 => {
                    if c != ',' {
                        arg_1.push(c)
                    } else if c == '\n' {
                        break;
                    } else {
                        current_state = GET_ARG_2_FISRT_CHAR;
                    }
                },
                GET_ARG_2_FISRT_CHAR => {
                    if c != ' ' && c != '\t' && c != '\n' {
                        if c.is_ascii_digit() || (c.is_ascii_punctuation() && (c != '_' && c != '%' && c != '[')) {
                            arg_2.push(c);
                            current_state = GET_ARG_2;
                        } else {
                            // ERROR
                        }
                    } else if c =='\n' {
                        // ERROR
                    }
                },
                GET_ARG_2 => {
                    if c != '\n' {
                        arg_1.push(c)
                    }
                },
                _ => {
                    //ERROR
                }
            }
        }

        if !arg_1.is_empty() {
            let arg_1 = arg_1.trim();
            // type?
        }

        if !arg_2.is_empty() {
            // type?
        }
    }

    fn arg_type_judge(&self, arg: &str) {
        if arg.starts_with("%") {
            //register
        } else if arg.starts_with("[") && arg.ends_with("]") {
            // address
        } else {
            if arg.ends_with("H") {

            } else if arg.ends_with("O") {

            } else if arg.ends_with("B") {

            } else {
                match u16::from_str_radix(arg, 10) {
                    Ok(n) => return ,
                    Err(e) => {

                    }
                }
            }
        }
    }
}