use vm1::isa::*;
use vm1::interpreter::*;

fn main() {
    let code = vec![
        Instruction::Set {reg: RegAddr {idx: 0}, val: 1},
        Instruction::Set {reg: RegAddr {idx: 1}, val: 1},
        Instruction::Add {a: RegAddr {idx: 0}, b: RegAddr {idx: 1}},
        Instruction::Store {reg: RegAddr {idx: 0}, addr: Addr {addr: 1}},
        Instruction::Print {addr: Addr {addr: 1}}
    ];

    interpret(&code);
}
