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

    pub fn generateData(&self) -> Result<(u32, u8), String> {
        let mut error_infos = String::new();
        let mut error = false;
    }

    fn calcData(&self) -> Result<(u32, u8), UnExceptedErrors> {
        // TODO: how to represent negative numbers
        match self.data_type.as_str() {
            "byte" | "ubyte" => {
                match self.Int() {
                    Ok(v) => {
                        if v > u8::MAX as u32 {
                            return Err(UnExceptedErrors::VOOERE(ValueOutOfExpressionRangeError{value: self.value, v_type: String::from("byte or ubyte")}));
                        } else {
                            return Ok((v, 1));
                        }
                    },
                    Err(e) => {
                        return Err(UnExceptedErrors::PIE(e));
                    }
                }
            },
            "word" | "uword" => {
                match self.Int() {
                    Ok(v) => {
                        if v > u16::MAX as u32 {
                            return Err(UnExceptedErrors::VOOERE(ValueOutOfExpressionRangeError{value: self.value, v_type: String::from("word or uword")}));
                        } else {
                            return Ok((v, 1));
                        }
                    },
                    Err(e) => {
                        return Err(UnExceptedErrors::PIE(e));
                    }
                }
            },
            "dword" | "udword" => {
                match self.Int() {
                    Ok(v) => {
                        if v > u8::MAX as u32 {
                            return Err(UnExceptedErrors::VOOERE(ValueOutOfExpressionRangeError{value: self.value, v_type: String::from("byte")}));
                        } else {
                            return Ok((v, 1));
                        }
                    },
                    Err(e) => {
                        return Err(UnExceptedErrors::PIE(e));
                    }
                }
            },
            _ => {
                match self.Int() {
                    Ok(v) => {
                        if v > u8::MAX as u32 {
                            return Err(UnExceptedErrors::VOOERE(ValueOutOfExpressionRangeError{value: self.value, v_type: String::from("byte")}));
                        } else {
                            return Ok((v, 1));
                        }
                    },
                    Err(e) => {
                        return Err(UnExceptedErrors::PIE(e));
                    }
                }
            },
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