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
    data: HashMap<u16, Val>,
    framebuffer: [i32; 1600],
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            data: HashMap::new(),
            framebuffer: [0; 1600],
        }
    }

    pub fn load(&self, addr: u16, rayhandle: &raylib::RaylibHandle) -> Val {
        if addr >= 0xF980 {
            if addr >= 0xF9BF {
                return Val::from(self.framebuffer[(addr - 0xF9BF) as usize]);
            } else {
                let (down, justchanged) = crate::vm1_raylib::keyboard::get_key_status((addr - 0xF980) as u8, rayhandle);
                return Val::from((justchanged as i32 * 2) + (down as i32 * 2)) // bottom bit is if down, second from bottom is if just changed
            }
        }
        self.data[&addr]
    }

    pub fn store(&mut self, addr: u16, val: Val) {
        if addr >= 0xF980 && addr <= 0xF9BE {
            return; // don't do anything; this is the keyboard
        }
        if addr >= 0xF9BF {
            self.framebuffer[(addr - 0xF9BF) as usize] = val.contents;
        }
        self.data.insert(addr, val);
    }

    pub fn get_framebuffer(&self) -> &[i32] {
        &self.framebuffer
    }
}

pub fn val_to_char(val: i32) -> char {
    match val {
        0 => ' ',
        _ => val as u8 as char,
    }
}