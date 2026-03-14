use std::{collections::HashMap, num::ParseIntError};
use crate::FileParser::Instr;

#[derive(Debug, Clone)]
struct Constraint {
    target_invalid_reg: Vec<String>,
    source_0_invalid_reg: Vec<String>,
    source_1_invalid_reg: Vec<String>,
    immediate_0_number_max: u32,
    immediate_1_number_max: u32
}

pub fn pars_instructions(instructions: Vec<Instr>, labels: HashMap<String, u64>) -> Vec<u8> {
    let mut result = vec![];
    for line in instructions {
        let mut bin = 0;

        if line.data.starts_with("LOAD8") || line.data.starts_with("load8") {
            bin = match InstPars::pars_load8(line.data.trim_start_matches("LOAD8").trim_start_matches("load8").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>(), labels.clone()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    println!("line: {}", line.line);
                    panic!();
                }
            };
        } else if line.data.starts_with("LOAD16") || line.data.starts_with("load16") {
            bin = match InstPars::pars_load16(line.data.trim_start_matches("LOAD16").trim_start_matches("load16").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>(), labels.clone()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    println!("line: {}", line.line);
                    panic!();
                }
            };
        } else if line.data.starts_with("LOAD32") || line.data.starts_with("load32") {
            bin = match InstPars::pars_load32(line.data.trim_start_matches("LOAD32").trim_start_matches("load32").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>(), labels.clone()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    println!("line: {}", line.line);
                    panic!();
                }
            };
        } else if line.data.starts_with("STORE8") || line.data.starts_with("store8") {
            bin = match InstPars::pars_store8(line.data.trim_start_matches("STORE8").trim_start_matches("store8").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>(), labels.clone()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    println!("line: {}", line.line);
                    panic!();
                }
            };
        } else if line.data.starts_with("STORE16") || line.data.starts_with("store16") {
            bin = match InstPars::pars_store16(line.data.trim_start_matches("STORE16").trim_start_matches("store16").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>(), labels.clone()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    println!("line: {}", line.line);
                    panic!();
                }
            };
        } else if line.data.starts_with("STORE32") || line.data.starts_with("store32") {
            bin = match InstPars::pars_store32(line.data.trim_start_matches("STORE32").trim_start_matches("store32").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>(), labels.clone()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    println!("line: {}", line.line);
                    panic!();
                }
            };
        } else if line.data.starts_with("MOVE") || line.data.starts_with("move") {
            bin = match InstPars::pars_move(line.data.trim_start_matches("MOVE").trim_start_matches("move").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>(), labels.clone()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    println!("line: {}", line.line);
                    panic!();
                }
            };
        } else if line.data.starts_with("ADD") || line.data.starts_with("add") {
            bin = match InstPars::pars_add(line.data.trim_start_matches("ADD").trim_start_matches("add").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>(), labels.clone()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    println!("line: {}", line.line);
                    panic!();
                }
            };
        } else if line.data.starts_with("SUB") || line.data.starts_with("sub") {
            bin = match InstPars::pars_sub(line.data.trim_start_matches("SUB").trim_start_matches("sub").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>(), labels.clone()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    println!("line: {}", line.line);
                    panic!();
                }
            };
        } else if line.data.starts_with("EQ") || line.data.starts_with("eq") {
            bin = match InstPars::pars_eq(line.data.trim_start_matches("EQ").trim_start_matches("eq").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>(), labels.clone()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    println!("line: {}", line.line);
                    panic!();
                }
            };
        } else if line.data.starts_with("JMP") || line.data.starts_with("jmp") {
            bin = match InstPars::pars_jmp(line.data.trim_start_matches("JMP").trim_start_matches("jmp").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>(), labels.clone()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    println!("line: {}", line.line);
                    panic!();
                }
            };
        } else if line.data.starts_with("OJMP") || line.data.starts_with("ojmp") {
            bin = match InstPars::pars_ojmp(line.data.trim_start_matches("OJMP").trim_start_matches("ojmp").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>(), labels.clone()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    println!("line: {}", line.line);
                    panic!();
                }
            };
        } else if line.data.starts_with("ZJMP") || line.data.starts_with("zjmp") {
            bin = match InstPars::pars_zjmp(line.data.trim_start_matches("ZJMP").trim_start_matches("zjmp").trim().split(',').map(|x| x.trim()).collect::<Vec<&str>>(), labels.clone()) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    println!("line: {}", line.line);
                    panic!();
                }
            };
        }

        if result.len() < line.address as usize {
            let diff = line.address as usize - result.len();
            let mut zeros = vec![0_u8; diff];
            result.append(&mut zeros);
            result.append(&mut bin.to_le_bytes().to_vec());
        } else if result.len() == line.address as usize {
            result.append(&mut bin.to_le_bytes().to_vec());
        } else if result.len() > line.address as usize {
            let data = bin.to_le_bytes();
            for addr in line.address..line.address + 4 {
                if result[addr as usize] == 0 {
                    result[addr as usize] = data[(addr - line.address) as usize];
                } else {
                    println!("Instruction address conflict.");
                    panic!();
                }
            }
        }
    }

    result
}

fn get_register_label(register_name: &str) -> Result<u8, String> {
    return match register_name {
        "PC" | "pc" => Ok(0b101001),
        "ZERO" | "zero" => Ok(0b000000),

        "A0" | "a0" => Ok(1),
        "A1" | "a1" => Ok(2),
        "A2" | "a2" => Ok(3),
        "A3" | "a3" => Ok(4),
        "AR0" | "ar0" => Ok(5),
        "AR1" | "ar1" => Ok(6),
        "AR2" | "ar2" => Ok(7),
        "ASS" | "ass" => Ok(8),
        "ASP" | "asp" => Ok(9),
        "ADS" | "ads" => Ok(10),

        "B0" | "b0" => Ok(11),
        "B1" | "b1" => Ok(12),
        "B2" | "b2" => Ok(13),
        "B3" | "b3" => Ok(14),
        "BR0" | "br0" => Ok(15),
        "BR1" | "br1" => Ok(16),
        "BR2" | "br2" => Ok(17),
        "BSS" | "bss" => Ok(18),
        "BSP" | "bsp" => Ok(19),
        "BDS" | "bds" => Ok(20),

        "C0" | "c0" => Ok(21),
        "C1" | "c1" => Ok(22),
        "C2" | "c2" => Ok(23),
        "C3" | "c3" => Ok(24),
        "CR0" | "cr0" => Ok(25),
        "CR1" | "cr1" => Ok(26),
        "CR2" | "cr2" => Ok(27),
        "CSS" | "css" => Ok(28),
        "CSP" | "csp" => Ok(29),
        "CDS" | "cds" => Ok(20),

        "D0" | "d0" => Ok(31),
        "D1" | "d1" => Ok(32),
        "D2" | "d2" => Ok(33),
        "D3" | "d3" => Ok(34),
        "DR0" | "dr0" => Ok(35),
        "DR1" | "dr1" => Ok(36),
        "DR2" | "dr2" => Ok(37),
        "DSS" | "dss" => Ok(38),
        "DSP" | "dsp" => Ok(39),
        "DDS" | "dds" => Ok(40),

        _ => Err(String::from("Unknown register name."))
    }
}

fn para_immediate_num(immediate_number: &str) -> Result<u32, ParseIntError>{
    if immediate_number.starts_with("0x") {
        let immediate_number = immediate_number.trim_start_matches("0x");
        return u32::from_str_radix(immediate_number, 16);
    } else if immediate_number.starts_with("0o") {
        let immediate_number = immediate_number.trim_start_matches("0o");
        return u32::from_str_radix(immediate_number, 8);
    } else if immediate_number.starts_with("0b") {
        let immediate_number = immediate_number.trim_start_matches("0b");
        return u32::from_str_radix(immediate_number, 2);
    } else {
        return u32::from_str_radix(immediate_number, 10);
    }
}

#[derive(Debug, Clone)]
struct Register {
    name: String,
    label: u8
}

#[derive(Debug, Clone)]
enum Source {
    REG(Register),
    IMM(u32)
}

fn generate_register_ast(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<Vec<Source>, String> {
    let mut result = vec![];

    for item in register_info {
        if item.starts_with('%') {
            let register = item.trim_start_matches("%");
            result.push(Source::REG( Register {
                name: register.to_string(),
                label: match get_register_label(register) {
                    Ok(v) => v,
                    Err(e) => {
                        println!("{}", e);
                        panic!();
                    }
                }
            }));
        } else if item.starts_with('[') {
            let imdn = item.trim_start_matches("[").trim_end_matches("]").trim();
            result.push(Source::IMM(match para_immediate_num(imdn) {
                Ok(v) => v,
                Err(e) => {
                    println!("{}", e);
                    panic!();
                }
            }));
        } else if labels.contains_key(item) {
            result.push(Source::IMM(labels[item] as u32));
        } else {
            match calculate_expression(item, labels.clone()) {
                Ok(val) => result.push(Source::IMM(val)),
                Err(e) => return Err(format!("Invalid expression '{}': {}", item, e)),
            }
        }
    }

    Ok(result)
}

fn calculate_expression(expression: &str, labels: HashMap<String, u64>) -> Result<u32, String> {
    // 词法分析：将表达式分割成令牌
    let mut tokens = vec![];
    let mut token = String::new();
    let mut curr_state = "";
    for c in expression.chars() {
        match c {
            '+' | '-' | '*' | '/' | '%' | '(' | ')' => {
                if !token.is_empty() {
                    tokens.push(token.clone());
                    token.clear();
                }
                tokens.push(c.to_string());
                curr_state = "";
            },
            '<' => {
                if curr_state == "slf" {
                    if !token.is_empty() {
                        tokens.push(token.clone());
                        token.clear();
                    }
                    tokens.push("<<".to_string());
                    curr_state = "";
                } else if curr_state.is_empty() {
                    curr_state = "slf";
                } else {
                    return Err(String::from("Invalid calculate symbol."));
                }
            },
            '>' => {
                if curr_state == "srf" {
                    if !token.is_empty() {
                        tokens.push(token.clone());
                        token.clear();
                    }
                    tokens.push(">>".to_string());
                    curr_state = "";
                } else if curr_state.is_empty() {
                    curr_state = "srf";
                } else {
                    return Err(String::from("Invalid calculate symbol."));
                }
            },
            _ => {
                // 如果当前处于状态，但遇到非状态字符，则先推送状态字符
                if curr_state == "slf" {
                    tokens.push("<".to_string());
                    curr_state = "";
                } else if curr_state == "srf" {
                    tokens.push(">".to_string());
                    curr_state = "";
                }
                token.push(c);
            }
        }
    }
    // 处理末尾可能的状态
    if curr_state == "slf" {
        tokens.push("<".to_string());
    } else if curr_state == "srf" {
        tokens.push(">".to_string());
    }
    // 处理最后一个令牌
    if !token.is_empty() {
        tokens.push(token);
    }

    // 定义令牌类型
    #[derive(Debug, Clone)]
    enum Token {
        Op(String),
        Num(i64),
    }

    // 将字符串令牌转换为Token枚举
    let mut parsed_tokens = vec![];
    for token in tokens {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }

        // 检查是否为数字字面量
        if token.starts_with("0x") {
            match i64::from_str_radix(&token[2..], 16) {
                Ok(v) => parsed_tokens.push(Token::Num(v)),
                Err(e) => return Err(format!("Invalid hex number '{}': {}", token, e)),
            }
        } else if token.starts_with("0o") {
            match i64::from_str_radix(&token[2..], 8) {
                Ok(v) => parsed_tokens.push(Token::Num(v)),
                Err(e) => return Err(format!("Invalid octal number '{}': {}", token, e)),
            }
        } else if token.starts_with("0b") {
            match i64::from_str_radix(&token[2..], 2) {
                Ok(v) => parsed_tokens.push(Token::Num(v)),
                Err(e) => return Err(format!("Invalid binary number '{}': {}", token, e)),
            }
        } else if token == "+" || token == "-" || token == "*" || token == "/" || token == "%" ||
                  token == "<<" || token == ">>" || token == "(" || token == ")" {
            parsed_tokens.push(Token::Op(token.to_string()));
        } else {
            // 尝试解析为十进制数字
            match token.parse::<i64>() {
                Ok(v) => parsed_tokens.push(Token::Num(v)),
                Err(_) => {
                    // 检查是否为标签
                    match labels.get(token) {
                        Some(&l) => parsed_tokens.push(Token::Num(l as i64)),
                        None => return Err(format!("Unknown symbol: {}", token)),
                    }
                }
            }
        }
    }

    // 普拉特解析器实现
    #[derive(Debug)]
    enum Expr {
        Number(i64),
        BinaryOp(String, Box<Expr>, Box<Expr>),
    }

    // 获取运算符优先级
    fn precedence(op: &str) -> i32 {
        match op {
            "<<" | ">>" => 1,
            "+" | "-" => 2,
            "*" | "/" | "%" => 3,
            ")" => -1,
            "(" => -1,
            _ => 0,
        }
    }

    // 解析表达式
    struct Parser {
        tokens: Vec<Token>,
        pos: usize,
    }

    impl Parser {
        fn new(tokens: Vec<Token>) -> Self {
            Parser { tokens, pos: 0 }
        }

        fn current(&self) -> Option<&Token> {
            self.tokens.get(self.pos)
        }

        fn consume(&mut self) -> Option<Token> {
            if self.pos < self.tokens.len() {
                let token = self.tokens[self.pos].clone();
                self.pos += 1;
                Some(token)
            } else {
                None
            }
        }

        fn parse_expression(&mut self, min_precedence: i32) -> Result<Expr, String> {
            // 解析左侧表达式
            let mut left = self.parse_primary()?;

            while let Some(Token::Op(op)) = self.current() {
                let prec = precedence(op);
                if prec < min_precedence {
                    break;
                }

                // 左结合性，所以当前优先级+1
                let next_min_precedence = prec + 1;
                let op = op.clone();
                self.consume(); // 消费运算符

                let right = self.parse_expression(next_min_precedence)?;
                left = Expr::BinaryOp(op, Box::new(left), Box::new(right));
            }

            Ok(left)
        }

        fn parse_primary(&mut self) -> Result<Expr, String> {
            match self.consume() {
                Some(Token::Num(n)) => Ok(Expr::Number(n)),
                Some(Token::Op(op)) if op == "(" => {
                    let expr = self.parse_expression(0)?;
                    match self.current() {
                        Some(Token::Op(closing)) if closing == ")" => {
                            self.consume();
                            Ok(expr)
                        }
                        _ => Err("Expected ')'".to_string()),
                    }
                }
                Some(Token::Op(op)) if op == "-" => {
                    // 一元负号（可选实现）
                    let expr = self.parse_primary()?;
                    Ok(Expr::BinaryOp("*".to_string(), Box::new(Expr::Number(-1)), Box::new(expr)))
                }
                _ => Err("Expected number or '('".to_string()),
            }
        }

        fn parse(&mut self) -> Result<Expr, String> {
            let expr = self.parse_expression(0)?;
            if self.pos < self.tokens.len() {
                Err("Unexpected tokens at end of expression".to_string())
            } else {
                Ok(expr)
            }
        }
    }

    let mut parser = Parser::new(parsed_tokens);
    let expr = parser.parse()?;

    // 计算表达式值
    fn eval(expr: Expr) -> Result<i64, String> {
        match expr {
            Expr::Number(n) => Ok(n),
            Expr::BinaryOp(op, left, right) => {
                let left_val = eval(*left)?;
                let right_val = eval(*right)?;
                match op.as_str() {
                    "+" => Ok(left_val + right_val),
                    "-" => Ok(left_val - right_val),
                    "*" => Ok(left_val * right_val),
                    "/" => {
                        if right_val == 0 {
                            return Err("Division by zero".to_string());
                        }
                        Ok(left_val / right_val)
                    }
                    "%" => {
                        if right_val == 0 {
                            return Err("Modulo by zero".to_string());
                        }
                        Ok(left_val % right_val)
                    }
                    "<<" => Ok(left_val << right_val),
                    ">>" => Ok(left_val >> right_val),
                    _ => Err(format!("Unknown operator: {}", op)),
                }
            }
        }
    }

    let result = eval(expr)?;
    // 转换为u32，检查溢出
    if result < 0 || result > u32::MAX as i64 {
        return Err("Expression result out of u32 range".to_string());
    }
    Ok(result as u32)
}

struct InstDiffTypePars {}

impl InstDiffTypePars {
    fn pars_ti(rast: Vec<Source>, constraint: Constraint, op_name: &str) -> Result<u32, String> {
        let target_register = match rast.get(0) {
            Some(s) => {
                match s {
                    Source::REG(r) => r,
                    Source::IMM(i) => return Err(format!("{}: Invalid target register.", op_name))
                }
            },
            None => return Err(format!("{}: missing parameters.", op_name))
        };

        let &immediate_number = match rast.get(1) {
            Some(s) => {
                match s {
                    Source::REG(_) => return Err(format!("{}: Invalid immediate number.", op_name)),
                    Source::IMM(i) => i
                }
            },
            None => return Err(format!("{}: missing parameters.", op_name))
        };

        if constraint.target_invalid_reg.contains(&target_register.name) {
            return Err(format!("{}: target register can't be {}", op_name, target_register.name));
        }

        if immediate_number > constraint.immediate_0_number_max {
            return Err(format!("{}: immediat number is grater then {}", op_name, constraint.immediate_0_number_max));
        }

        let bin_code = ((target_register.label as u32) << 16) | immediate_number;
        return Ok(bin_code);
    }

    fn pars_s(rast: Vec<Source>, constraint: Constraint, op_name: &str) -> Result<u32, String> {
        let target_register = match rast.get(0) {
            Some(s) => {
                match s {
                    Source::REG(r) => r,
                    Source::IMM(i) => return Err(format!("{}: Invalid target register.", op_name))
                }
            },
            None => return Err(format!("{}: missing parameters.", op_name))
        };

        if constraint.target_invalid_reg.contains(&target_register.name) {
            return Err(format!("{}: target register can't be {}", op_name, target_register.name));
        }

        let bin_code = ((target_register.label as u32) << 16);
        return Ok(bin_code);
    }

    fn pars_i(rast: Vec<Source>, constraint: Constraint, op_name: &str) -> Result<u32, String> {
        let &immediate_number = match rast.get(0) {
            Some(s) => {
                match s {
                    Source::REG(_) => return Err(format!("{}: Invalid immediate number.", op_name)),
                    Source::IMM(i) => i
                }
            },
            None => return Err(format!("{}: missing parameters.", op_name))
        };

        if immediate_number > constraint.immediate_0_number_max {
            return Err(format!("immediat number is grater then {}", constraint.immediate_0_number_max));
        }

        return Ok(immediate_number);
    }

    fn pars_ss(rast: Vec<Source>, constraint: Constraint, op_name: &str) -> Result<u32, String> {
        let source_0_register = match rast.get(0) {
            Some(s) => {
                match s {
                    Source::REG(r) => r,
                    Source::IMM(i) => return Err(format!("{}: Invalid source 0 register.", op_name))
                }
            },
            None => return Err(String::from("LOAD8: missing parameters."))
        };

        let source_1_register = match rast.get(1) {
            Some(s) => {
                match s {
                    Source::REG(r) => r,
                    Source::IMM(i) => return Err(format!("{}: Invalid source 1 register.", op_name))
                }
            },
            None => return Err(String::from("LOAD8: missing parameters."))
        };

        if constraint.source_0_invalid_reg.contains(&source_0_register.name) {
            return Err(format!("{}: source 0 register can't be {}", op_name, source_0_register.name));
        }

        if constraint.source_1_invalid_reg.contains(&source_1_register.name) {
            return Err(format!("{}: source 1 register can't be {}", op_name, source_1_register.name));
        }

        let bin_code = ((source_0_register.label as u32) << 10) | ((source_1_register.label as u32) << 4);
        return Ok(bin_code);
    }

    fn pars_ts(rast: Vec<Source>, constraint: Constraint, op_name: &str) -> Result<u32, String> {
        let target_register = match rast.get(0) {
            Some(s) => {
                match s {
                    Source::REG(r) => r,
                    Source::IMM(i) => return Err(format!("{}: Invalid target register.", op_name))
                }
            },
            None => return Err(format!("{}: missing parameters.", op_name))
        };

        let source_register = match rast.get(1) {
            Some(s) => {
                match s {
                    Source::REG(r) => r,
                    Source::IMM(i) => return Err(format!("{}: Invalid source register.", op_name))
                }
            },
            None => return Err(format!("{}: missing parameters.", op_name))
        };

        if constraint.target_invalid_reg.contains(&target_register.name) {
            return Err(format!("target register can't be {}", target_register.name));
        }

        if constraint.source_0_invalid_reg.contains(&source_register.name) {
            return Err(format!("source register can't be {}", source_register.name));
        }

        let bin_code = ((target_register.label as u32) << 16) | ((source_register.label as u32) << 10);
        return Ok(bin_code);
    }

    fn pars_tsi(rast: Vec<Source>, constraint: Constraint, op_name: &str) -> Result<u32, String> {
        let target_register = match rast.get(0) {
            Some(s) => {
                match s {
                    Source::REG(r) => r,
                    Source::IMM(i) => return Err(format!("{}: Invalid target register.", op_name))
                }
            },
            None => return Err(format!("{}: missing parameters.", op_name))
        };

        let source_register = match rast.get(1) {
            Some(s) => {
                match s {
                    Source::REG(r) => r,
                    Source::IMM(i) => return Err(format!("{}: Invalid source register.", op_name))
                }
            },
            None => return Err(format!("{}: missing parameters.", op_name))
        };

        let &immediate_number = match rast.get(2) {
            Some(s) => {
                match s {
                    Source::REG(_) => return Err(format!("{}: Invalid immediate number.", op_name)),
                    Source::IMM(i) => i
                }
            },
            None => return Err(format!("{}: missing parameters.", op_name))
        };

        if constraint.target_invalid_reg.contains(&target_register.name) {
            return Err(format!("target register can't be {}", target_register.name));
        }

        if constraint.source_0_invalid_reg.contains(&source_register.name) {
            return Err(format!("source register can't be {}", source_register.name));
        }

        if immediate_number > constraint.immediate_0_number_max {
            return Err(format!("immediat number is grater then {}", constraint.immediate_0_number_max));
        }

        let bin_code = ((target_register.label as u32) << 16) | ((source_register.label as u32) << 10) | immediate_number;
        return Ok(bin_code);
    }

    fn pars_tss(rast: Vec<Source>, constraint: Constraint, op_name: &str) -> Result<u32, String> {
        let target_register = match rast.get(0) {
            Some(s) => {
                match s {
                    Source::REG(r) => r,
                    Source::IMM(i) => return Err(format!("{}: Invalid target register.", op_name))
                }
            },
            None => return Err(format!("{}: missing parameters.", op_name))
        };

        let source_0_register = match rast.get(1) {
            Some(s) => {
                match s {
                    Source::REG(r) => r,
                    Source::IMM(i) => return Err(format!("{}: Invalid source register.", op_name))
                }
            },
            None => return Err(format!("{}: missing parameters.", op_name))
        };

        let source_1_register = match rast.get(2) {
            Some(s) => {
                match s {
                    Source::REG(r) => r,
                    Source::IMM(i) => return Err(format!("{}: Invalid source register.", op_name))
                }
            },
            None => return Err(format!("{}: missing parameters.", op_name))
        };

        if constraint.target_invalid_reg.contains(&target_register.name) {
            return Err(format!("target register can't be {}", target_register.name));
        }

        if constraint.source_0_invalid_reg.contains(&source_0_register.name) {
            return Err(format!("source 0 register can't be {}", source_0_register.name));
        }

        if constraint.source_1_invalid_reg.contains(&source_1_register.name) {
            return Err(format!("source 1 register can't be {}", source_1_register.name));
        }

        let bin_code = ((target_register.label as u32) << 16) | ((source_0_register.label as u32) << 10) | ((source_1_register.label as u32) << 4);
        return Ok(bin_code);
    }

    fn pars_tii(rast: Vec<Source>, constraint: Constraint, op_name: &str) -> Result<u32, String> {
        let target_register = match rast.get(0) {
            Some(s) => {
                match s {
                    Source::REG(r) => r,
                    Source::IMM(i) => return Err(format!("{}: Invalid target register.", op_name))
                }
            },
            None => return Err(format!("{}: missing parameters.", op_name))
        };

        let &immediate_0_number = match rast.get(1) {
            Some(s) => {
                match s {
                    Source::REG(_) => return Err(format!("{}: Invalid immediate number.", op_name)),
                    Source::IMM(i) => i
                }
            },
            None => return Err(format!("{}: missing parameters.", op_name))
        };

        let &immediate_1_number = match rast.get(2) {
            Some(s) => {
                match s {
                    Source::REG(_) => return Err(format!("{}: Invalid immediate number.", op_name)),
                    Source::IMM(i) => i
                }
            },
            None => return Err(format!("{}: missing parameters.", op_name))
        };

        if constraint.target_invalid_reg.contains(&target_register.name) {
            return Err(format!("target register can't be {}", target_register.name));
        }

        if immediate_0_number > constraint.immediate_0_number_max {
            return Err(format!("immediat number 0 is grater then {}", constraint.immediate_0_number_max));
        }

        if immediate_1_number > constraint.immediate_1_number_max {
            return Err(format!("immediat number 1 is grater then {}", constraint.immediate_1_number_max));
        }

        let bin_code = ((target_register.label as u32) << 16) | (immediate_0_number << 10) | (immediate_1_number << 4);
        return Ok(bin_code);
    }
}

struct InstPars {}

impl InstPars {
    fn pars_load8(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
        let op_name = "LOAD8";

        let rast = match generate_register_ast(register_info, labels) {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let constraint = Constraint {
            target_invalid_reg: vec![String::from("PC"), String::from("ZERO")],
            source_0_invalid_reg: vec![],
            source_1_invalid_reg: vec![],
            immediate_0_number_max: 0xFF,
            immediate_1_number_max: 0
        };

        match rast.len() {
            1 => return Err(format!("{}: Too few arguments!", op_name)),
            2 => {
                match InstDiffTypePars::pars_ti(rast.clone(), constraint.clone(), op_name) {
                    Ok(b) => return Ok((0b0000_0000_01 << 22) | b),
                    Err(e) => {
                        match InstDiffTypePars::pars_s(rast, constraint, op_name) {
                            Ok(b) => return Ok((0b1100_0000_10 << 22) | b),
                            Err(e) => return Err(e)
                        }
                    }
                }
            },
            3 => {
                match InstDiffTypePars::pars_ti(rast, constraint, op_name) {
                    Ok(b) => return Ok((0b0000_0000_10 << 22) | b),
                    Err(e) => return Err(e)
                }
            },
            _ => return Err(format!("{}: Too much arguments!", op_name))
        } 
    }

    fn pars_load16(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
        let op_name = "LOAD16";

        let rast = match generate_register_ast(register_info, labels) {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let constraint = Constraint {
            target_invalid_reg: vec![String::from("PC"), String::from("ZERO")],
            source_0_invalid_reg: vec![],
            source_1_invalid_reg: vec![],
            immediate_0_number_max: 0xFF,
            immediate_1_number_max: 0
        };

        match rast.len() {
            1 => return Err(format!("{}: Too few arguments!", op_name)),
            2 => {
                match InstDiffTypePars::pars_ti(rast.clone(), constraint.clone(), op_name) {
                    Ok(b) => return Ok((0b0000_0000_11 << 22) | b),
                    Err(e1) => {
                        match InstDiffTypePars::pars_s(rast, constraint, op_name) {
                            Ok(b) => return Ok((0b1100_0001_00 << 22) | b),
                            Err(e) => return Err(e1 + "\n" + &e)
                        }
                    }
                }
            },
            3 => {
                match InstDiffTypePars::pars_ti(rast, constraint, op_name) {
                    Ok(b) => return Ok((0b0000_0001_00 << 22) | b),
                    Err(e) => Err(e)
                }
            },
            _ => return Err(format!("{}: Too much arguments!", op_name))
        } 
    }

    fn pars_load32(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
        let op_name = "LOAD32";

        let rast = match generate_register_ast(register_info, labels) {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let constraint = Constraint {
            target_invalid_reg: vec![String::from("PC"), String::from("ZERO")],
            source_0_invalid_reg: vec![],
            source_1_invalid_reg: vec![],
            immediate_0_number_max: 0xFF,
            immediate_1_number_max: 0
        };

        match rast.len() {
            1 => return Err(format!("{}: Too few arguments!", op_name)),
            2 => {
                match InstDiffTypePars::pars_ti(rast.clone(), constraint.clone(), op_name) {
                    Ok(b) => return Ok((0b0000_0001_10 << 22) | b),
                    Err(e1) => {
                        match InstDiffTypePars::pars_s(rast, constraint, op_name) {
                            Ok(b) => return Ok((0b1100_0001_01 << 22) | b),
                            Err(e) => return Err(e1 + "\n" + &e)
                        }
                    }
                }
            },
            3 => {
                match InstDiffTypePars::pars_tss(rast, constraint, op_name) {
                    Ok(b) => return Ok((0b0000_0001_01 << 22) | b),
                    Err(e) => return Err(e)
                }
            },
            _ => return Err(format!("{}: Too much arguments!", op_name))
        } 
    }

    fn pars_store8(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
        let op_name = "STORE8";

        let rast = match generate_register_ast(register_info, labels) {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let constraint = Constraint {
            target_invalid_reg: vec![String::from("PC"), String::from("ZERO")],
            source_0_invalid_reg: vec![],
            source_1_invalid_reg: vec![],
            immediate_0_number_max: 0xFF,
            immediate_1_number_max: 0
        };

        match rast.len() {
            1 => return Err(format!("{}: Too few arguments!", op_name)),
            2 => {
                match InstDiffTypePars::pars_ts(rast, constraint, op_name) {
                    Ok(b) => return Ok((0b0000_0001_10 << 22) | b),
                    Err(e) => return Err(e)
                }
            },
            3 => {
                match InstDiffTypePars::pars_tss(rast, constraint, op_name) {
                    Ok(b) => return Ok((0b0000_0001_10 << 22) | b),
                    Err(e) => return Err(e)
                }
            },
            _ => return Err(format!("{}: Too much arguments!", op_name))
        } 
    }

    fn pars_store16(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
        let op_name = "STORE16";

        let rast = match generate_register_ast(register_info, labels) {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let constraint = Constraint {
            target_invalid_reg: vec![String::from("PC"), String::from("ZERO")],
            source_0_invalid_reg: vec![],
            source_1_invalid_reg: vec![],
            immediate_0_number_max: 0xFF,
            immediate_1_number_max: 0
        };

        match rast.len() {
            1 => return Err(format!("{}: Too few arguments!", op_name)),
            2 => {
                match InstDiffTypePars::pars_ts(rast, constraint, op_name) {
                    Ok(b) => return Ok((0b0000_0001_11 << 22) | b),
                    Err(e) => return Err(e)
                }
            },
            3 => {
                match InstDiffTypePars::pars_tss(rast, constraint, op_name) {
                    Ok(b) => return Ok((0b0000_0001_11 << 22) | b),
                    Err(e) => return Err(e)
                }
            },
            _ => return Err(format!("{}: Too much arguments!", op_name))
        } 
    }

    fn pars_store32(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
        let op_name = "STORE32";

        let rast = match generate_register_ast(register_info, labels) {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let constraint = Constraint {
            target_invalid_reg: vec![String::from("PC"), String::from("ZERO")],
            source_0_invalid_reg: vec![],
            source_1_invalid_reg: vec![],
            immediate_0_number_max: 0xFF,
            immediate_1_number_max: 0
        };

        match rast.len() {
            1 => return Err(format!("{}: Too few arguments!", op_name)),
            2 => {
                match InstDiffTypePars::pars_ts(rast, constraint, op_name) {
                    Ok(b) => return Ok((0b0000_0010_00 << 22) | b),
                    Err(e) => return Err(e)
                }
            },
            3 => {
                match InstDiffTypePars::pars_tss(rast, constraint, op_name) {
                    Ok(b) => return Ok((0b0000_0010_00 << 22) | b),
                    Err(e) => return Err(e)
                }
            },
            _ => return Err(format!("{}: Too much arguments!", op_name))
        } 
    }

    fn pars_move(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
        let op_name = "MOVE";

        let rast = match generate_register_ast(register_info, labels) {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let constraint = Constraint {
            target_invalid_reg: vec![String::from("PC"), String::from("ZERO")],
            source_0_invalid_reg: vec![],
            source_1_invalid_reg: vec![],
            immediate_0_number_max: 0xFF,
            immediate_1_number_max: 0
        };

        match rast.len() {
            1 => return Err(format!("{}: Too few arguments!", op_name)),
            2 => {
                match InstDiffTypePars::pars_ts(rast, constraint, op_name) {
                    Ok(b) => return Ok((0b0000_0010_01 << 22) | b),
                    Err(e) => return Err(e)
                }
            },
            _ => return Err(format!("{}: Too much arguments!", op_name))
        } 
    }

    fn pars_add(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
        let op_name = "ADD";

        let rast = match generate_register_ast(register_info, labels) {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let constraint = Constraint {
            target_invalid_reg: vec![String::from("PC"), String::from("ZERO")],
            source_0_invalid_reg: vec![],
            source_1_invalid_reg: vec![],
            immediate_0_number_max: 0xFF,
            immediate_1_number_max: 0
        };

        match rast.len() {
            1 | 2 => return Err(format!("{}: Too few arguments!", op_name)),
            3 => {
                match InstDiffTypePars::pars_tss(rast.clone(), constraint.clone(), op_name) {
                    Ok(b) => return Ok((0b1000_0000_01 << 22) | b),
                    Err(e1) => {
                        match InstDiffTypePars::pars_tsi(rast, constraint, op_name) {
                            Ok(b) => return Ok((0b1000_0000_00 << 22) | b),
                            Err(e) => return Err(e1 + "\n" + &e)
                        }
                    }
                }
            },
            _ => return Err(format!("{}: Too much arguments!", op_name))
        } 
    }

    fn pars_sub(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
        let op_name = "SUB";

        let rast = match generate_register_ast(register_info, labels) {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let constraint = Constraint {
            target_invalid_reg: vec![String::from("PC"), String::from("ZERO")],
            source_0_invalid_reg: vec![],
            source_1_invalid_reg: vec![],
            immediate_0_number_max: 0xFF,
            immediate_1_number_max: 0
        };

        match rast.len() {
            1 | 2 => return Err(format!("{}: Too few arguments!", op_name)),
            3 => {
                match InstDiffTypePars::pars_tss(rast.clone(), constraint.clone(), op_name) {
                    Ok(b) => return Ok((0b1000_0000_01 << 22) | b),
                    Err(e1) => {
                        match InstDiffTypePars::pars_tsi(rast, constraint, op_name) {
                            Ok(b) => return Ok((0b1000_0000_00 << 22) | b),
                            Err(e) => return Err(e1 + "\n" + &e)
                        }
                    }
                }
            },
            _ => return Err(format!("{}: Too much arguments!", op_name))
        }
    }

    fn pars_eq(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
        let op_name = "EQ";

        let rast = match generate_register_ast(register_info, labels) {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let constraint = Constraint {
            target_invalid_reg: vec![String::from("PC"), String::from("ZERO")],
            source_0_invalid_reg: vec![],
            source_1_invalid_reg: vec![],
            immediate_0_number_max: 0x3FF,
            immediate_1_number_max: 0
        };

        match rast.len() {
            1 | 2 => return Err(format!("{}: Too few arguments!", op_name)),
            3 => match InstDiffTypePars::pars_tss(rast.clone(), constraint.clone(), op_name) {
                Ok(b) => return Ok((0b1001_0000_11 << 22) | b),
                Err(e1) => {
                    match InstDiffTypePars::pars_tsi(rast, constraint, op_name) {
                        Ok(b) => return Ok((0b1001_0000_10 << 22) | b),
                        Err(e) => return Err(e1 + "\n" + &e)
                    }
                }
            }
            _ => return Err(format!("{}: Too much arguments!", op_name))
        } 
    }

    fn pars_jmp(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
        let op_name = "JMP";

        let rast = match generate_register_ast(register_info, labels) {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let constraint = Constraint {
            target_invalid_reg: vec![String::from("PC"), String::from("ZERO")],
            source_0_invalid_reg: vec![],
            source_1_invalid_reg: vec![],
            immediate_0_number_max: 0x3FFFFF,
            immediate_1_number_max: 0
        };

        match rast.len() {
            1 => {
                match InstDiffTypePars::pars_i(rast.clone(), constraint.clone(), op_name) {
                    Ok(b) => return Ok((0b1100_0000_00 << 22) | b),
                    Err(e1) => {
                        match InstDiffTypePars::pars_s(rast, constraint, op_name) {
                            Ok(b) => return Ok((0b1100_0000_01 << 22) | b),
                            Err(e) => return Err(e1 + "\n" + &e)
                        }
                    }
                }
            },
            2 => {
                match InstDiffTypePars::pars_ss(rast, constraint, op_name) {
                    Ok(b) => return Ok((0b1100_0000_01 << 22) | b),
                    Err(e) => return Err(e)
                }
            },
            _ => return Err(format!("{}: Too much arguments!", op_name))
        } 
    }

    fn pars_ojmp(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
        let op_name = "OJMP";

        let rast = match generate_register_ast(register_info, labels) {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let constraint = Constraint {
            target_invalid_reg: vec![String::from("PC"), String::from("ZERO")],
            source_0_invalid_reg: vec![],
            source_1_invalid_reg: vec![],
            immediate_0_number_max: 0x3FFFFF,
            immediate_1_number_max: 0
        };

        match rast.len() {
            1 => return Err(format!("{}: Too few arguments!", op_name)),
            2 => {
                match InstDiffTypePars::pars_ti(rast.clone(), constraint.clone(), op_name) {
                    Ok(b) => return Ok((0b1100_0000_10 << 22) | b),
                    Err(e1) => {
                        match InstDiffTypePars::pars_ts(rast, constraint, op_name) {
                            Ok(b) => return Ok((0b1100_0000_11 << 22) | b),
                            Err(e) => return Err(e1 + "\n" + &e)
                        }
                    }
                }
            },
            _ => return Err(format!("{}: Too much arguments!", op_name))
        } 
    }

    fn pars_zjmp(register_info: Vec<&str>, labels: HashMap<String, u64>) -> Result<u32, String> {
        let op_name = "ZJMP";

        let rast = match generate_register_ast(register_info, labels) {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let constraint = Constraint {
            target_invalid_reg: vec![String::from("PC"), String::from("ZERO")],
            source_0_invalid_reg: vec![],
            source_1_invalid_reg: vec![],
            immediate_0_number_max: 0x3FFFFF,
            immediate_1_number_max: 0
        };

        match rast.len() {
            1 => return Err(format!("{}: Too few arguments!", op_name)),
            2 => {
                match InstDiffTypePars::pars_ti(rast.clone(), constraint.clone(), op_name) {
                    Ok(b) => return Ok((0b1100_0001_00 << 22) | b),
                    Err(e1) => {
                        match InstDiffTypePars::pars_ts(rast, constraint, op_name) {
                            Ok(b) => return Ok((0b1100_0001_01 << 22) | b),
                            Err(e) => return Err(e1 + "\n" + &e)
                        }
                    }
                }
            },
            _ => return Err(format!("{}: Too much arguments!", op_name))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_calculate_expression_basic() {
        let labels = HashMap::new();
        assert_eq!(calculate_expression("1 + 2", labels.clone()), Ok(3));
        assert_eq!(calculate_expression("2 * 3", labels.clone()), Ok(6));
        assert_eq!(calculate_expression("1 + 2 * 3", labels.clone()), Ok(7));
        assert_eq!(calculate_expression("(1 + 2) * 3", labels.clone()), Ok(9));
        assert_eq!(calculate_expression("10 / 2", labels.clone()), Ok(5));
        assert_eq!(calculate_expression("10 % 3", labels.clone()), Ok(1));
        assert_eq!(calculate_expression("1 << 4", labels.clone()), Ok(16));
        assert_eq!(calculate_expression("16 >> 2", labels.clone()), Ok(4));
    }

    #[test]
    fn test_calculate_expression_complex() {
        let labels = HashMap::new();
        // 嵌套括号
        assert_eq!(calculate_expression("((1 + 2) * 3) + 4", labels.clone()), Ok(13));
        assert_eq!(calculate_expression("2 * (3 + 4) * 5", labels.clone()), Ok(70));

        // 混合运算符
        assert_eq!(calculate_expression("1 + 2 * 3 << 2", labels.clone()), Ok(28)); // 1 + (2*3) = 7, 7 << 2 = 28
        assert_eq!(calculate_expression("(1 + 2) << (3 - 1)", labels.clone()), Ok(12)); // 3 << 2 = 12

        // 除法取模组合
        assert_eq!(calculate_expression("10 / 3 * 3 + 10 % 3", labels.clone()), Ok(10));

        // 多位移位
        assert_eq!(calculate_expression("1 << 2 << 3", labels.clone()), Ok(32)); // 1 << 2 = 4, 4 << 3 = 32
        assert_eq!(calculate_expression("64 >> 2 >> 1", labels.clone()), Ok(8)); // 64 >> 2 = 16, 16 >> 1 = 8
    }

    #[test]
    fn test_calculate_expression_with_labels() {
        let mut labels = HashMap::new();
        labels.insert("label1".to_string(), 10);
        labels.insert("label2".to_string(), 20);
        labels.insert("addr".to_string(), 0x1000);

        assert_eq!(calculate_expression("label1 + label2", labels.clone()), Ok(30));
        assert_eq!(calculate_expression("addr + 0x20", labels.clone()), Ok(0x1020));
        assert_eq!(calculate_expression("(addr >> 4) + 1", labels.clone()), Ok(0x101));
    }

    #[test]
    fn test_calculate_expression_different_bases() {
        let labels = HashMap::new();
        // 十六进制
        assert_eq!(calculate_expression("0x10 + 0x20", labels.clone()), Ok(0x30));
        assert_eq!(calculate_expression("0xFF >> 4", labels.clone()), Ok(0xF));

        // 八进制（可选测试）
        assert_eq!(calculate_expression("0o10 + 0o20", labels.clone()), Ok(0o30));

        // 二进制
        assert_eq!(calculate_expression("0b1100 >> 2", labels.clone()), Ok(0b11));

        // 混合进制
        assert_eq!(calculate_expression("0x10 + 16", labels.clone()), Ok(0x20));
    }

    #[test]
    fn test_calculate_expression_edge_cases() {
        let labels = HashMap::new();
        // 除零错误
        assert!(calculate_expression("10 / 0", labels.clone()).is_err());
        assert!(calculate_expression("10 % 0", labels.clone()).is_err());

        // 溢出检查
        assert!(calculate_expression("0xFFFFFFFF + 1", labels.clone()).is_err()); // 超过u32范围

        // 负数（可能不被支持）
        assert!(calculate_expression("-1", labels.clone()).is_err());

        // 无效语法
        assert!(calculate_expression("1 + + 2", labels.clone()).is_err());
        assert!(calculate_expression("1 * * 2", labels.clone()).is_err());
        assert!(calculate_expression("(1 + 2", labels.clone()).is_err()); // 缺少右括号
        assert!(calculate_expression("1 + 2)", labels.clone()).is_err()); // 缺少左括号

        // 无效符号
        assert!(calculate_expression("1 & 2", labels.clone()).is_err()); // & 不支持
        assert!(calculate_expression("1 | 2", labels.clone()).is_err()); // | 不支持
    }

    #[test]
    fn test_calculate_expression_advanced() {
        let labels = HashMap::new();

        // 复杂嵌套表达式
        assert_eq!(calculate_expression("(1 + 2) * (3 + 4)", labels.clone()), Ok(21));
        assert_eq!(calculate_expression("((1 << 2) + (3 >> 1)) * 2", labels.clone()), Ok(10)); // (4 + 1) * 2 = 10

        // 多层括号
        assert_eq!(calculate_expression("((((1 + 1))))", labels.clone()), Ok(2));

        // 运算符优先级验证
        assert_eq!(calculate_expression("1 + 2 << 3", labels.clone()), Ok(24)); // (1 + 2) << 3 = 3 << 3 = 24? 不，+优先级高于<<? 实际上<<优先级低于+，所以是(1+2)<<3=24
        assert_eq!(calculate_expression("1 << 2 + 3", labels.clone()), Ok(32)); // 1 << (2 + 3) = 1 << 5 = 32

        // 空格处理
        assert_eq!(calculate_expression("1+2", labels.clone()), Ok(3));
        assert_eq!(calculate_expression("1 + 2", labels.clone()), Ok(3));
        assert_eq!(calculate_expression("  1   +   2  ", labels.clone()), Ok(3));
        assert_eq!(calculate_expression("( 1 + 2 ) * 3", labels.clone()), Ok(9));

        // 大数计算
        assert_eq!(calculate_expression("0xFFFF * 0xFFFF", labels.clone()), Ok(0xFFFE0001)); // 需要检查是否在u32范围内
        assert_eq!(calculate_expression("0xFFFFFFFF / 0xFFFF", labels.clone()), Ok(0x10001));

        // 位移边界
        assert_eq!(calculate_expression("1 << 31", labels.clone()), Ok(0x80000000));
        assert_eq!(calculate_expression("0x80000000 >> 31", labels.clone()), Ok(1));

        // 除法和取模
        assert_eq!(calculate_expression("7 / 2", labels.clone()), Ok(3)); // 整数除法
        assert_eq!(calculate_expression("7 % 2", labels.clone()), Ok(1));

        // 表达式中的零
        assert_eq!(calculate_expression("0 * 100", labels.clone()), Ok(0));
        assert_eq!(calculate_expression("100 + 0", labels.clone()), Ok(100));
        assert_eq!(calculate_expression("0 >> 5", labels.clone()), Ok(0));
        assert_eq!(calculate_expression("0 << 5", labels.clone()), Ok(0));
    }
}

