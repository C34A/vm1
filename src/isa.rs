
#[derive(Copy, Clone, Debug)]
pub struct RegAddr {
    pub idx: u8
}

#[derive(Copy, Clone, Debug)]
pub struct Addr {
    pub addr: u16
}

#[derive(Copy, Clone, Debug)]
pub struct CAddr {
    pub caddr: u16
}

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    None,
    Set {reg: RegAddr, val: i32},
    Add {a: RegAddr, b: RegAddr},
    Sub {a: RegAddr, b: RegAddr},
    Inc {reg: RegAddr, val: i32},
    Dec {reg: RegAddr, val: i32},
    Mult {a: RegAddr, b: RegAddr},
    Div {a: RegAddr, b: RegAddr},
    Rem {a: RegAddr, b: RegAddr},
    Load {addr: Addr, reg: RegAddr},
    LoadDeref {areg: RegAddr, dreg: RegAddr},
    Store {reg: RegAddr, addr: Addr},
    StoreDeref {dreg: RegAddr, areg: RegAddr},
    Jmp {addr: CAddr},
    Jeq {a: RegAddr, b: RegAddr, addr: CAddr},
    Jgt {a: RegAddr, b: RegAddr, addr: CAddr},
    Jlt {a: RegAddr, b: RegAddr, addr: CAddr},
    Print {addr: Addr},
    PrintR {reg: RegAddr},
    Draw,
}