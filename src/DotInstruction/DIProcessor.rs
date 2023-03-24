use std::collections::HashMap;
use super::BaseDInstructions::{
    Setting_item,
    SETTINGS,
    SET,
    VAR,
    STR,
    ARR,
    DEF
};

enum DI {
    SE(SET),
    VA(VAR),
    ST(STR),
    AR(ARR),
    DE(DEF)
}

pub struct DotInstrctionsProcessor {
    file: Vec<(usize, String)>,
    settings_table: HashMap<String, Setting_item>,
    datas: Vec<u8>
}

impl DotInstrctionsProcessor {
    pub fn new(file: Vec<(usize, String)>) -> DotInstrctionsProcessor {
        DotInstrctionsProcessor {
            file,
            settings_table: SETTINGS,
            datas: vec![]
        }
    }

    pub fn extract(&mut self) -> Vec<(usize, String)>{
        let mut pi = vec![];
        let mut i = vec![];

        for (line_num, line) in self.file {
            if line.starts_with(".") {
                pi.push((line_num, line));
            } else {
                i.push((line_num, line));
            }
        }

        self.file = pi;
        return i;
    }

    pub fn process(&mut self) {
        
    }
}

struct DIProcessor {
    line_num: usize,
    line: String
}

impl DIProcessor {
    fn new(line_num: usize, line: String) -> DIProcessor {
        DIProcessor {
            line_num,
            line
        }
    }

    fn start(&self) -> Result<DI, String> {
        if self.line.starts_with(".SET") {
            self.pset(self.line.trim_start_matches(".SET"));
        } else if self.line.starts_with(".VAR") {

        } else if self.line.starts_with(".STR") {

        } else if self.line.starts_with(".ARR") {

        } else if self.line.starts_with(".DEF") {

        } else {
            return Err(format!("Line: {} - \"{}\" not a legal preprocessing command", self.line_num, self.line));
        }
    }

    fn pset(&self, args: &str) {
        let mut farg = String::new();
        let mut sarg = String::new();
        let mut nvs = false;

        const FIRST_ARG: u8 = 0;
        const BLANK: u8 = 1;
        const SECOND_ARG: u8 = 2;

        let mut curr_state = FIRST_ARG;
        for c in args.chars() {
            match curr_state {
                FIRST_ARG => {
                    if c != '\t' || c != ' ' {
                        farg.push(c);
                    } else {
                        curr_state = BLANK;
                    }
                },
                BLANK => {
                    if c == '\n' {
                        nvs = true;
                        break;
                    } else if c.is_ascii_alphabetic() && c != ' ' && c != '\t' {
                        curr_state = SECOND_ARG;
                    } else if c == ' ' && c == '\t' {
                        ;
                    } else {

                    }
                }
            }
        }
    }
}