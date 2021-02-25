use std::env;
use std::fs;

use vm1::isa::*;
use vm1::interpreter::Interpreter;
use vm1::raylib_run;

use vm1::assembler;

fn main() -> Result<(), String>{
    // let code1 = vec![
    //     Instruction::Set {reg: RegAddr {idx: 0}, val: 1},
    //     Instruction::Inc {reg: RegAddr {idx: 0}, val: 64},
    //     Instruction::Store {reg: RegAddr {idx: 0}, addr: Addr {addr: 1}},
    //     Instruction::Set {reg: RegAddr {idx: 1}, val: 31167},
    //     Instruction::Set {reg: RegAddr {idx: 2}, val: 32767},
    //     Instruction::StoreDeref {dreg: RegAddr {idx: 0}, areg: RegAddr {idx: 1}},
    //     Instruction::Inc {reg: RegAddr {idx: 1}, val: 1},
    //     Instruction::Jlt {a: RegAddr {idx: 1}, b: RegAddr {idx: 2}, addr: CAddr {caddr: 5}},
    //     Instruction::Draw,
    // ];

    // let codestr = String::from("set r0 1
    //                             inc r0 64
    //                             store r0 1
    //                             set r1 31167
    //                             set r2 32767
    //                             set r4 91
    //                             store r0 r1
    //                             inc  r1 1
    //                             inc r0 1
    //                             jlt r0 r4 11
    //                             set r0 65
    //                             jlt r1 r2 6
    //                             draw
    //                             ");
    let args: Vec<String> = env::args().collect();

    println!("{:?}",args);

    match args.get(1) {
        None => {
            return Err(String::from("no arguments given."))
        },
        Some(s) => {
            match &s[..] {
                "compilerun" => {
                    let filename = args.get(2).expect("expected filename in 3rd argument");
                    let file = fs::File::open(filename).expect(&format!("failed to read file: {}", filename)[..]);
                    let code = assembler::gen_code_read(file)?;
                    run(&code);
                },
                _ => {
                    return Err(format!("unrecognized argument: {}", s));
                }
            }
        }
    }

    Ok(())
}

fn run(code: &Vec<Instruction>) {
    let mut vm = Interpreter::new(code);
    raylib_run::run(&mut vm);
}
