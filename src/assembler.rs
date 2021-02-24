use std::io::Read;

use crate::isa::*;

pub fn gen_code_read(mut text: impl Read) -> Vec<Instruction> {
    let mut textstr = String::new();
    text.read_to_string(&mut textstr);
    gen_code(&textstr)
}

pub fn gen_code(mut text: &String) -> Vec<Instruction> {

    let mut ret = vec![];

    for line in text.split('\n') {
        let (trim, _) = match line.find(';') {
            None => (line, ""),
            Some(idx) => line.split_at(idx),
        };

        let mut iter = trim.split_ascii_whitespace();
        let instr = iter.next();
        match instr {
            None => {continue;},
            Some(s) => match s {
                "nop" => {ret.push(Instruction::None);},
                "set" => {
                    let reg1 = get_register_or_panic(iter.next().unwrap());
                    let val = parse_val(iter.next().unwrap());
                    ret.push(
                        Instruction::Set {
                            reg: RegAddr {idx: reg1},
                            val: val
                        }
                    )
                }
                "add" => {
                    let reg1 = get_register_or_panic(iter.next().unwrap());
                    let reg2 = get_register_or_panic(iter.next().unwrap());
                    ret.push(
                        Instruction::Add{
                            a: RegAddr {idx: reg1},
                            b: RegAddr {idx: reg2}
                        }
                    );
                },
                "sub" => {
                    let reg1 = get_register_or_panic(iter.next().unwrap());
                    let reg2 = get_register_or_panic(iter.next().unwrap());
                    ret.push(
                        Instruction::Sub {
                            a: RegAddr {idx: reg1},
                            b: RegAddr {idx: reg2}
                        }
                    );
                },
                "mult" => {
                    let reg1 = get_register_or_panic(iter.next().unwrap());
                    let reg2 = get_register_or_panic(iter.next().unwrap());
                    ret.push(
                        Instruction::Mult {
                            a: RegAddr {idx: reg1},
                            b: RegAddr {idx: reg2}
                        }
                    );
                },
                "div" => {
                    let reg1 = get_register_or_panic(iter.next().unwrap());
                    let reg2 = get_register_or_panic(iter.next().unwrap());
                    ret.push(
                        Instruction::Div {
                            a: RegAddr {idx: reg1},
                            b: RegAddr {idx: reg2}
                        }
                    );
                },
                "rem" => {
                    let reg1 = get_register_or_panic(iter.next().unwrap());
                    let reg2 = get_register_or_panic(iter.next().unwrap());
                    ret.push(
                        Instruction::Rem {
                            a: RegAddr {idx: reg1},
                            b: RegAddr {idx: reg2}
                        }
                    );
                },
                "inc" => {
                    let reg1 = get_register_or_panic(iter.next().unwrap());
                    let val = parse_val(iter.next().unwrap());
                    ret.push(
                        Instruction::Inc {
                            reg: RegAddr {idx: reg1},
                            val: val,
                        }
                    );
                },
                "dec" => {
                    let reg1 = get_register_or_panic(iter.next().unwrap());
                    let val = parse_val(iter.next().unwrap());
                    ret.push(
                        Instruction::Dec {
                            reg: RegAddr {idx: reg1},
                            val: val,
                        }
                    );
                },
                "load" => {
                    let thing1 = iter.next().unwrap();
                    match get_register(thing1) {
                        Some(reg) => {
                            let thing2 = get_register_or_panic(iter.next().unwrap());
                            ret.push(
                                Instruction::LoadDeref {
                                    areg: RegAddr {idx: reg},
                                    dreg: RegAddr {idx: thing2}
                                }
                            );
                        },
                        None => {
                            let addr = parse_val(thing1) as u16;
                            let thing2 = get_register_or_panic(iter.next().unwrap());
                            ret.push(
                                Instruction::Load {
                                    addr: Addr {addr: addr},
                                    reg: RegAddr {idx: thing2}
                                }
                            );
                        }
                    }
                },
                "store" => {
                    let reg1 = get_register_or_panic(iter.next().unwrap());
                    let thing2 = iter.next().unwrap();
                    match get_register(thing2) {
                        Some(reg) => {
                            let reg2 = get_register(thing2).expect(&format!("failed to get register: {}", line)[..]);
                            ret.push(
                                Instruction::StoreDeref {
                                    dreg: RegAddr {idx: reg1},
                                    areg: RegAddr {idx: reg2}
                                }
                            );
                        },
                        None => {
                            let addr = parse_val(thing2) as u16;
                            ret.push(
                                Instruction::Store {
                                    addr: Addr {addr: addr},
                                    reg: RegAddr {idx: reg1}
                                }
                            );
                        }
                    }
                },
                "jmp" => {
                    let addr = parse_caddr_or_panic(iter.next().unwrap(), line);
                    ret.push(
                        Instruction::Jmp {
                            addr: addr
                        }
                    );
                },
                "jeq" => {
                    let reg1 = get_register_or_panic(iter.next().unwrap());
                    let reg2 = get_register_or_panic(iter.next().unwrap());
                    let addr = parse_caddr_or_panic(iter.next().unwrap(), line);
                    ret.push(
                        Instruction::Jeq {
                            a: RegAddr {idx: reg1},
                            b: RegAddr {idx: reg2},
                            addr: addr
                        }
                    );
                },
                "jgt" => {
                    let reg1 = get_register_or_panic(iter.next().unwrap());
                    let reg2 = get_register_or_panic(iter.next().unwrap());
                    let addr = parse_caddr_or_panic(iter.next().unwrap(), line);
                    ret.push(
                        Instruction::Jgt {
                            a: RegAddr {idx: reg1},
                            b: RegAddr {idx: reg2},
                            addr: addr
                        }
                    );
                },
                "jlt" => {
                    let reg1 = get_register_or_panic(iter.next().unwrap());
                    let reg2 = get_register_or_panic(iter.next().unwrap());
                    let addr = parse_caddr_or_panic(iter.next().unwrap(), line);
                    ret.push(
                        Instruction::Jlt {
                            a: RegAddr {idx: reg1},
                            b: RegAddr {idx: reg2},
                            addr: addr
                        }
                    );
                },
                "print" => {
                    let addr = parse_addr_or_panic(iter.next().unwrap(), line);
                    ret.push(
                        Instruction::Print {
                            addr: addr
                        }
                    );
                },
                "printr" => {
                    let reg1 = get_register_or_panic(iter.next().unwrap());
                    ret.push(
                        Instruction::PrintR {
                            reg: RegAddr {idx: reg1}
                        }
                    );
                }
                "draw" => {
                    ret.push(Instruction::Draw);
                }
                _ => {panic!("syntax error: {}", line);}
            }
        }
    }

    ret
}

fn parse_val(text: &str) -> i32 {
    text.parse::<i32>().expect("failed to parse int val")
}

fn get_register_or_panic(text: &str) -> u8 {
    let (first, rest) = text.split_at(1);
    if first.as_bytes()[0] as char != 'r' {
        panic!("syntax error- expected register: {}", text);
    } else {
        rest.parse::<u8>().expect("failed to parse register")
    }
}

fn get_register(text: &str) -> Option<u8> {
    let (first, rest) = text.split_at(1);
    if first.as_bytes()[0] as char != 'r' {
        None
    } else {
        Some(rest.parse::<u8>().expect("failed to parse register"))
    }
}

fn parse_caddr_or_panic(text: &str, line: &str) -> CAddr {
    let maybe_addr = text.parse::<u16>();
        match maybe_addr {
            Ok(addr) => {
                CAddr {caddr: addr}
            },
            Err(e) => {
                panic!("syntax error- expected code address: {}\n{}", line, e);
            }
        }
}

fn parse_addr_or_panic(text: &str, line: &str) -> Addr {
    let maybe_addr = text.parse::<u16>();
        match maybe_addr {
            Ok(addr) => {
               Addr {addr: addr}
            },
            Err(e) => {
                panic!("syntax error- expected code address: {}\n{}", line, e);
            }
        }
}