use std::env;
use std::fs;
// use std::io::Write;

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
                    let code = compile(filename);
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
                "compileprint" => {
                    let filename = args.get(2).expect("expected filename in 3rd argument");
                    let code = compile(filename);
                    match code {
                        Ok(result) => {
                            // good to go!
                            println!("{:?}", result);
                        },
                        Err(e) => {
                            // something went wrong...
                            println!("{}", e);
                            return;
                        }
                    };
                },
                "compile" => {
                    let filename = args.get(2).expect("expected filename in 3rd argument");
                    let code = compile(filename);
                    // slice of everything to get only name, not path and extension
                    let filename = filename
                        .split("/")
                        .last()
                        .unwrap();
                    let pos = filename.rfind(".");
                    let filename = match pos {
                        Some(i) => {
                            filename.split_at(i).0
                        },
                        None => filename
                    };
                    match code {
                        Ok(result) => {
                            // good to go!
                            let mut newfilename = filename.to_string();
                            newfilename.push_str(".bm1"); // append .bm1
                            // serialize
                            let binary = bincode::serialize(&result).expect("failed to serialize code!");
                            // open file
                            let mut output_file = fs::File::create(&newfilename).expect("failed to create output file");
                            // .. and write!
                            output_file.write_all(&binary).expect("failed to write to output file!");
                            println!("Successfully compiled to {}", newfilename);
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

fn compile(filename: &str) -> Result<Vec<Instruction>, String> {
    let file = fs::File::open(filename).expect(&format!("failed to read file: {}", filename)[..]);
    assembler::gen_code_read(file)
}