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
        write!(f, "In instruction {}, {} is not a valid register", self.inst, self.register)
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

pub struct LOAD {
    inst_type: String,
    inst_name: String,
    op_code: u32,
    // binary register label
    target_register_label: u32,
    target_register_start_bit: u8,
    // binary immediate number
    immediate_number: u32,
    immediate_number_start_bit: u8,
    fsource_register_label: u32,
    ssource_register_label: u32,
    fsource_register_start_bit: u8,
    ssource_register_start_bit: u8
}

impl LOAD {
    fn new(inst_type: String, inst_name: String, op_code: u16, trsb: u8, insb: u8, fsrsb: u8, ssrsb: u8) -> LOAD {
        LOAD {
            inst_type,
            inst_name,
            op_code: (op_code as u32) << 22,
            target_register_label: 0,
            target_register_start_bit: trsb,
            immediate_number: 0,
            immediate_number_start_bit: insb,
            fsource_register_label: 0,
            ssource_register_label: 0,
            fsource_register_start_bit: fsrsb,
            ssource_register_start_bit: ssrsb,
        }
    }

    fn setTargetRegister(&mut self, target_register: String) -> Result<(), NotAValidRegisterError> {
        match general_register.get(&target_register) {
            None => return Err(NotAValidRegisterError{inst: self.inst_name, register: target_register}),
            Some(v) => self.target_register_label = (*v as u32) << self.target_register_start_bit
        };

        Ok(())
    }

    fn setImmediateNumber(&mut self, immediate_number: String) -> Result<(), ParseIntError>{
        if immediate_number.starts_with("hex") {
            self.immediate_number = match u32::from_str_radix(immediate_number.trim_start_matches("hex"), 16) {
                Ok(v) => v << self.immediate_number_start_bit,
                Err(e) => return Err(e)
            };
        } else if immediate_number.starts_with("oct") {
            self.immediate_number = match u32::from_str_radix(immediate_number.trim_start_matches("oct"), 8) {
                Ok(v) => v << self.immediate_number_start_bit,
                Err(e) => return Err(e)
            };
        } else if immediate_number.starts_with("bin") {
            self.immediate_number = match u32::from_str_radix(immediate_number.trim_start_matches("bin"), 2) {
                Ok(v) => v << self.immediate_number_start_bit,
                Err(e) => return Err(e)
            };
        } else {
            self.immediate_number = match u32::from_str_radix(immediate_number.as_str(), 10) {
                Ok(v) => v << self.immediate_number_start_bit,
                Err(e) => return Err(e)
            };
        }

        Ok(())
    }

    fn setFSourceRegister(&mut self, first_source_register: String) -> Result<(), NotAValidRegisterError> {
        match all_register.get(&first_source_register) {
            None => return Err(NotAValidRegisterError{inst: self.inst_name, register: first_source_register}),
            Some(v) => self.fsource_register_label = (*v as u32) << self.fsource_register_start_bit
        };

        Ok(())
    }

    fn setSSourceRegister(&mut self, second_source_register: String) -> Result<(), NotAValidRegisterError> {
        match all_register.get(&second_source_register) {
            None => return Err(NotAValidRegisterError{inst: self.inst_name, register: second_source_register}),
            Some(v) => self.ssource_register_label = (*v as u32) << self.ssource_register_start_bit
        };

        Ok(())
    }

    pub fn generateCode(&mut self, line_num: usize, target_register: String, immediate_number: Option<String>, fsource_register: Option<String>, ssource_register: Option<String>) -> Result<u32, String>{
        /// Considering the efficiency of the compiler, here is a method that consumes more memory and improves compilation speed.
        /// Each instruction is processed by a coroutine, and at the same time, two memory spaces are opened for saving the processing results,
        /// one is normal and the other is abnormal, and the space is consistent with the number of instructions in the compiled file.

        let mut error_infos = String::new();
        let mut error = false;

        match self.setTargetRegister(target_register) {
            Ok(_) => (),
            Err(e) => {
                error = true;
                error_infos += &format!("Line: {} - {}\n", line_num, e)
            }
        };

        match immediate_number {
            None => (),
            Some(v) => match self.setImmediateNumber(v) {
                Ok(_) => (),
                Err(e) => {
                    error = true;
                    error_infos += &format!("Line: {} - An error occurred while parsing the immediate number: {}\n", line_num, e)
                }
            }
        }

        match fsource_register {
            None => (),
            Some(v) => match self.setFSourceRegister(v) {
                Ok(_) => (),
                Err(e) => {
                    error = true;
                    error_infos += &format!("Line: {} - {}\n", line_num, e)
                }
            }
        }

        match ssource_register {
            None => (),
            Some(v) => match self.setSSourceRegister(v) {
                Ok(_) => (),
                Err(e) => {
                    error = true;
                    error_infos += &format!("Line: {} - {}\n", line_num, e)
                }
            }
        }

        if error {
            return Err(error_infos);
        } else {
            match self.argsToBinaryCode() {
                Ok(v) => return Ok(v),
                Err(_) => {
                    error_infos += &format!("Line: {} - Binary opcode generation error, info: opcode: {:320b}, imd_num: {:320b}, t_r: {:320b}, fs_r: {:320b}, ss_r: {:320b}\n", line_num, self.op_code,self.immediate_number, self.target_register_label, self.fsource_register_label, self.ssource_register_label);
                    return Err(error_infos);
                }
            }
        }
    }

    fn argsToBinaryCode(&self) -> Result<u32, ()> {
        /// A easy way to check
        let a = self.op_code + self.target_register_label + self.ssource_register_label + self.fsource_register_label + self.immediate_number;
        let b = self.op_code | self.target_register_label | self.ssource_register_label | self.fsource_register_label + self.immediate_number;
        if a == b {
            return Ok(a);
        } else {
            return Err(());
        }
    }
}

pub struct STORE {
    inst_type: String,
    inst_name: String,
    op_code: u32,
    ftarget_register_label: u32,
    starget_register_label: u32,
    ftarget_register_start_bit: u8,
    starget_register_start_bit: u8,
    source_register_label: u32,
    source_register_start_bit: u8
}

impl STORE {
    pub fn new(inst_type: String, inst_name: String, op_code: u16) -> STORE {
        STORE {
            inst_type,
            inst_name,
            op_code: (op_code as u32) << 22,
            ftarget_register_label: 0,
            starget_register_label: 0,
            ftarget_register_start_bit: 6,
            starget_register_start_bit: 0,
            source_register_label: 0,
            source_register_start_bit: 16
        }
    }

    fn setFTargetRegister(&mut self, first_target_register: String) -> Result<(), NotAValidRegisterError> {
        match general_register.get(&first_target_register) {
            None => return Err(NotAValidRegisterError{inst: self.inst_name, register: first_target_register}),
            Some(v) => self.ftarget_register_label = (*v as u32) << self.ftarget_register_start_bit
        };

        Ok(())
    }

    fn setSTargetRegister(&mut self, second_target_register: String) -> Result<(), NotAValidRegisterError> {
        match general_register.get(&second_target_register) {
            None => return Err(NotAValidRegisterError{inst: self.inst_name, register: second_target_register}),
            Some(v) => self.starget_register_label = (*v as u32) << self.starget_register_start_bit
        };

        Ok(())
    }

    fn setSourceRegister(&mut self, source_register: String) -> Result<(), NotAValidRegisterError> {
        match all_register.get(&source_register) {
            None => return Err(NotAValidRegisterError{inst: self.inst_name, register: source_register}),
            Some(v) => self.source_register_label = (*v as u32) << self.source_register_start_bit
        };

        Ok(())
    }

    pub fn generateCode(&mut self, line_num: usize, ftarget_register: String, starget_register: String, source_register: String) -> Result<u32, String>{
        /// Considering the efficiency of the compiler, here is a method that consumes more memory and improves compilation speed.
        /// Each instruction is processed by a coroutine, and at the same time, two memory spaces are opened for saving the processing results,
        /// one is normal and the other is abnormal, and the space is consistent with the number of instructions in the compiled file.

        let mut error_infos = String::new();
        let mut error = false;

        match self.setFTargetRegister(ftarget_register) {
            Ok(_) => (),
            Err(e) => {
                error = true;
                error_infos += &format!("Line: {} - {}\n", line_num, e)
            }
        };

        match self.setSTargetRegister(starget_register) {
            Ok(_) => (),
            Err(e) => {
                error = true;
                error_infos += &format!("Line: {} - {}\n", line_num, e)
            }
        };

        match self.setSourceRegister(source_register) {
            Ok(_) => (),
            Err(e) => {
                error = true;
                error_infos += &format!("Line: {} - {}\n", line_num, e)
            }
        }

        if error {
            return Err(error_infos);
        } else {
            match self.argsToBinaryCode() {
                Ok(v) => return Ok(v),
                Err(_) => {
                    error_infos += &format!("Line: {} - Binary opcode generation error, info: opcode: {:320b}, ft_r: {:320b}, st_r: {:320b}, s_r: {:320b}\n", line_num, self.op_code, self.ftarget_register_label, self.starget_register_label, self.source_register_label);
                    return Err(error_infos);
                }
            }
        }
    }

    fn argsToBinaryCode(&self) -> Result<u32, ()> {
        /// A easy way to check
        let a = self.op_code + self.ftarget_register_label + self.starget_register_label + self.source_register_label;
        let b = self.op_code | self.ftarget_register_label | self.starget_register_label | self.source_register_label;
        if a == b {
            return Ok(a);
        } else {
            return Err(());
        }
    }
}

pub struct MOVE {
    inst_type: String,
    inst_name: String,
    op_code: u32,
    target_register_label: u32,
    target_register_start_bit: u8,
    source_register_label: u32,
    source_register_start_bit: u8
}

impl MOVE {
    pub fn new(inst_type: String, inst_name: String, op_code: u16) -> MOVE {
        MOVE {
            inst_type,
            inst_name,
            op_code: (op_code as u32) << 22,
            target_register_label: 0,
            target_register_start_bit: 16,
            source_register_label: 0,
            source_register_start_bit: 0
        }
    }

    fn setTargetRegister(&mut self, target_register: String) -> Result<(), NotAValidRegisterError> {
        match general_register.get(&target_register) {
            None => return Err(NotAValidRegisterError{inst: self.inst_name, register: target_register}),
            Some(v) => self.target_register_label = (*v as u32) << self.target_register_start_bit
        };

        Ok(())
    }

    fn setSourceRegister(&mut self, source_register: String) -> Result<(), NotAValidRegisterError> {
        match all_register.get(&source_register) {
            None => return Err(NotAValidRegisterError{inst: self.inst_name, register: source_register}),
            Some(v) => self.source_register_label = (*v as u32) << self.source_register_start_bit
        };

        Ok(())
    }

    pub fn generateCode(&mut self, line_num: usize, target_register: String, source_register: String) -> Result<u32, String>{
        /// Considering the efficiency of the compiler, here is a method that consumes more memory and improves compilation speed.
        /// Each instruction is processed by a coroutine, and at the same time, two memory spaces are opened for saving the processing results,
        /// one is normal and the other is abnormal, and the space is consistent with the number of instructions in the compiled file.

        let mut error_infos = String::new();
        let mut error = false;

        match self.setTargetRegister(target_register) {
            Ok(_) => (),
            Err(e) => {
                error = true;
                error_infos += &format!("Line: {} - {}\n", line_num, e)
            }
        };

        match self.setSourceRegister(source_register) {
            Ok(_) => (),
            Err(e) => {
                error = true;
                error_infos += &format!("Line: {} - {}\n", line_num, e)
            }
        }

        if error {
            return Err(error_infos);
        } else {
            match self.argsToBinaryCode() {
                Ok(v) => return Ok(v),
                Err(_) => {
                    error_infos += &format!("Line: {} - Binary opcode generation error, info: opcode: {:320b}, t_r: {:320b}, s_r: {:320b}\n", line_num, self.op_code, self.target_register_label, self.source_register_label);
                    return Err(error_infos);
                }
            }
        }
    }

    fn argsToBinaryCode(&self) -> Result<u32, ()> {
        /// A easy way to check
        let a = self.op_code + self.target_register_label + self.source_register_label;
        let b = self.op_code | self.target_register_label | self.source_register_label;

        if a == b {
            return Ok(a);
        } else {
            return Err(());
        }
    }
}

pub struct INTEGER {
    inst_type: String,
    inst_name: String,
    op_code: u32,
    // binary register label
    target_register_label: u32,
    target_register_start_bit: u8,
    // binary immediate number
    immediate_number: u32,
    immediate_number_start_bit: u8,
    source_register_label: u32,
    asource_register_label: u32,
    source_register_start_bit: u8,
    asource_register_start_bit: u8
}

impl INTEGER {
    fn new(inst_type: String, inst_name: String, op_code: u16, trsb: u8, insb: u8, srsb: u8, asrsb: u8) -> INTEGER {
        INTEGER {
            inst_type,
            inst_name,
            op_code: (op_code as u32) << 22,
            target_register_label: 0,
            target_register_start_bit: trsb,
            immediate_number: 0,
            immediate_number_start_bit: insb,
            source_register_label: 0,
            asource_register_label: 0,
            source_register_start_bit: srsb,
            asource_register_start_bit: asrsb,
        }
    }

    fn setTargetRegister(&mut self, target_register: String) -> Result<(), NotAValidRegisterError> {
        match general_register.get(&target_register) {
            None => return Err(NotAValidRegisterError{inst: self.inst_name, register: target_register}),
            Some(v) => self.target_register_label = (*v as u32) << self.target_register_start_bit
        };

        Ok(())
    }

    fn setImmediateNumber(&mut self, immediate_number: String) -> Result<(), ParseIntError>{
        if immediate_number.starts_with("hex") {
            self.immediate_number = match u32::from_str_radix(immediate_number.trim_start_matches("hex"), 16) {
                Ok(v) => v << self.immediate_number_start_bit,
                Err(e) => return Err(e)
            };
        } else if immediate_number.starts_with("oct") {
            self.immediate_number = match u32::from_str_radix(immediate_number.trim_start_matches("oct"), 8) {
                Ok(v) => v << self.immediate_number_start_bit,
                Err(e) => return Err(e)
            };
        } else if immediate_number.starts_with("bin") {
            self.immediate_number = match u32::from_str_radix(immediate_number.trim_start_matches("bin"), 2) {
                Ok(v) => v << self.immediate_number_start_bit,
                Err(e) => return Err(e)
            };
        } else {
            self.immediate_number = match u32::from_str_radix(immediate_number.as_str(), 10) {
                Ok(v) => v << self.immediate_number_start_bit,
                Err(e) => return Err(e)
            };
        }

        Ok(())
    }

    fn setSourceRegister(&mut self, source_register: String) -> Result<(), NotAValidRegisterError> {
        match all_register.get(&source_register) {
            None => return Err(NotAValidRegisterError{inst: self.inst_name, register: source_register}),
            Some(v) => self.source_register_label = (*v as u32) << self.source_register_start_bit
        };

        Ok(())
    }

    fn setASourceRegister(&mut self, another_source_register: String) -> Result<(), NotAValidRegisterError> {
        match all_register.get(&another_source_register) {
            None => return Err(NotAValidRegisterError{inst: self.inst_name, register: another_source_register}),
            Some(v) => self.asource_register_label = (*v as u32) << self.asource_register_start_bit
        };

        Ok(())
    }

    pub fn generateCode(&mut self, line_num: usize, target_register: String, immediate_number: Option<String>, source_register: String, asource_register: Option<String>) -> Result<u32, String>{
        /// Considering the efficiency of the compiler, here is a method that consumes more memory and improves compilation speed.
        /// Each instruction is processed by a coroutine, and at the same time, two memory spaces are opened for saving the processing results,
        /// one is normal and the other is abnormal, and the space is consistent with the number of instructions in the compiled file.

        let mut error_infos = String::new();
        let mut error = false;

        match self.setTargetRegister(target_register) {
            Ok(_) => (),
            Err(e) => {
                error = true;
                error_infos += &format!("Line: {} - {}\n", line_num, e)
            }
        };

        match immediate_number {
            None => (),
            Some(v) => match self.setImmediateNumber(v) {
                Ok(_) => (),
                Err(e) => {
                    error = true;
                    error_infos += &format!("Line: {} - An error occurred while parsing the immediate number: {}\n", line_num, e)
                }
            }
        }

        match self.setSourceRegister(source_register) {
            Ok(_) => (),
            Err(e) => {
                error = true;
                error_infos += &format!("Line: {} - {}\n", line_num, e)
            }
        }

        match asource_register {
            None => (),
            Some(v) => match self.setASourceRegister(v) {
                Ok(_) => (),
                Err(e) => {
                    error = true;
                    error_infos += &format!("Line: {} - {}\n", line_num, e)
                }
            }
        }

        if error {
            return Err(error_infos);
        } else {
            match self.argsToBinaryCode() {
                Ok(v) => return Ok(v),
                Err(_) => {
                    error_infos += &format!("Line: {} - Binary opcode generation error, info: opcode: {:320b}, imd_num: {:320b}, t_r: {:320b}, s_r: {:320b}, as_r: {:320b}\n", line_num, self.op_code,self.immediate_number, self.target_register_label, self.source_register_label, self.asource_register_label);
                    return Err(error_infos);
                }
            }
        }
    }

    fn argsToBinaryCode(&self) -> Result<u32, ()> {
        /// A easy way to check
        let a = self.op_code + self.target_register_label + self.source_register_label + self.asource_register_label + self.immediate_number;
        let b = self.op_code | self.target_register_label | self.source_register_label | self.asource_register_label + self.immediate_number;
        if a == b {
            return Ok(a);
        } else {
            return Err(());
        }
    }
}

pub struct BRANCH {
    inst_type: String,
    inst_name: String,
    op_code: u32,
    // binary register label
    target_register_label: u32,
    target_register_start_bit: u8,
    // binary immediate number
    immediate_number: u32,
    immediate_number_start_bit: u8,
    source_register_label: u32,
    asource_register_label: u32,
    source_register_start_bit: u8,
    asource_register_start_bit: u8
}

impl BRANCH {
    fn new(inst_type: String, inst_name: String, op_code: u16, trsb: u8, insb: u8, srsb: u8, asrsb: u8) -> BRANCH {
        BRANCH {
            inst_type,
            inst_name,
            op_code: (op_code as u32) << 22,
            target_register_label: 0,
            target_register_start_bit: trsb,
            immediate_number: 0,
            immediate_number_start_bit: insb,
            source_register_label: 0,
            asource_register_label: 0,
            source_register_start_bit: srsb,
            asource_register_start_bit: asrsb,
        }
    }

    fn setTargetRegister(&mut self, target_register: String) -> Result<(), NotAValidRegisterError> {
        match general_register.get(&target_register) {
            None => return Err(NotAValidRegisterError{inst: self.inst_name, register: target_register}),
            Some(v) => self.target_register_label = (*v as u32) << self.target_register_start_bit
        };

        Ok(())
    }

    fn setImmediateNumber(&mut self, immediate_number: String) -> Result<(), ParseIntError>{
        if immediate_number.starts_with("hex") {
            self.immediate_number = match u32::from_str_radix(immediate_number.trim_start_matches("hex"), 16) {
                Ok(v) => v << self.immediate_number_start_bit,
                Err(e) => return Err(e)
            };
        } else if immediate_number.starts_with("oct") {
            self.immediate_number = match u32::from_str_radix(immediate_number.trim_start_matches("oct"), 8) {
                Ok(v) => v << self.immediate_number_start_bit,
                Err(e) => return Err(e)
            };
        } else if immediate_number.starts_with("bin") {
            self.immediate_number = match u32::from_str_radix(immediate_number.trim_start_matches("bin"), 2) {
                Ok(v) => v << self.immediate_number_start_bit,
                Err(e) => return Err(e)
            };
        } else {
            self.immediate_number = match u32::from_str_radix(immediate_number.as_str(), 10) {
                Ok(v) => v << self.immediate_number_start_bit,
                Err(e) => return Err(e)
            };
        }

        Ok(())
    }

    fn setSourceRegister(&mut self, source_register: String) -> Result<(), NotAValidRegisterError> {
        match all_register.get(&source_register) {
            None => return Err(NotAValidRegisterError{inst: self.inst_name, register: source_register}),
            Some(v) => self.source_register_label = (*v as u32) << self.source_register_start_bit
        };

        Ok(())
    }

    fn setASourceRegister(&mut self, another_source_register: String) -> Result<(), NotAValidRegisterError> {
        match all_register.get(&another_source_register) {
            None => return Err(NotAValidRegisterError{inst: self.inst_name, register: another_source_register}),
            Some(v) => self.asource_register_label = (*v as u32) << self.asource_register_start_bit
        };

        Ok(())
    }

    pub fn generateCode(&mut self, line_num: usize, target_register: String, immediate_number: Option<String>, source_register: String, asource_register: Option<String>) -> Result<u32, String>{
        /// Considering the efficiency of the compiler, here is a method that consumes more memory and improves compilation speed.
        /// Each instruction is processed by a coroutine, and at the same time, two memory spaces are opened for saving the processing results,
        /// one is normal and the other is abnormal, and the space is consistent with the number of instructions in the compiled file.

        let mut error_infos = String::new();
        let mut error = false;

        match self.setTargetRegister(target_register) {
            Ok(_) => (),
            Err(e) => {
                error = true;
                error_infos += &format!("Line: {} - {}\n", line_num, e)
            }
        };

        match immediate_number {
            None => (),
            Some(v) => match self.setImmediateNumber(v) {
                Ok(_) => (),
                Err(e) => {
                    error = true;
                    error_infos += &format!("Line: {} - An error occurred while parsing the immediate number: {}\n", line_num, e)
                }
            }
        }

        match self.setSourceRegister(source_register) {
            Ok(_) => (),
            Err(e) => {
                error = true;
                error_infos += &format!("Line: {} - {}\n", line_num, e)
            }
        }

        match asource_register {
            None => (),
            Some(v) => match self.setASourceRegister(v) {
                Ok(_) => (),
                Err(e) => {
                    error = true;
                    error_infos += &format!("Line: {} - {}\n", line_num, e)
                }
            }
        }

        if error {
            return Err(error_infos);
        } else {
            match self.argsToBinaryCode() {
                Ok(v) => return Ok(v),
                Err(_) => {
                    error_infos += &format!("Line: {} - Binary opcode generation error, info: opcode: {:320b}, imd_num: {:320b}, t_r: {:320b}, s_r: {:320b}, as_r: {:320b}\n", line_num, self.op_code,self.immediate_number, self.target_register_label, self.source_register_label, self.asource_register_label);
                    return Err(error_infos);
                }
            }
        }
    }

    fn argsToBinaryCode(&self) -> Result<u32, ()> {
        /// A easy way to check
        let a = self.op_code + self.target_register_label + self.source_register_label + self.asource_register_label + self.immediate_number;
        let b = self.op_code | self.target_register_label | self.source_register_label | self.asource_register_label + self.immediate_number;
        if a == b {
            return Ok(a);
        } else {
            return Err(());
        }
    }
}

pub struct JUMP {
    inst_type: String,
    inst_name: String,
    op_code: u32,
    // binary register label
    source_register_label: u32,
    source_register_start_bit: u8,
    // binary immediate number
    immediate_number: u32,
    immediate_number_start_bit: u8,
    ftarget_register_label: u32,
    starget_register_label: u32,
    ftarget_register_start_bit: u8,
    starget_register_start_bit: u8
}

impl JUMP {
    fn new(inst_type: String, inst_name: String, op_code: u16, srsb: u8, insb: u8, ftrsb: u8, strsb: u8) -> JUMP {
        JUMP {
            inst_type,
            inst_name,
            op_code: (op_code as u32) << 22,
            source_register_label: 0,
            source_register_start_bit: srsb,
            immediate_number: 0,
            immediate_number_start_bit: insb,
            ftarget_register_label: 0,
            starget_register_label: 0,
            ftarget_register_start_bit: ftrsb,
            starget_register_start_bit: strsb,
        }
    }

    fn setSourceRegister(&mut self, source_register: String) -> Result<(), NotAValidRegisterError> {
        match general_register.get(&source_register) {
            None => return Err(NotAValidRegisterError{inst: self.inst_name, register: source_register}),
            Some(v) => self.source_register_label = (*v as u32) << self.source_register_start_bit
        };

        Ok(())
    }

    fn setImmediateNumber(&mut self, immediate_number: String) -> Result<(), ParseIntError>{
        if immediate_number.starts_with("hex") {
            self.immediate_number = match u32::from_str_radix(immediate_number.trim_start_matches("hex"), 16) {
                Ok(v) => v << self.immediate_number_start_bit,
                Err(e) => return Err(e)
            };
        } else if immediate_number.starts_with("oct") {
            self.immediate_number = match u32::from_str_radix(immediate_number.trim_start_matches("oct"), 8) {
                Ok(v) => v << self.immediate_number_start_bit,
                Err(e) => return Err(e)
            };
        } else if immediate_number.starts_with("bin") {
            self.immediate_number = match u32::from_str_radix(immediate_number.trim_start_matches("bin"), 2) {
                Ok(v) => v << self.immediate_number_start_bit,
                Err(e) => return Err(e)
            };
        } else {
            self.immediate_number = match u32::from_str_radix(immediate_number.as_str(), 10) {
                Ok(v) => v << self.immediate_number_start_bit,
                Err(e) => return Err(e)
            };
        }

        Ok(())
    }

    fn setFTargetRegister(&mut self, first_target_register: String) -> Result<(), NotAValidRegisterError> {
        match all_register.get(&first_target_register) {
            None => return Err(NotAValidRegisterError{inst: self.inst_name, register: first_target_register}),
            Some(v) => self.ftarget_register_label = (*v as u32) << self.ftarget_register_start_bit
        };

        Ok(())
    }

    fn setSTargetRegister(&mut self, second_target_register: String) -> Result<(), NotAValidRegisterError> {
        match all_register.get(&second_target_register) {
            None => return Err(NotAValidRegisterError{inst: self.inst_name, register: second_target_register}),
            Some(v) => self.starget_register_label = (*v as u32) << self.starget_register_start_bit
        };

        Ok(())
    }

    pub fn generateCode(&mut self, line_num: usize, source_register: String, immediate_number: Option<String>, ftarget_register: Option<String>, starget_register: Option<String>) -> Result<u32, String>{
        /// Considering the efficiency of the compiler, here is a method that consumes more memory and improves compilation speed.
        /// Each instruction is processed by a coroutine, and at the same time, two memory spaces are opened for saving the processing results,
        /// one is normal and the other is abnormal, and the space is consistent with the number of instructions in the compiled file.

        let mut error_infos = String::new();
        let mut error = false;

        match self.setSourceRegister(source_register) {
            Ok(_) => (),
            Err(e) => {
                error = true;
                error_infos += &format!("Line: {} - {}\n", line_num, e)
            }
        };

        match immediate_number {
            None => (),
            Some(v) => match self.setImmediateNumber(v) {
                Ok(_) => (),
                Err(e) => {
                    error = true;
                    error_infos += &format!("Line: {} - An error occurred while parsing the immediate number: {}\n", line_num, e)
                }
            }
        }

        match ftarget_register {
            None => (),
            Some(v) => match self.setFTargetRegister(v) {
                Ok(_) => (),
                Err(e) => {
                    error = true;
                    error_infos += &format!("Line: {} - {}\n", line_num, e)
                }
            }
        }

        match starget_register {
            None => (),
            Some(v) => match self.setSTargetRegister(v) {
                Ok(_) => (),
                Err(e) => {
                    error = true;
                    error_infos += &format!("Line: {} - {}\n", line_num, e)
                }
            }
        }

        if error {
            return Err(error_infos);
        } else {
            match self.argsToBinaryCode() {
                Ok(v) => return Ok(v),
                Err(_) => {
                    error_infos += &format!("Line: {} - Binary opcode generation error, info: opcode: {:320b}, imd_num: {:320b}, s_r: {:320b}, ft_r: {:320b}, st_r: {:320b}\n", line_num, self.op_code,self.immediate_number, self.source_register_label, self.ftarget_register_label, self.starget_register_label);
                    return Err(error_infos);
                }
            }
        }
    }

    fn argsToBinaryCode(&self) -> Result<u32, ()> {
        /// A easy way to check
        let a = self.op_code + self.source_register_label + self.starget_register_label + self.ftarget_register_label + self.immediate_number;
        let b = self.op_code | self.source_register_label | self.starget_register_label | self.ftarget_register_label + self.immediate_number;
        if a == b {
            return Ok(a);
        } else {
            return Err(());
        }
    }
}

pub struct OTHERS {
    inst_type: String,
    inst_name: String,
    op_code: u32
}

impl OTHERS {
    pub fn new(inst_type: String, inst_name: String, op_code: u16) -> OTHERS {
        OTHERS {
            inst_type,
            inst_name,
            op_code: (op_code as u32) << 22
        }
    }

    pub fn generateCode(&self) -> Result<(u32), String> {
        return Ok(self.op_code)
    }
}