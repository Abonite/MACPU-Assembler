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
            match self.pset(self.line.trim_start_matches(".SET")) {
                Ok(v) => return Ok(DI::SE(v)),
                Err(e) => return Err(e)
            };
        } else if self.line.starts_with(".VAR") {
            match self.pvar(self.line.trim_start_matches(".VAR")) {
                Ok(v) => return Ok(DI::VA(v)),
                Err(e) => return Err(e)
            }
        } else if self.line.starts_with(".STR") {
            match self.pstr(self.line.trim_start_matches(".STR")) {
                Ok(v) => return Ok(DI::ST(v)),
                Err(e) => return Err(e)
            }
        } else if self.line.starts_with(".ARR") {

        } else if self.line.starts_with(".DEF") {

        } else {
            return Err(format!("Line: {} - \"{}\" not a legal preprocessing command", self.line_num, self.line));
        }
    }

    fn pset(&self, args: &str) -> Result<SET, String> {
        let mut farg = String::new();
        let mut sarg = String::new();
        let mut nvs = false;

        const FIRST_CHAR: u8 = 0;
        const FIRST_ARG: u8 = 1;
        const BLANK: u8 = 2;
        const SECOND_ARG: u8 = 3;

        let mut curr_state = FIRST_CHAR;
        for c in args.chars() {
            match curr_state {
                FIRST_CHAR => {
                    if !c.is_ascii_digit() {
                        farg.push(c);
                        curr_state = FIRST_ARG;
                    } else {
                        return Err(format!("The first character cannot be a number"));
                    }
                },
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
                    } else if c != '\n' && c != ' ' && c != '\t' {
                        curr_state = SECOND_ARG;
                    } else if c != '\n' && (c == ' ' || c == '\t') {
                    } else {
                        return Err(format!("Unusual string"));
                    }
                },
                SECOND_ARG => {
                    if c != '\n' {
                        sarg.push(c);
                    } else {
                        break;
                    }
                },
                _ => return Err(format!("State machine exception"))
            }
        }
        return Ok(SET::new(farg, sarg, nvs));
    }

    fn pvar(&self, args: &str) -> Result<VAR, String> {
        let mut farg = String::new();
        let mut sarg = String::new();
        let mut targ = String::new();

        const FIRST_CHAR: u8 = 0;
        const FIRST_ARG: u8 = 1;
        const FIRST_BLANK: u8 = 2;
        const SECOND_ARG: u8 = 3;
        const SECOND_BLANK: u8 = 4;
        const THIRD_ARG: u8 = 5;

        let mut curr_state = FIRST_CHAR;
        for c in args.chars() {
            match curr_state {
                FIRST_CHAR => {
                    if !c.is_ascii_digit() {
                        farg.push(c);
                        curr_state = FIRST_ARG;
                    } else {
                        return Err(format!("The first character cannot be a number"));
                    }
                },
                FIRST_ARG => {
                    if c != '\t' || c != ' ' {
                        farg.push(c);
                    } else {
                        curr_state = FIRST_BLANK;
                    }
                },
                FIRST_BLANK => {
                    if c == '\n' {
                        break;
                    } else if c != '\n' && c != ' ' && c != '\t' {
                        curr_state = SECOND_ARG;
                    } else if c != '\n' && (c == ' ' || c == '\t') {
                    } else {
                        return Err(format!("Unusual string"));
                    }
                },
                SECOND_ARG => {
                    if c != '\n' && (c != '\t' || c != ' ') {
                        farg.push(c);
                    } else if c == '\n' {
                        break;
                    } else {
                        curr_state = SECOND_BLANK;
                    }
                },
                SECOND_BLANK => {
                    if c == '\n' {
                        break;
                    } else if c != '\n' && c != ' ' && c != '\t' {
                        curr_state = THIRD_ARG;
                    } else if c != '\n' && (c == ' ' || c == '\t') {
                    } else {
                        return Err(format!("Unusual string"));
                    }
                },
                THIRD_ARG => {
                    if c != '\n' {
                        targ.push(c);
                    } else {
                        break;
                    }
                },
                _ => return Err(format!("State machine exception"))
            }
        }

        if targ.is_empty() {
            targ = sarg;
            sarg = farg;
            farg = String::from("dword");
        }

        return Ok(VAR::new(farg, sarg, targ));
    }

    fn pstr(&self, args: &str) -> Result<STR, String> {
        let mut farg = String::new();
        let mut sarg = String::new();

        const FIRST_CHAR: u8 = 0;
        const FIRST_ARG: u8 = 1;
        const BLANK: u8 = 2;
        const SECOND_ARG: u8 = 3;

        let mut curr_state = FIRST_CHAR;
        for c in args.chars() {
            match curr_state {
                FIRST_CHAR => {
                    if !c.is_ascii_digit() {
                        farg.push(c);
                        curr_state = FIRST_ARG;
                    } else {
                        return Err(format!("The first character cannot be a number"));
                    }
                },
                FIRST_ARG => {
                    if c != '\t' || c != ' ' {
                        farg.push(c);
                    } else {
                        curr_state = BLANK;
                    }
                },
                BLANK => {
                    if c == '\n' {
                        return Err(format!("No strings available"))
                    } else if c == '\"' {
                        curr_state = SECOND_ARG;
                    } else if c != '\n' && (c == ' ' || c == '\t') {
                    } else {
                        return Err(format!("Unusual string"));
                    }
                },
                SECOND_ARG => {
                    if c != '\"' {
                        sarg.push(c);
                    } else {
                        break;
                    }
                },
                _ => return Err(format!("State machine exception"))
            }
        }
        return Ok(STR::new(farg, sarg));
    }

    fn parr(&self, args: &str) -> Result<ARR, String> {
        let mut farg = String::new();
        let mut sarg = String::new();
        let mut arr_item = vec![];

        const FIRST_CHAR: u8 = 0;
        const FIRST_ARG: u8 = 1;
        const FIRST_BLANK: u8 = 2;
        const SECOND_ARG: u8 = 3;
        const SECOND_BLANK: u8 = 4;
        const THIRD_ARG: u8 = 5;

        let mut curr_state = FIRST_CHAR;
        for c in args.chars() {
            match curr_state {
                FIRST_CHAR => {
                    if !c.is_ascii_digit() {
                        farg.push(c);
                        curr_state = FIRST_ARG;
                    } else {
                        return Err(format!("The first character cannot be a number"));
                    }
                },
                FIRST_ARG => {
                    if c != '\t' || c != ' ' {
                        farg.push(c);
                    } else {
                        curr_state = BLANK;
                    }
                },
                BLANK => {
                    if c == '\n' {
                        return Err(format!("No strings available"))
                    } else if c == '\"' {
                        curr_state = SECOND_ARG;
                    } else if c != '\n' && (c == ' ' || c == '\t') {
                    } else {
                        return Err(format!("Unusual string"));
                    }
                },
                SECOND_ARG => {
                    if c != '\"' {
                        sarg.push(c);
                    } else {
                        break;
                    }
                },
                _ => return Err(format!("State machine exception"))
            }
        }
        return Ok(STR::new(farg, sarg));
    }
}