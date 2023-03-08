use core::num::dec2flt::parse::parse_number;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;

#[derive(Debug)]
struct NotAValidRegisterError {
    inst: String,
    register: String
}

impl Error for NotAValidRegisterError {}
impl Display for NotAValidRegisterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "In instruction {}, {} is not a valid register!", self.inst, self.register)
    }
}

const all_register: HashMap<String, u8> = HashMap::from_iter(vec![
    (String::from("ZERO"), 0_u8), (String::from("PC"), 41_u8),
    // Part A Register
    (String::from("A1"), 1_u8), (String::from("A2"), 2_u8),
    (String::from("A3"), 3_u8), (String::from("A4"), 4_u8),
    (String::from("AR1"), 5_u8), (String::from("AR2"), 6_u8),
    (String::from("AR3"), 7_u8), (String::from("ASS"), 8_u8),
    (String::from("ASP"), 9_u8), (String::from("ADS"), 10_u8),
    // Part B Register
    (String::from("B1"), 11_u8), (String::from("B2"), 12_u8),
    (String::from("B3"), 13_u8), (String::from("B4"), 14_u8),
    (String::from("BR1"), 15_u8), (String::from("BR2"), 16_u8),
    (String::from("BR3"), 17_u8), (String::from("BSS"), 18_u8),
    (String::from("BSP"), 19_u8), (String::from("BDS"), 20_u8),
    // Part C Register
    (String::from("C1"), 21_u8), (String::from("C2"), 22_u8),
    (String::from("C3"), 23_u8), (String::from("C4"), 24_u8),
    (String::from("CR1"), 25_u8), (String::from("CR2"), 26_u8),
    (String::from("CR3"), 27_u8), (String::from("CSS"), 28_u8),
    (String::from("CSP"), 29_u8), (String::from("CDS"), 30_u8),
    // Part D Register
    (String::from("D1"), 31_u8), (String::from("D2"), 32_u8),
    (String::from("D3"), 33_u8), (String::from("D4"), 34_u8),
    (String::from("DR1"), 35_u8), (String::from("DR2"), 36_u8),
    (String::from("DR3"), 37_u8), (String::from("DSS"), 38_u8),
    (String::from("DSP"), 39_u8), (String::from("DDS"), 40_u8)
    ]);

const nonzero_register: HashMap<String, u8> = HashMap::from_iter(vec![
    (String::from("PC"), 41_u8),
    // Part A Register
    (String::from("A1"), 1_u8), (String::from("A2"), 2_u8),
    (String::from("A3"), 3_u8), (String::from("A4"), 4_u8),
    (String::from("AR1"), 5_u8), (String::from("AR2"), 6_u8),
    (String::from("AR3"), 7_u8), (String::from("ASS"), 8_u8),
    (String::from("ASP"), 9_u8), (String::from("ADS"), 10_u8),
    // Part B Register
    (String::from("B1"), 11_u8), (String::from("B2"), 12_u8),
    (String::from("B3"), 13_u8), (String::from("B4"), 14_u8),
    (String::from("BR1"), 15_u8), (String::from("BR2"), 16_u8),
    (String::from("BR3"), 17_u8), (String::from("BSS"), 18_u8),
    (String::from("BSP"), 19_u8), (String::from("BDS"), 20_u8),
    // Part C Register
    (String::from("C1"), 21_u8), (String::from("C2"), 22_u8),
    (String::from("C3"), 23_u8), (String::from("C4"), 24_u8),
    (String::from("CR1"), 25_u8), (String::from("CR2"), 26_u8),
    (String::from("CR3"), 27_u8), (String::from("CSS"), 28_u8),
    (String::from("CSP"), 29_u8), (String::from("CDS"), 30_u8),
    // Part D Register
    (String::from("D1"), 31_u8), (String::from("D2"), 32_u8),
    (String::from("D3"), 33_u8), (String::from("D4"), 34_u8),
    (String::from("DR1"), 35_u8), (String::from("DR2"), 36_u8),
    (String::from("DR3"), 37_u8), (String::from("DSS"), 38_u8),
    (String::from("DSP"), 39_u8), (String::from("DDS"), 40_u8)
    ]);

const nonpc_register: HashMap<String, u8> = HashMap::from_iter(vec![
    (String::from("ZERO"), 0_u8),
    // Part A Register
    (String::from("A1"), 1_u8), (String::from("A2"), 2_u8),
    (String::from("A3"), 3_u8), (String::from("A4"), 4_u8),
    (String::from("AR1"), 5_u8), (String::from("AR2"), 6_u8),
    (String::from("AR3"), 7_u8), (String::from("ASS"), 8_u8),
    (String::from("ASP"), 9_u8), (String::from("ADS"), 10_u8),
    // Part B Register
    (String::from("B1"), 11_u8), (String::from("B2"), 12_u8),
    (String::from("B3"), 13_u8), (String::from("B4"), 14_u8),
    (String::from("BR1"), 15_u8), (String::from("BR2"), 16_u8),
    (String::from("BR3"), 17_u8), (String::from("BSS"), 18_u8),
    (String::from("BSP"), 19_u8), (String::from("BDS"), 20_u8),
    // Part C Register
    (String::from("C1"), 21_u8), (String::from("C2"), 22_u8),
    (String::from("C3"), 23_u8), (String::from("C4"), 24_u8),
    (String::from("CR1"), 25_u8), (String::from("CR2"), 26_u8),
    (String::from("CR3"), 27_u8), (String::from("CSS"), 28_u8),
    (String::from("CSP"), 29_u8), (String::from("CDS"), 30_u8),
    // Part D Register
    (String::from("D1"), 31_u8), (String::from("D2"), 32_u8),
    (String::from("D3"), 33_u8), (String::from("D4"), 34_u8),
    (String::from("DR1"), 35_u8), (String::from("DR2"), 36_u8),
    (String::from("DR3"), 37_u8), (String::from("DSS"), 38_u8),
    (String::from("DSP"), 39_u8), (String::from("DDS"), 40_u8)
    ]);

const general_register: HashMap<String, u8> = HashMap::from_iter(vec![
    // Part A Register
    (String::from("A1"), 1_u8), (String::from("A2"), 2_u8),
    (String::from("A3"), 3_u8), (String::from("A4"), 4_u8),
    (String::from("AR1"), 5_u8), (String::from("AR2"), 6_u8),
    (String::from("AR3"), 7_u8), (String::from("ASS"), 8_u8),
    (String::from("ASP"), 9_u8), (String::from("ADS"), 10_u8),
    // Part B Register
    (String::from("B1"), 11_u8), (String::from("B2"), 12_u8),
    (String::from("B3"), 13_u8), (String::from("B4"), 14_u8),
    (String::from("BR1"), 15_u8), (String::from("BR2"), 16_u8),
    (String::from("BR3"), 17_u8), (String::from("BSS"), 18_u8),
    (String::from("BSP"), 19_u8), (String::from("BDS"), 20_u8),
    // Part C Register
    (String::from("C1"), 21_u8), (String::from("C2"), 22_u8),
    (String::from("C3"), 23_u8), (String::from("C4"), 24_u8),
    (String::from("CR1"), 25_u8), (String::from("CR2"), 26_u8),
    (String::from("CR3"), 27_u8), (String::from("CSS"), 28_u8),
    (String::from("CSP"), 29_u8), (String::from("CDS"), 30_u8),
    // Part D Register
    (String::from("D1"), 31_u8), (String::from("D2"), 32_u8),
    (String::from("D3"), 33_u8), (String::from("D4"), 34_u8),
    (String::from("DR1"), 35_u8), (String::from("DR2"), 36_u8),
    (String::from("DR3"), 37_u8), (String::from("DSS"), 38_u8),
    (String::from("DSP"), 39_u8), (String::from("DDS"), 40_u8)
    ]);

struct Instruction {
    inst_type: String,
    inst_name: String,
    op_code: u16,
    op_code_lenth: u8,
    // binary register label
    source_register_label: Option<u8>,
    target_register_label: Option<u8>,
    // binary immediate number
    binary_immediate_number: Option<u32>,
    source_register_lut: HashMap<String, u8>,
    target_register_lut: HashMap<String, u8>
}

impl Instruction {

}

struct LOAD {
    inst_type: String,
    inst_name: String,
    op_code: u16,
    op_code_lenth: u8,
    // binary register label
    source_register_label: Option<u8>,
    target_register_label: Option<u8>,
    // binary immediate number
    binary_immediate_number: Option<u32>,
    source_register_lut: HashMap<String, u8>,
    target_register_lut: HashMap<String, u8>,
    fsource_register_label: Option<u8>,
    ssource_register_label: Option<u8>,
    fsource_register_lut: HashMap<String, u8>,
    ssource_register_lut: HashMap<String, u8>
}

impl LOAD {
    fn new(inst_type: String, inst_name: String, op_code: u16) -> LOAD {
        LOAD {
            inst_type,
            inst_name,
            op_code,
            op_code_lenth: 10,
            source_register_label: None,
            target_register_label: None,
            binary_immediate_number: None,
            source_register_lut: all_register,
            target_register_lut: all_register,
            fsource_register_label: None,
            ssource_register_label: None,
            fsource_register_lut: all_register,
            ssource_register_lut: all_register
        }
    }

    fn setSourceRegister(&mut self, source_register: String) -> Result<(), NotAValidRegisterError> {
        match self.source_register_lut.get(&source_register) {
            None => return Err(NotAValidRegisterError{inst: self.inst_name, register: source_register}),
            Some(v) => self.source_register_label = Some(v.clone())
        };

        Ok(())
    }

    fn setTargetRegister(&mut self, target_register: String) -> Result<(), NotAValidRegisterError> {
        match self.target_register_lut.get(&target_register) {
            None => return Err(NotAValidRegisterError{inst: self.inst_name, register: target_register}),
            Some(v) => self.target_register_label = Some(v.clone())
        };

        Ok(())
    }

    fn setImmediateNumber(&mut self, immediate_number: String) -> Result<(), ParseIntError>{
        if immediate_number.starts_with("hex") {
            self.binary_immediate_number = match u32::from_str_radix(immediate_number.trim_start_matches("hex"), 16) {
                Ok(v) => Some(v),
                Err(e) => return Err(e)
            };
        } else if immediate_number.starts_with("oct") {
            self.binary_immediate_number = match u32::from_str_radix(immediate_number.trim_start_matches("oct"), 8) {
                Ok(v) => Some(v),
                Err(e) => return Err(e)
            };
        } else if immediate_number.starts_with("bin") {
            self.binary_immediate_number = match u32::from_str_radix(immediate_number.trim_start_matches("bin"), 2) {
                Ok(v) => Some(v),
                Err(e) => return Err(e)
            };
        } else {
            self.binary_immediate_number = match u32::from_str_radix(immediate_number.as_str(), 10) {
                Ok(v) => Some(v),
                Err(e) => return Err(e)
            };
        }

        Ok(())
    }

    fn setFSourceRegister(&mut self, first_source_register: String) -> Result<(), NotAValidRegisterError> {
        match self.fsource_register_lut.get(&first_source_register) {
            None => return Err(NotAValidRegisterError{inst: self.inst_name, register: first_source_register}),
            Some(v) => self.fsource_register_label = Some(v.clone())
        };

        Ok(())
    }

    fn setSSourceRegister(&mut self, second_source_register: String) -> Result<(), NotAValidRegisterError> {
        match self.ssource_register_lut.get(&second_source_register) {
            None => return Err(NotAValidRegisterError{inst: self.inst_name, register: second_source_register}),
            Some(v) => self.ssource_register_label = Some(v.clone())
        };

        Ok(())
    }

    fn generateCode(&mut self, target_register: String, oargs: Vec<String>) {
        match self.setTargetRegister(target_register) {
            Ok(_) => (),
            // TODO: How to handle exceptions?
            Err(e) => ()
        };
    }
}