use crate::machine::*;
use crate::isa::*;

pub fn interpret(rom: &Vec<Instruction>) {
    let mut code_ptr: u16 = 0;
    let mut ram = Ram::new();
    let mut registry = Registry::new();

    while code_ptr < (rom.len() as u16) {
        let ret = interp_instr(rom[code_ptr as usize], &mut ram, &mut registry);
        code_ptr = match ret {
            None => code_ptr + 1,
            Some(addr) => addr,
        };
    }
}

pub fn interp_instr(inst: Instruction, ram: &mut Ram, registry: &mut Registry) -> Option<u16> {
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
        Instruction::Store {reg, addr} => {
            ram.store(addr.addr, registry.get(reg.idx));
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
            if a_val.contents < b_val.contents {
                ret = Some(addr.caddr);
            }
        },
        Instruction::Print {addr} => {
            println!("{}", ram.load(addr.addr).contents);
        },
        Instruction::Set {reg, val} => {
            registry.put(reg.idx, Val::from(val));
        }
        Instruction::None => {},
    };
    ret
}