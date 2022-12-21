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

const IDLE: u16 = 0b0;
const GET_ARG_FIRST_CHAR: u16 = 0b1;
const GET_ARG: u16 = 0b10;
const FINISH: u16 = 0b100;


pub struct DotInstrctionsProcessor {
    dot_instrctions: Vec<(usize, String)>,
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
            dot_instrctions
        }, instrcutions)
    }

    pub fn generate(&self) {
        for (line_num, line) in self.dot_instrctions.clone() {
            if line.starts_with(".SET") {

            } else if line.starts_with(".DATA") {

            } else if line.starts_with(".ARRAY") {

            } else if line.starts_with(".DEFINE") {

            } else {
                // Error
            }
        }
    }

    fn one_arg(&self, line: String) {
        /// line must be removed start

        let mut curr_state = IDLE;

        let mut arg = String::new();

        for c in line.chars() {
            match curr_state {
                IDLE => {
                    if c == ' ' {
                        curr_state = GET_ARG_FIRST_CHAR;
                    }
                },
                GET_ARG_FIRST_CHAR => {
                    if c.is_ascii_digit() {
                        // error: start with digit
                    } else if c.is_ascii_punctuation() && c != '_' {
                        // error: can not start with punctuation but "_"
                    } else {
                        arg.push(c);
                        curr_state = GET_ARG;
                    }
                },
                GET_ARG => {
                    if c != ' ' {
                        arg.push(c)
                    } else {
                        curr_state = FINISH;
                    }
                },
                FINISH => {
                    if c != ' ' || c != '\t' {
                        // error: too many args
                    }
                },
                _ => {
                    curr_state == IDLE;
                    // error: state macheine error
                }
            }
        }
    }
}

struct AST {
    
}

fn set_parser(line: String) {

}