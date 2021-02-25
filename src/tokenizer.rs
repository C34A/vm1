

pub enum Token {
    Label(String),
    Instr(String, Vec<Token>),
    Literal(i32),
    Reg(u8),
}

pub fn tokenize(mut text: &String) -> Vec<Token> {
    let mut ret = vec![];
    for line in text.split('\n') {
        let maybe_tok = tokenize_line(line);
        match maybe_tok {
            None => (),
            Some(tok) => ret.push(tok),
        }
    }
    ret
}

fn tokenize_line(line: &str) -> Option<Token> {
    let ret: Option<Token> = None;
    let lsplit = line.split_ascii_whitespace();
    match lsplit.next() {
        None => None,
        Some(s) => {
            if(s.ends_with(':')) {
                let (name, _) = s.split_at(s.len() - 1);
                return Some(Token::Label(String::from(s)));
            }
            let mut args: Vec<Token> = vec![];
            for thing in lsplit {
                if thing.starts_with(';') {break;}
                
            }
        }
    }
}