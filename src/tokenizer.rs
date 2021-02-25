
#[derive(Debug)]
pub enum Token {
    Label(String),
    Instr(String, Vec<Token>),
    Literal(i32),
    Reg(u8),
    AtReg(u8),
    AtLiteral(u16),
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

fn tokenize_line(mut line: &str) -> Option<Token> {
    match line.find(';') {
        None => (),
        Some(idx) => {
            line = line.split_at(idx).0;
        }
    }
    let mut lsplit = line.split_ascii_whitespace();
    match lsplit.next() {
        None => None,
        Some(s) => {
            if s.ends_with(':') {
                let (name, _) = s.split_at(s.len() - 1);
                return Some(Token::Label(String::from(name)));
            }
            let mut args: Vec<Token> = vec![];
            for thing in lsplit {
                match tokenize_one(thing) {
                    Err(e) => {
                        println!("failed to parse: {}\n{}", thing, e);
                    }, 
                    Ok(tok) => {
                        args.push(tok);
                    },
                }
            }
            return Some(Token::Instr(String::from(s), args));
        }
    }
}

fn tokenize_one(mut chunk: &str) -> Result<Token, String> {
    let mut is_addr = false;
    if chunk.starts_with('@') {
        is_addr = true;
        chunk = chunk.split_at(1).1;
    }
    println!("{}",chunk);
    if chunk.starts_with('r') { // is register
        chunk = chunk.split_at(1).1;
        println!("{}", chunk);
        let idx = chunk.parse::<u8>().expect("failed to parse register");
        match is_addr {
            true => Ok(Token::AtReg(idx)),
            false => Ok(Token::Reg(idx)),
        }
    } else {
        match is_addr {
            false => {
                let val = chunk.parse::<i32>().expect("failed to parse immediate value");
                Ok(Token::Literal(val))
            },
            true => {
                let val = chunk.parse::<u16>().expect("failed to parse immediate address value");
                Ok(Token::AtLiteral(val))
            }
        }
    }

}