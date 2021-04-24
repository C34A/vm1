use std::env;
use std::fs;

use vm1::isa::*;
use vm1::interpreter::Interpreter;
use vm1::vm1_raylib;

use vm1::assembler;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        None => {
            println!("no arguments given.");
            return;
        },
        Some(s) => {
            match &s[..] {
                "compilerun" => {
                    let filename = args.get(2).expect("expected filename in 3rd argument");
                    let file = fs::File::open(filename).expect(&format!("failed to read file: {}", filename)[..]);
                    let code = assembler::gen_code_read(file);
                    match code {
                        Ok(result) => {
                            // good to go!
                            run(&result)
                        },
                        Err(e) => {
                            // something went wrong...
                            println!("{}", e);
                            return;
                        }
                    };
                },
                _ => {
                    println!("unrecognized argument: {}", s);
                    return;
                }
            }
        }
    }
}

fn run(code: &Vec<Instruction>) {
    let mut vm = Interpreter::new(code);
    vm1_raylib::run(&mut vm);
}
