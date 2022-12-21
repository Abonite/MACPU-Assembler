use std::collections::HashMap;

enum INFO {
    Comb(HashMap<Vec<String>, u16>),
    Notcomb(HashMap<Vec<String>, bool>)
}

pub struct inst_info {
    bcode: u16,
    arg_num: u8,
    comb_inst: Option<bool>,
    arg_kinds_info: Option<INFO>
}

pub fn get_instructions() -> HashMap<String, inst_info> {
    HashMap::from(
        [
            (String::from("NOP"),
            inst_info{
                bcode: 0x0000,
                arg_num: 0,
                comb_inst: None,
                arg_kinds_info: None
            }),
            (String::from("MOV"),
            inst_info{
                bcode: 0x0001,
                arg_num: 2,
                comb_inst: Some(true),
                arg_kinds_info: Some(INFO::Comb(HashMap::from([
                    (vec![String::from("addr"), String::from("regs")], 0),
                    (vec![String::from("regs"), String::from("regs")], 1),
                    (vec![String::from("regs"), String::from("addr")], 2)
                ])))
            }),
            (String::from("LOAD"),
            inst_info{
                bcode: 0x0004,
                arg_num: 2,
                comb_inst: Some(false),
                arg_kinds_info: Some(INFO::Notcomb(HashMap::from([
                    (vec![String::from("imdn"), String::from("regs")], true),
                ])))
            }),
            (String::from("ADD"),
            inst_info{
                bcode: 0x0100,
                arg_num: 2,
                comb_inst: Some(false),
                arg_kinds_info: Some(INFO::Notcomb(HashMap::from([
                    (vec![String::from("regs"), String::from("regs")], true),
                ])))
            }),
            (String::from("SUB"),
            inst_info{
                bcode: 0x0101,
                arg_num: 2,
                comb_inst: Some(false),
                arg_kinds_info: Some(INFO::Notcomb(HashMap::from([
                    (vec![String::from("regs"), String::from("regs")], true),
                ])))
            }),
            (String::from("AND"),
            inst_info{
                bcode: 0x0102,
                arg_num: 2,
                comb_inst: Some(false),
                arg_kinds_info: Some(INFO::Notcomb(HashMap::from([
                    (vec![String::from("regs"), String::from("regs")], true),
                ])))
            }),
            (String::from("OR"),
            inst_info{
                bcode: 0x0103,
                arg_num: 2,
                comb_inst: Some(false),
                arg_kinds_info: Some(INFO::Notcomb(HashMap::from([
                    (vec![String::from("regs"), String::from("regs")], true),
                ])))
            }),
            // Logical Not
            (String::from("LNOT"),
            inst_info{
                bcode: 0x0104,
                arg_num: 1,
                comb_inst: Some(false),
                arg_kinds_info: Some(INFO::Notcomb(HashMap::from([
                    (vec![String::from("regs")], true),
                ])))
            }),
            // Bitwise Not
            (String::from("BNOT"),
            inst_info{
                bcode: 0x0105,
                arg_num: 1,
                comb_inst: Some(false),
                arg_kinds_info: Some(INFO::Notcomb(HashMap::from([
                    (vec![String::from("regs")], true),
                ])))
            }),
            (String::from("XOR"),
            inst_info{
                bcode: 0x0106,
                arg_num: 2,
                comb_inst: Some(false),
                arg_kinds_info: Some(INFO::Notcomb(HashMap::from([
                    (vec![String::from("regs"), String::from("regs")], true),
                ])))
            }),
            (String::from("RAND"),
            inst_info{
                bcode: 0x0107,
                arg_num: 1,
                comb_inst: Some(false),
                arg_kinds_info: Some(INFO::Notcomb(HashMap::from([
                    (vec![String::from("regs")], true),
                ])))
            }),
            (String::from("ROR"),
            inst_info{
                bcode: 0x0108,
                arg_num: 1,
                comb_inst: Some(false),
                arg_kinds_info: Some(INFO::Notcomb(HashMap::from([
                    (vec![String::from("regs")], true),
                ])))
            }),
            (String::from("RXOR"),
            inst_info{
                bcode: 0x0109,
                arg_num: 1,
                comb_inst: Some(false),
                arg_kinds_info: Some(INFO::Notcomb(HashMap::from([
                    (vec![String::from("regs")], true),
                ])))
            }),
            (String::from("LSL"),
            inst_info{
                bcode: 0x010A,
                arg_num: 2,
                comb_inst: Some(false),
                arg_kinds_info: Some(INFO::Notcomb(HashMap::from([
                    (vec![String::from("regs"), String::from("regs")], true),
                ])))
            }),
            (String::from("LSR"),
            inst_info{
                bcode: 0x010B,
                arg_num: 2,
                comb_inst: Some(false),
                arg_kinds_info: Some(INFO::Notcomb(HashMap::from([
                    (vec![String::from("regs"), String::from("regs")], true),
                ])))
            }),
            (String::from("ASL"),
            inst_info{
                bcode: 0x010C,
                arg_num: 2,
                comb_inst: Some(false),
                arg_kinds_info: Some(INFO::Notcomb(HashMap::from([
                    (vec![String::from("regs"), String::from("regs")], true),
                ])))
            }),
            (String::from("ASR"),
            inst_info{
                bcode: 0x010D,
                arg_num: 2,
                comb_inst: Some(false),
                arg_kinds_info: Some(INFO::Notcomb(HashMap::from([
                    (vec![String::from("regs"), String::from("regs")], true),
                ])))
            }),
            (String::from("CSL"),
            inst_info{
                bcode: 0x010E,
                arg_num: 2,
                comb_inst: Some(false),
                arg_kinds_info: Some(INFO::Notcomb(HashMap::from([
                    (vec![String::from("regs"), String::from("regs")], true),
                ])))
            }),
            (String::from("CSR"),
            inst_info{
                bcode: 0x010F,
                arg_num: 2,
                comb_inst: Some(false),
                arg_kinds_info: Some(INFO::Notcomb(HashMap::from([
                    (vec![String::from("regs"), String::from("regs")], true),
                ])))
            }),
            (String::from("INC"),
            inst_info{
                bcode: 0x0110,
                arg_num: 1,
                comb_inst: Some(false),
                arg_kinds_info: Some(INFO::Notcomb(HashMap::from([
                    (vec![String::from("regs")], true),
                ])))
            }),
            (String::from("DEC"),
            inst_info{
                bcode: 0x0111,
                arg_num: 1,
                comb_inst: Some(false),
                arg_kinds_info: Some(INFO::Notcomb(HashMap::from([
                    (vec![String::from("regs")], true),
                ])))
            }),
            (String::from("JMP"),
            inst_info{
                bcode: 0x0200,
                arg_num: 1,
                comb_inst: Some(false),
                arg_kinds_info: Some(INFO::Notcomb(HashMap::from([
                    (vec![String::from("addr")], true),
                    (vec![String::from("label")], true),
                ])))
            }),
            (String::from("INT"),
            inst_info{
                bcode: 0x8000,
                arg_num: 1,
                comb_inst: Some(false),
                arg_kinds_info: Some(INFO::Notcomb(HashMap::from([
                    (vec![String::from("imdn")], true),
                ])))
            }),
        ]
    )
}
