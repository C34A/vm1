use crate::machine::*;
use crate::isa::*;
use crate::call_stack::*;

pub struct Interpreter<'a> {
    rom: &'a Vec<Instruction>,
    ram: Ram,
    registry: Registry,
    stack: CallStack,
}

impl<'a> Interpreter<'a> {
    pub fn interpret(&mut self) {
        while self.stack.get_current_addr() < (self.rom.len() as u16) {
            self.interpret_one();
        }
    }

    //returns false when end of ROM is reached.
    pub fn interpret_one(&mut self) -> (bool, bool) {
        if self.stack.get_current_addr() >= self.rom.len() as u16 { return (false, true); }
        let (stack_op, draw) = interp_instr(self.rom[self.stack.get_current_addr() as usize], &mut self.ram, &mut self.registry);
        self.stack.handle_cmd(stack_op);
        (true, draw)
    }

    pub fn new(code: &'a Vec<Instruction>) -> Interpreter {
        Interpreter {
            rom: code,
            ram: Ram::new(),
            registry: Registry::new(),
            stack: CallStack::new(),
        }
    }

    pub fn get_framebuffer(&self) -> &[i32] {
        self.ram.get_framebuffer()
    }
}

fn interp_instr(inst: Instruction, ram: &mut Ram, registry: &mut Registry) -> (CallStackCommand, bool) {
    let mut draw = false;
    let stack_op = match inst {
        Instruction::Add {rega, regb} => {
            let a_val = registry.get(rega);
            let b_val = registry.get(regb);
            registry.put(rega, Val::from(a_val.contents + b_val.contents));
            CallStackCommand::Increment
        },
        Instruction::Sub {rega, regb} => {
            let a_val = registry.get(rega);
            let b_val = registry.get(regb);
            registry.put(rega, Val::from(a_val.contents - b_val.contents));
            CallStackCommand::Increment
        },
        Instruction::Inc {reg, val} => {
            let before = registry.get(reg);
            registry.put(reg, Val{contents: before.contents + val});
            CallStackCommand::Increment
        },
        Instruction::Dec {reg, val} => {
            let before = registry.get(reg);
            registry.put(reg, Val{contents: before.contents - val});
            CallStackCommand::Increment
        },
        Instruction::Div {rega, regb} => {
            let a_val = registry.get(rega);
            let b_val = registry.get(regb);
            registry.put(rega, Val::from(a_val.contents / b_val.contents));
            CallStackCommand::Increment
        },
        Instruction::Mult {rega, regb} => {
            let a_val = registry.get(rega);
            let b_val = registry.get(regb);
            registry.put(rega, Val::from(a_val.contents * b_val.contents));
            CallStackCommand::Increment
        },
        Instruction::Rem {rega, regb} => {
            let a_val = registry.get(rega);
            let b_val = registry.get(regb);
            registry.put(rega, Val::from(a_val.contents % b_val.contents));
            CallStackCommand::Increment
        },
        Instruction::Load {addr, reg} => {
            registry.put(reg, ram.load(addr));
            CallStackCommand::Increment
        },
        Instruction::LoadDeref {addr_reg, data_reg} => {
            let add = registry.get(addr_reg);
            registry.put(data_reg, ram.load(add.contents as u16));
            CallStackCommand::Increment
        },
        Instruction::Store {reg, addr} => {
            ram.store(addr, registry.get(reg));
            CallStackCommand::Increment
        },
        Instruction::StoreDeref {data_reg, addr_reg} => {
            let dat = registry.get(data_reg);
            let address = registry.get(addr_reg);
            ram.store(address.contents as u16, dat);
            CallStackCommand::Increment
        },
        Instruction::Jmp {addr} => {
            CallStackCommand::Jump(addr)
        },
        Instruction::Jeq {rega, regb, addr} => {
            let a_val = registry.get(rega);
            let b_val = registry.get(regb);
            if a_val.contents == b_val.contents {
                CallStackCommand::Jump(addr)
            } else {
                CallStackCommand::Increment
            }
        },
        Instruction::Jgt {rega, regb, addr} => {
            let a_val = registry.get(rega);
            let b_val = registry.get(regb);
            if a_val.contents > b_val.contents {
                CallStackCommand::Jump(addr)
            } else {
                CallStackCommand::Increment
            }
        },
        Instruction::Jlt {rega, regb, addr} => {
            let a_val = registry.get(rega);
            let b_val = registry.get(regb);
            if a_val.contents < b_val.contents {
                CallStackCommand::Jump(addr)
            } else {
                CallStackCommand::Increment
            }
        },
        Instruction::Jsr {caddr} => CallStackCommand::JumpSubroutine(caddr),
        Instruction::Rsr => CallStackCommand::ReturnSubroutine,
        Instruction::Print {addr} => {
            println!("{}", ram.load(addr).contents);
            CallStackCommand::Increment
        },
        Instruction::Set {reg, val} => {
            registry.put(reg, Val::from(val));
            CallStackCommand::Increment
        },
        Instruction::PrintR {reg} => {
            println!("{}", registry.get(reg).contents);
            CallStackCommand::Increment
        },
        Instruction::Draw => {
            draw = true;
            CallStackCommand::Increment
        }
        Instruction::None => CallStackCommand::Increment,
    };
    (stack_op, draw)
}