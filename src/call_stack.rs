pub struct CallStack {
    stack: Vec<u16>
}

pub enum CallStackCommand {
    Increment,
    Jump(u16),
    JumpSubroutine(u16),
    ReturnSubroutine,
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
        let current = self.stack.pop().expect("Unable to get current address because stack is empty!");
        match cmd {
            CallStackCommand::Increment => {
                self.stack.push(current + 1); // push the next address to the stack
            },
            CallStackCommand::Jump(addr) => {
                self.stack.push(addr); // the current address remains popped and the new address replaces it
            },
            CallStackCommand::JumpSubroutine(addr) => {
                self.stack.push(current + 1);
                self.stack.push(addr);
            },
            CallStackCommand::ReturnSubroutine => {
                (); 
                // current addr is already popped, so doing nothing leaves the previous
                // jsr address at the top of the stack
            }
        }
    }
}