use std::io::Read;

use std::collections::HashMap;

use crate::isa::*;
use crate::tokenizer::*;

pub fn gen_code_read(mut text: impl Read) -> Result<Vec<Instruction>, String> {
    let mut textstr = String::new();
    let _ = text.read_to_string(&mut textstr);
    gen_code(&textstr)
}

pub fn gen_code(text: &String) -> Result<Vec<Instruction>, String> {
    let tokens = tokenize(text);
    compile(&tokens)
}

pub fn compile(tokens: &Vec<Token>) -> Result<Vec<Instruction>, String> {
    let mut label_map: HashMap<String, u16> = HashMap::new();
    for (idx, token) in tokens.iter().enumerate() {
        match token {
            Token::Label(name) => {
                if label_map.contains_key(name) {
                    return Err(format!("ERROR: label {} defined multiple times.", name));
                } else {
                    label_map.insert(String::from(name) , idx as u16);
                }
            },
            _ => ()
            // {
            //     return Err(format!("ERROR: unexpected token (label): {:?}", token));
            // },
        }
    }

    let mut ret: Vec<Instruction> = vec![];

    for token in tokens {
        match token {
            Token::Label(_) => {
                ret.push(Instruction::None);
            },
            Token::Instr(instr, args) => {
                ret.push(compile_instr(&instr, &args, &label_map)?)
            },
            _ => return Err(format!("ERROR: unexpected token {:?}", token)),
        }
    }

    Ok(ret)
}

fn compile_instr(inst_name: &String, args: &Vec<Token>, labels: &HashMap<String, u16>) -> Result<Instruction, String> {
    match &inst_name[..] {
        "nop" => Ok(Instruction::None),
        "set" => {
            let reg = try_get_reg(args.get(0))?;
            let val = try_get_lit(args.get(1))?;
            Ok(Instruction::Set {reg: reg, val: val})
        },
        "add" => {
            let rega = try_get_reg(args.get(0))?;
            let regb = try_get_reg(args.get(1))?;
            Ok(Instruction::Add {rega: rega, regb: regb})
        },
        "sub" => {
            let rega = try_get_reg(args.get(0))?;
            let regb = try_get_reg(args.get(1))?;
            Ok(Instruction::Sub {rega: rega, regb: regb})
        },
        "mult" => {
            let rega = try_get_reg(args.get(0))?;
            let regb = try_get_reg(args.get(1))?;
            Ok(Instruction::Mult {rega: rega, regb: regb})
        },
        "div" => {
            let rega = try_get_reg(args.get(0))?;
            let regb = try_get_reg(args.get(1))?;
            Ok(Instruction::Div {rega: rega, regb: regb})
        },
        "rem" => {
            let rega = try_get_reg(args.get(0))?;
            let regb = try_get_reg(args.get(1))?;
            Ok(Instruction::Rem {rega: rega, regb: regb})
        },
        "inc" => {
            let reg = try_get_reg(args.get(0))?;
            let val = try_get_lit(args.get(1))?;
            Ok(Instruction::Inc {reg: reg, val: val})
        },
        "dec" => {
            let reg = try_get_reg(args.get(0))?;
            let val = try_get_lit(args.get(1))?;
            Ok(Instruction::Dec {reg: reg, val: val})
        },
        "load" => {
            let first = args.get(0);
            let maybe_addr_reg = try_get_addr_reg(first);
            let data_reg = try_get_reg(args.get(1))?;
            match maybe_addr_reg {
                Ok(reg) => {
                    Ok(Instruction::LoadDeref {addr_reg: reg, data_reg: data_reg})
                },
                Err(_) => {
                    let maybe_addr = try_get_addr_lit(first, labels);
                    match maybe_addr {
                        Ok(addr) => {
                            Ok(Instruction::Load {addr: addr, reg: data_reg})
                        }
                        Err(_) => {
                            Err(String::from("ERROR: load - expected address or @register"))
                        }
                    }
                }
            }
        },
        "store" => {
            let data_reg = try_get_reg(args.get(0))?;
            let second = args.get(1);
            let maybe_addr_reg = try_get_addr_reg(second);
            match maybe_addr_reg {
                Ok(reg) => {
                    Ok(Instruction::StoreDeref {addr_reg: reg, data_reg: data_reg})
                },
                Err(_) => {
                    let maybe_addr = try_get_addr_lit(second, labels);
                    match maybe_addr {
                        Ok(addr) => {
                            Ok(Instruction::Store {addr: addr, reg: data_reg})
                        }
                        Err(_) => {
                            Err(String::from("ERROR: store - expected address or @register"))
                        }
                    }
                }
            }
        },
        "jmp" => {
            let addr = try_get_addr_lit(args.get(0), labels)?;
            Ok(Instruction::Jmp {addr: addr})
        },
        "jeq" => {
            let rega = try_get_reg(args.get(0))?;
            let regb = try_get_reg(args.get(1))?;
            let addr = try_get_addr_lit(args.get(2), labels)?;
            Ok(Instruction::Jeq {rega: rega, regb: regb, addr: addr})
        },
        "jgt" => {
            let rega = try_get_reg(args.get(0))?;
            let regb = try_get_reg(args.get(1))?;
            let addr = try_get_addr_lit(args.get(2), labels)?;
            Ok(Instruction::Jgt {rega: rega, regb: regb, addr: addr})
        },
        "jlt" => {
            let rega = try_get_reg(args.get(0))?;
            let regb = try_get_reg(args.get(1))?;
            let addr = try_get_addr_lit(args.get(2), labels)?;
            Ok(Instruction::Jlt {rega: rega, regb: regb, addr: addr})
        },
        "print" => {
            let first = args.get(0);
            match try_get_addr_lit(first, labels) {
                Ok(addr) => Ok(Instruction::Print {addr: addr}),
                Err(_) => match try_get_reg(first) {
                    Ok(reg) => Ok(Instruction::PrintR {reg: reg}),
                    Err(_) => {
                        Err(String::from("ERROR: print expected register or address"))
                    }
                }
            }
        },
        "draw" => Ok(Instruction::Draw),
        _ => Err(format!("ERROR: instruction not recognized: {}", inst_name))
    }
}

fn try_get_reg(maybe_token: Option<&Token>) -> Result<u8, String> {
    match maybe_token {
        None => return Err(String::from("ERROR: not enough args, expected register")),
        Some(tok) => {
            match *tok {
                Token::Reg(idx) => Ok(idx),
                _ => return Err(format!("ERROR: expected register, found {:?}", tok)),
            }
        }
    }
}

fn try_get_addr_reg(maybe_token: Option<&Token>) -> Result<u8, String> {
    match maybe_token {
        None => return Err(String::from("ERROR: not enough args, expected @register")),
        Some(tok) => {
            match *tok {
                Token::AtReg(idx) => Ok(idx),
                _ => return Err(format!("ERROR: expected @register, found {:?}", tok)),
            }
        }
    }
}

fn try_get_lit(maybe_token: Option<&Token>) -> Result<i32, String> {
    match maybe_token {
        None => return Err(String::from("ERROR: not enough args, expected address")),
        Some(tok) => {
            match *tok {
                Token::Literal(val) => Ok(val),
                _ => return Err(format!("ERROR: expected literal, found {:?}", tok)),
            }
        }
    }
}

fn try_get_addr_lit(maybe_token: Option<&Token>, labels: &HashMap<String, u16>) -> Result<u16, String> {
    match maybe_token {
        None => return Err(String::from("ERROR: not enough args, expected address")),
        Some(tok) => {
            match tok {
                Token::AtLiteral(addr) => Ok(*addr),
                Token::LabelAddr(name) => {
                    match labels.get(name) {
                        Some(addr) => Ok(*addr),
                        None => Err(format!("ERROR: unrecognized label: {}", name))
                    }
                },
                _ => return Err(format!("ERROR: expected @address, found {:?}", tok)),
            }
        }
    }
}