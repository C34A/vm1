
#[derive(Debug)]
pub enum Token {
    Label(String),
    Instr(String, Vec<Token>),
    Literal(i32),
    Reg(u8),
    AtReg(u8),
    AtLiteral(u16),
    LabelAddr(String),
}

pub fn tokenize(text: &String) -> Vec<Token> {
    let mut ret = vec![];
    let mut line_idx = 0;
    for line in text.split('\n') {
        line_idx += 1;
        let maybe_tok = tokenize_line(line, line_idx);
        match maybe_tok {
            None => (),
            Some(tok) => ret.push(tok),
        }
    }
    ret
}

fn tokenize_line(mut line: &str, idx: i32) -> Option<Token> {
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
                        println!("line {} - failed to parse: {}\n{}", idx, thing, e);
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

fn tokenize_one(mut chunk: &str) -> Result<Token, String> { // this is serious ass, though works for now
                                                            // TODO: rewrite this and make it not ass
    let mut is_addr = false;
    if chunk.starts_with('@') {
        is_addr = true;
        chunk = chunk.split_at(1).1;
    }

    if chunk.starts_with('r') { // is register
        chunk = chunk.split_at(1).1;
        // println!("{}", chunk);
        let idx = chunk.parse::<u8>().expect("failed to parse register");
        match is_addr {
            true => Ok(Token::AtReg(idx)),
            false => Ok(Token::Reg(idx)),
        }
    } else {
        match is_addr {
            false => {
                match &chunk[0..1] {
                    ":" => {
                        Ok(Token::LabelAddr(String::from(chunk.split_at(1).1))) // split to remove ':', use rest
                    },
                    _ => {
                        if chunk.starts_with("0x") {
                            match u16::from_str_radix(&chunk[2..], 16) {
                                Ok(v) => Ok(Token::Literal(v as i32)),
                                Err(e) => Err(format!("failed to parse hex: {} ({})", chunk, e)),
                            }
                        } else{
                            match chunk.parse::<i32>() {
                                Ok(v) => Ok(Token::Literal(v)),
                                Err(e) => Err(format!("failed to parse integer literal: {} ({})", chunk, e)),
                            }
                        }
                    }
                }
            },
            true => {
                let val = chunk.parse::<u16>().expect("failed to parse immediate address value");
                Ok(Token::AtLiteral(val))
            }
        }
    }

}