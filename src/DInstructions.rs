use std::collections::HashMap;
use std::num::ParseIntError;
use std::error::Error;
use std::fmt::{Display, Formatter};


#[derive(Debug)]
struct UnparseableStringError {
    inst: String,
    value: String
}

impl Error for UnparseableStringError {}
impl Display for UnparseableStringError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "In instruction {}, {} cannot be parsed as a valid value", self.inst, self.value)
    }
}

#[derive(Debug)]
struct ValueOutOfExpressionRangeError {
    value: String,
    v_type: String
}

impl Error for ValueOutOfExpressionRangeError {}
impl Display for ValueOutOfExpressionRangeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "The value {} used exceeds the limit of {}", self.value, self.v_type)
    }
}

enum UnExceptedErrors {
    USE(UnparseableStringError),
    VOOERE(ValueOutOfExpressionRangeError),
    PIE(ParseIntError),
    JS(String)
}

// Settings table
const SETTINGS: HashMap<String, Setting_item> = HashMap::from_iter(vec![
    (String::from("S_CODE_SA"), Setting_item::I(0x0)),
    (String::from("S_DATA_SA"), Setting_item::I(0x2000)),
    (String::from("S_STACK_SA"), Setting_item::I(0x3000))
]);

enum Setting_item {
    B(bool),
    I(u32)
}

// Requires no spaces before and after all values

pub struct SET {
    setting_item: String,
    value: String,
    no_value_setting: bool
}

impl SET {
    pub fn new(si: String, v: String, nvs: bool) -> SET {
        SET {
            setting_item: si,
            value: v,
            no_value_setting: nvs
        }
    }

    pub fn setTable(&self, line_num: usize) -> Result<(), String> {
        let mut error_infos = String::new();
        let mut error = false;

        match SETTINGS.get(&self.setting_item) {
            None => error_infos += &format!("Line: {} - An illegal setting item {} is used in instruction {}\n", line_num, self.setting_item,String::from("SET")),
            Some(v) => match v {
                Setting_item::B(b) => {
                    match self.Bool() {
                        Ok(v) => {SETTINGS.insert(self.setting_item, Setting_item::B(v)); ()},
                        Err(e) => {
                            error = true;
                            error_infos += &format!("Line: {} - {}\n", line_num, e);
                        }
                    }
                },
                Setting_item::I(i) => {
                    match self.Int() {
                        Ok(v) => {SETTINGS.insert(self.setting_item, Setting_item::I(v)); ()},
                        Err(e) => {
                            error = true;
                            error_infos += &format!("Line: {} - {}\n", line_num, e);
                        }
                    }
                },
            }
        }

        if error {
            return Err(error_infos);
        } else {
            return Ok(());
        }
    }

    fn Bool(&self) -> Result<bool, UnparseableStringError>{
        match self.value.as_str() {
            "false" => Ok(false),
            "true" => Ok(true),
            "FALSE" => Ok(false),
            "TRUE" => Ok(true),
            _ => return Err(UnparseableStringError{inst: String::from("SET"), value: self.value})
        }
    }

    fn Int(&self) -> Result<u32, ParseIntError> {
        if self.value.starts_with("hex") {
            return u32::from_str_radix(self.value.trim_start_matches("hex"), 16);
        } else if self.value.starts_with("oct") {
            return u32::from_str_radix(self.value.trim_start_matches("oct"), 8);
        } else if self.value.starts_with("bin") {
            return u32::from_str_radix(self.value.trim_start_matches("bin"), 2);
        } else {
            return u32::from_str_radix(self.value.as_str(), 10);
        }
    }
}

pub struct VAR {
    name: String,
    data_type: String,
    value: String
}

impl VAR {
    pub fn new(name: String, data_type: String, value: String) -> VAR {
        VAR {
            name,
            data_type,
            value
        }
    }

    pub fn generateData(&self, line_num: usize) -> Result<Vec<u8>, String> {
        let mut error_infos = String::new();
        let mut error = false;

        let r = match self.calcData() {
            Ok(v) => v,
            Err(e) => {
                error = true;
                match e {
                    UnExceptedErrors::JS(ev) => error_infos += &format!("Line: {} - {}", line_num, ev),
                    UnExceptedErrors::PIE(ev) => error_infos += &format!("Line: {} - {}", line_num, ev),
                    UnExceptedErrors::USE(ev) => error_infos += &format!("Line: {} - {}", line_num, ev),
                    UnExceptedErrors::VOOERE(ev) => error_infos += &format!("Line: {} - {}", line_num, ev)
                }
                vec![]
            }
        };

        if error {
            return Err(error_infos);
        } else {
            return Ok(r)
        }
    }

    fn calcData(&self) -> Result<Vec<u8>, UnExceptedErrors> {
        let mut radix = 10;
        let mut src_str = "";
        if self.value.starts_with("hex") {
            radix = 16;
            src_str = self.value.trim_start_matches("hex");
        } else if self.value.starts_with("oct") {
            radix = 8;
            src_str = self.value.trim_start_matches("oct");
        } else if self.value.starts_with("bin") {
            radix = 2;
            src_str = self.value.trim_start_matches("bin");
        } else {
            radix = 10;
            src_str = self.value.as_str();
        }

        let max_value = match self.data_type.as_str() {
            "byte" => u8::MAX as u32,
            "word" => u16::MAX as u32,
            "dword" | _ => u32::MAX as u32
        };

        match u32::from_str_radix(src_str, radix) {
            Ok(v) => {
                if v > max_value {
                    ???
                }
            }
            Err(e) => return Err(UnExceptedErrors::PIE(e))
        }
    }
}

struct STR {
    name: String,
    value: String
}

impl STR {
    pub fn new(name: String, value: String) -> STR {
        STR {
            name,
            // with out """
            value
        }
    }

    pub fn generateData(&self, line_num: usize) -> Vec<u8> {
        return self.value.into_bytes();
    }
}

struct ARR {
    name: String,
    data_type: String,
    value: String
}

impl ARR {
    pub fn new(name: String, data_type: String, value: String) -> ARR {
        ARR {
            name,
            data_type,
            value
        }
    }

    pub fn generateData(&self) {
        let mut error = false;
        let mut error_infos = String::new();

        let r = self.value.chars().map(|x| match self.toInt(x.trim()) {
            Ok(v) => v,
            Err(e) => 
        });
    }

    fn toInt(&self, v: &str) -> Result<U, UnExceptedErrors> {
        let mut radix = 10;
        let mut src_str = "";
        if v.starts_with("hex") {
            radix = 16;
            src_str = v.trim_start_matches("hex");
        } else if v.starts_with("oct") {
            radix = 8;
            src_str = v.trim_start_matches("oct");
        } else if v.starts_with("bin") {
            radix = 2;
            src_str = v.trim_start_matches("bin");
        } else {
            radix = 10;
            src_str = v;
        }

        match self.data_type.as_str() {
            "byte" => {
                match u32::from_str_radix(src_str, radix) {
                    Ok(v) => {
                        if v > u8::MAX as u32 {
                            return Err(UnExceptedErrors::VOOERE(ValueOutOfExpressionRangeError { value: self.value, v_type: String::from("byte") }));
                        } else {
                            return Ok(U::b(v as u8))
                        }
                    },
                    Err(e) => return Err(UnExceptedErrors::PIE(e))
                };
            },
            "word" => {
                match u32::from_str_radix(src_str, radix) {
                    Ok(v) => {
                        if v > u16::MAX as u32 {
                            return Err(UnExceptedErrors::VOOERE(ValueOutOfExpressionRangeError { value: self.value, v_type: String::from("word") }));
                        } else {
                            return Ok(U::w(v as u16))
                        }
                    },
                    Err(e) => return Err(UnExceptedErrors::PIE(e))
                };
            },
            "dword" | _ => {
                match u32::from_str_radix(src_str, radix) {
                    Ok(v) => {
                        if v > u32::MAX {
                            return Err(UnExceptedErrors::VOOERE(ValueOutOfExpressionRangeError { value: self.value, v_type: String::from("dword") }));
                        } else {
                            return Ok(U::d(v))
                        }
                    },
                    Err(e) => return Err(UnExceptedErrors::PIE(e))
                };
            }
        }
    }
}