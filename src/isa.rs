
#[derive(Copy, Clone)]pub struct RegAddr {
    pub idx: u8
}

#[derive(Copy, Clone)]
pub struct Addr {
    pub addr: u32
}

#[derive(Copy, Clone)]
pub struct CAddr {
    pub caddr: u16
}

#[derive(Copy, Clone)]
pub enum Instruction {
    None,
    Set {reg: RegAddr, val: i32},
    Add {a: RegAddr, b: RegAddr},
    Sub {a: RegAddr, b: RegAddr},
    Mult {a: RegAddr, b: RegAddr},
    Div {a: RegAddr, b: RegAddr},
    Rem {a: RegAddr, b: RegAddr},
    Load {addr: Addr, reg: RegAddr},
    Store {reg: RegAddr, addr: Addr},
    Jmp {addr: CAddr},
    Jeq {a: RegAddr, b: RegAddr, addr: CAddr},
    Jgt {a: RegAddr, b: RegAddr, addr: CAddr},
    Jlt {a: RegAddr, b: RegAddr, addr: CAddr},
    Print {addr: Addr},
}