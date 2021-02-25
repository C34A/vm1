use crate::machine::*;
use crate::isa::*;

pub struct Interpreter<'a> {
    rom: &'a Vec<Instruction>,
    ram: Ram,
    registry: Registry,
    code_ptr: u16,
}

impl<'a> Interpreter<'a> {
    pub fn interpret(&mut self) {
        while self.code_ptr < (self.rom.len() as u16) {
            self.interpret_one();
        }
    }

    //returns false when end of ROM is reached.
    pub fn interpret_one(&mut self) -> (bool, bool) {
        if self.code_ptr >= self.rom.len() as u16 { return (false, true); }
        let (new_ptr, draw) = interp_instr(self.rom[self.code_ptr as usize], &mut self.ram, &mut self.registry);
        self.code_ptr = match new_ptr {
            None => self.code_ptr + 1,
            Some(addr) => addr,
        };
        (true, draw)
    }

    pub fn new(code: &'a Vec<Instruction>) -> Interpreter {
        Interpreter {
            rom: code,
            ram: Ram::new(),
            registry: Registry::new(),
            code_ptr: 0,
        }
    }

    pub fn get_framebuffer(&self) -> &[i32] {
        self.ram.get_framebuffer()
    }
}

fn interp_instr(inst: Instruction, ram: &mut Ram, registry: &mut Registry) -> (Option<u16>, bool) {
    let mut ret: Option<u16> = None;
    let mut draw = false;
    match inst {
        Instruction::Add {rega, regb} => {
            let a_val = registry.get(rega);
            let b_val = registry.get(regb);
            registry.put(rega, Val::from(a_val.contents + b_val.contents));
        },
        Instruction::Sub {rega, regb} => {
            let a_val = registry.get(rega);
            let b_val = registry.get(regb);
            registry.put(rega, Val::from(a_val.contents - b_val.contents));
        },
        Instruction::Inc {reg, val} => {
            let before = registry.get(reg);
            registry.put(reg, Val{contents: before.contents + val});
        },
        Instruction::Dec {reg, val} => {
            let before = registry.get(reg);
            registry.put(reg, Val{contents: before.contents - val});
        },
        Instruction::Div {rega, regb} => {
            let a_val = registry.get(rega);
            let b_val = registry.get(regb);
            registry.put(rega, Val::from(a_val.contents / b_val.contents));
        },
        Instruction::Mult {rega, regb} => {
            let a_val = registry.get(rega);
            let b_val = registry.get(regb);
            registry.put(rega, Val::from(a_val.contents * b_val.contents));
        },
        Instruction::Rem {rega, regb} => {
            let a_val = registry.get(rega);
            let b_val = registry.get(regb);
            registry.put(rega, Val::from(a_val.contents % b_val.contents));
        },
        Instruction::Load {addr, reg} => {
            registry.put(reg, ram.load(addr));
        },
        Instruction::LoadDeref {addr_reg, data_reg} => {
            let add = registry.get(addr_reg);
            registry.put(data_reg, ram.load(add.contents as u16));
        },
        Instruction::Store {reg, addr} => {
            ram.store(addr, registry.get(reg));
        },
        Instruction::StoreDeref {data_reg, addr_reg} => {
            let dat = registry.get(data_reg);
            let address = registry.get(addr_reg);
            ram.store(address.contents as u16, dat);
        },
        Instruction::Jmp {addr} => {
            ret = Some(addr);
        },
        Instruction::Jeq {rega, regb, addr} => {
            let a_val = registry.get(rega);
            let b_val = registry.get(regb);
            if a_val.contents == b_val.contents {
                ret = Some(addr);
            }
        },
        Instruction::Jgt {rega, regb, addr} => {
            let a_val = registry.get(rega);
            let b_val = registry.get(regb);
            if a_val.contents > b_val.contents {
                ret = Some(addr);
            }
        },
        Instruction::Jlt {rega, regb, addr} => {
            let a_val = registry.get(rega);
            let b_val = registry.get(regb);
            if a_val.contents < b_val.contents {
                ret = Some(addr);
            }
        },
        Instruction::Print {addr} => {
            println!("{}", ram.load(addr).contents);
        },
        Instruction::Set {reg, val} => {
            registry.put(reg, Val::from(val));
        },
        Instruction::PrintR {reg} => {
            println!("{}", registry.get(reg).contents);
        },
        Instruction::Draw => {draw = true;}
        Instruction::None => {},
    };
    (ret, draw)
}