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
    pub fn interpret_one(&mut self) -> bool {
        if self.code_ptr >= self.rom.len() as u16 { return false; }
        let ret = interp_instr(self.rom[self.code_ptr as usize], &mut self.ram, &mut self.registry);
        self.code_ptr = match ret {
            None => self.code_ptr + 1,
            Some(addr) => addr,
        };
        true
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

fn interp_instr(inst: Instruction, ram: &mut Ram, registry: &mut Registry) -> Option<u16> {
    let mut ret: Option<u16> = None;
    match inst {
        Instruction::Add {a, b} => {
            let a_val = registry.get(a.idx);
            let b_val = registry.get(b.idx);
            registry.put(a.idx, Val::from(a_val.contents + b_val.contents));
        },
        Instruction::Sub {a, b} => {
            let a_val = registry.get(a.idx);
            let b_val = registry.get(b.idx);
            registry.put(a.idx, Val::from(a_val.contents - b_val.contents));
        },
        Instruction::Inc {reg, val} => {
            let before = registry.get(reg.idx);
            registry.put(reg.idx, Val{contents: before.contents + val});
        },
        Instruction::Dec {reg, val} => {
            let before = registry.get(reg.idx);
            registry.put(reg.idx, Val{contents: before.contents - val});
        },
        Instruction::Div {a, b} => {
            let a_val = registry.get(a.idx);
            let b_val = registry.get(b.idx);
            registry.put(a.idx, Val::from(a_val.contents / b_val.contents));
        },
        Instruction::Mult {a, b} => {
            let a_val = registry.get(a.idx);
            let b_val = registry.get(b.idx);
            registry.put(a.idx, Val::from(a_val.contents * b_val.contents));
        },
        Instruction::Rem {a, b} => {
            let a_val = registry.get(a.idx);
            let b_val = registry.get(b.idx);
            registry.put(a.idx, Val::from(a_val.contents % b_val.contents));
        },
        Instruction::Load {addr, reg} => {
            registry.put(reg.idx, ram.load(addr.addr));
        },
        Instruction::LoadDeref {areg, dreg} => {
            let add = registry.get(areg.idx);
            registry.put(dreg.idx, ram.load(add.contents as u16));
        },
        Instruction::Store {reg, addr} => {
            ram.store(addr.addr, registry.get(reg.idx));
        },
        Instruction::StoreDeref {dreg, areg} => {
            let dat = registry.get(dreg.idx);
            let address = registry.get(areg.idx);
            ram.store(address.contents as u16, dat);
        },
        Instruction::Jmp {addr} => {
            ret = Some(addr.caddr);
        },
        Instruction::Jeq {a, b, addr} => {
            let a_val = registry.get(a.idx);
            let b_val = registry.get(b.idx);
            if a_val.contents == b_val.contents {
                ret = Some(addr.caddr);
            }
        },
        Instruction::Jgt {a, b, addr} => {
            let a_val = registry.get(a.idx);
            let b_val = registry.get(b.idx);
            if a_val.contents > b_val.contents {
                ret = Some(addr.caddr);
            }
        },
        Instruction::Jlt {a, b, addr} => {
            let a_val = registry.get(a.idx);
            let b_val = registry.get(b.idx);
            println!("a val: {} b val: {}", a_val.contents, b_val.contents);
            if a_val.contents < b_val.contents {
                println!("less than!");
                ret = Some(addr.caddr);
            }
        },
        Instruction::Print {addr} => {
            println!("{}", ram.load(addr.addr).contents);
        },
        Instruction::Set {reg, val} => {
            registry.put(reg.idx, Val::from(val));
        },
        Instruction::PrintR {reg} => {
            println!("{}", registry.get(reg.idx).contents);
        },
        Instruction::None => {},
    };
    ret
}