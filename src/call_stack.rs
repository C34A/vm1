pub struct CallStack {
    stack: Vec<u16>
}

pub enum CallStackCommand {
    Increment,
    Jump,
    JumpSubroutine,
}

impl CallStack {
    pub fn new() -> CallStack {
        CallStack {
            stack: vec![]
        }
    }

    pub fn get_current_addr(&self) -> u16 {
        *self.get_current_addr_ref()
    }

    pub fn get_current_addr_ref(&self) -> &u16 {
        self.stack.last().expect("Unable to get exec address because call stack is empty!")
    }

    pub fn handle_cmd(&mut self, cmd: CallStackCommand) {
        let current_r = self.get_current_addr_ref();
        match cmd {
            CallStackCommand::Increment => {
                current_r = 
            }
        }
    }
}