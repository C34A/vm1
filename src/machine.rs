use std::collections::HashMap;

#[derive(Copy, Clone)]
pub struct Val {
    pub contents: i32
}

impl Val {
    pub fn new() -> Val {
        Val {contents: 0}
    }

    pub fn from(a: i32) -> Val {
        Val {contents: a}
    }
}

pub struct Registry {
    data: [Val; 256],
}

impl Registry {
    pub fn new() -> Registry {
        Registry {
            data: [Val::new(); 256],
        }
    }

    pub fn get(&self, reg: u8) -> Val {
        self.data[reg as usize]
    }

    pub fn put(&mut self, reg: u8, data: Val) {
        self.data[reg as usize] = data;
    }
}

pub struct Ram {
    data: HashMap<u32, Val>,
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            data: HashMap::new(),
        }
    }

    pub fn load(&self, addr: u32) -> Val {
        self.data[&addr]
    }

    pub fn store(&mut self, addr: u32, val: Val) {
        self.data.insert(addr, val);
    }
}