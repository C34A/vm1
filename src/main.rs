use vm1::isa::*;
use vm1::interpreter::Interpreter;
use vm1::raylib_run;

fn main() {
    let code = vec![
        Instruction::Set {reg: RegAddr {idx: 0}, val: 1},
        Instruction::Inc {reg: RegAddr {idx: 0}, val: 64},
        Instruction::Store {reg: RegAddr {idx: 0}, addr: Addr {addr: 1}},
        Instruction::Set {reg: RegAddr {idx: 1}, val: 31167},
        Instruction::Set {reg: RegAddr {idx: 2}, val: 32767},
        Instruction::StoreDeref {dreg: RegAddr {idx: 0}, areg: RegAddr {idx: 1}},
        Instruction::Inc {reg: RegAddr {idx: 1}, val: 1},
        Instruction::Jlt {a: RegAddr {idx: 1}, b: RegAddr {idx: 2}, addr: CAddr {caddr: 5}}
    ];

    let mut vm = Interpreter::new(&code);
    raylib_run::run(&mut vm);
}
