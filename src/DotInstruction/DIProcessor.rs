use std::collections::HashMap;
use tokio;
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
    datas_table: HashMap<String, usize>,
    datas: Vec<u8>
}

impl DotInstrctionsProcessor {
    pub fn new(file: Vec<(usize, String)>) -> DotInstrctionsProcessor {
        DotInstrctionsProcessor {
            file,
            settings_table: SETTINGS,
            datas_table: HashMap::new(),
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

    pub async fn process(&mut self) {
        let mut dip_handles = vec![];
        let mut errors = String::new();

        for (line_num, line) in self.file.clone() {
            let dip = DIProcessor::new(line_num, line);
            dip_handles.push(tokio::spawn(dip.start()))
        }

        for h in dip_handles {
            match h.await.unwrap() {
                Ok((l, v)) => match v {
                    // Here, due to asynchronous operations causing data to be out of order,
                    // a hashmap is needed to record the position of each data
                    DI::AR(d) => {
                        self.datas_table.insert(d.name.clone(), self.datas.len());
                        match d.generateData(l) {
                            Ok(u) => self.datas.append(&mut u),
                            Err(e) => errors += &e
                        }
                    },
                    DI::DE(d) => {},
                    DI::SE(d) => {},
                    DI::ST(d) => {
                        self.datas_table.insert(d.name, self.datas.len());
                        self.datas.append(&mut d.generateData(l));
                    },
                    DI::VA(d) => {
                        self.datas_table.insert(d.name, self.datas.len());
                        match d.generateData(l) {
                            Ok(u) => self.datas.append(&mut u),
                            Err(e) => errors += &e
                        }
                    }
                },
                Err(e) => {}
            }
        }
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

    async fn start(&self) -> Result<(usize, DI), String> {
        if self.line.starts_with(".SET") {
            match self.pset(self.line.trim_start_matches(".SET")) {
                Ok(v) => return Ok((self.line_num, DI::SE(v))),
                Err(e) => return Err(format!("Line: {} - \"{}\"", self.line_num, e))
            };
        } else if self.line.starts_with(".VAR") {
            match self.pvar(self.line.trim_start_matches(".VAR")) {
                Ok(v) => return Ok((self.line_num, DI::VA(v))),
                Err(e) => return Err(format!("Line: {} - \"{}\"", self.line_num, e))
            }
        } else if self.line.starts_with(".STR") {
            match self.pstr(self.line.trim_start_matches(".STR")) {
                Ok(v) => return Ok((self.line_num, DI::ST(v))),
                Err(e) => return Err(format!("Line: {} - \"{}\"", self.line_num, e))
            }
        } else if self.line.starts_with(".ARR") {
            match self.parr(self.line.trim_start_matches(".ARR")) {
                Ok(v) => return Ok((self.line_num, DI::AR(v))),
                Err(e) => return Err(format!("Line: {} - \"{}\"", self.line_num, e))
            }
        } else if self.line.starts_with(".DEF") {
            match self.pdef(self.line.trim_start_matches(".DEF")) {
                Ok(v) => return Ok((self.line_num, DI::DE(v))),
                Err(e) => return Err(format!("Line: {} - \"{}\"", self.line_num, e))
            }
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

        return Ok(ARR::new(farg, sarg, targ));
    }

    fn pdef(&self, args: &str) -> Result<DEF, String> {
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
                        return Err(format!("Unusual string"));
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
        return Ok(DEF {name: farg, value: sarg});
    }
}