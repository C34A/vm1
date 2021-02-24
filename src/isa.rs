

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    None,
    Set {reg: u8, val: i32},
    Add {rega: u8, regb: u8},
    Sub {rega: u8, regb: u8},
    Inc {reg: u8, val: i32},
    Dec {reg: u8, val: i32},
    Mult {rega: u8, regb: u8},
    Div {rega: u8, regb: u8},
    Rem {rega: u8, regb: u8},
    Load {addr: u16, reg: u8},
    LoadDeref {addr_reg: u8, data_reg: u8},
    Store {reg: u8, addr: u16},
    StoreDeref {data_reg: u8, addr_reg: u8},
    Jmp {addr: u16},
    Jeq {rega: u8, regb: u8, addr: u16},
    Jgt {rega: u8, regb: u8, addr: u16},
    Jlt {rega: u8, regb: u8, addr: u16},
    Print {addr: u16},
    PrintR {reg: u8},
    Draw,
}