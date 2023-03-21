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
}