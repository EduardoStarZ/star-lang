use std::char;

use crate::tokens::{Token, TokenLiteral};

pub struct Scanner {
    pub file: String,
    pub char_idx: usize,
    pub curr_char: char,
    pub file_size: usize
}

pub trait Scanning {
    fn next(&mut self) -> Option<char>;
    fn previous(&mut self) -> Option<char>;
    fn skip(&mut self) -> Option<char>;
    fn scan(&mut self, token_struct : &mut Token) -> bool;
    fn scan_file(&mut self) -> Vec<TokenLiteral>;
}

impl Scanning for Scanner {
    fn next(&mut self) -> Option<char> {
        if self.file_size > self.char_idx {
            let c : char = self.file.chars().collect::<Vec<char>>()[self.char_idx];

            self.char_idx += 1;
            self.curr_char = c;

            return Some(c);
        }

        return None; 
    }

    fn previous(&mut self) -> Option<char> {
        if 0 < self.char_idx {
            let c : char = self.file.chars().collect::<Vec<char>>()[self.char_idx-1];

            self.char_idx -= 1;
            self.curr_char = c;

            return Some(c);
        }

        return None;
    }

    fn skip(&mut self) -> Option<char>  {
        let mut c : char = match self.next() {
            Some(value) => value,
            None => return None
        };

        while ' ' == c || '\t' == c || '\n' == c || '\r' == c {
            c = match self.next() {
                Some(value) => value,
                None => return None
            };
        }

        return Some(c);
    }

    fn scan(&mut self, token_struct : &mut Token) -> bool {
        let c : char = match self.skip() {
            Some(value) => value,
            None => return false
        };

        match c {
            '+' => token_struct.value = TokenLiteral::TPlus,
            '-' => token_struct.value = TokenLiteral::TMinus,
            '*' => token_struct.value = TokenLiteral::TStar,
            '/' => token_struct.value = TokenLiteral::TSlash,
            '(' => token_struct.value = TokenLiteral::TParOpen,
            ')' => token_struct.value = TokenLiteral::TParClose,
            '[' => token_struct.value = TokenLiteral::TBraOpen,
            ']' => token_struct.value = TokenLiteral::TBraClose,
            '{' => token_struct.value = TokenLiteral::TKeyOpen,
            '}' => token_struct.value = TokenLiteral::TKeyClose,
            ';' => token_struct.value = TokenLiteral::TEndOfExpression,
            _ => {
                if c.is_numeric() {
                    token_struct.value = TokenLiteral::TIntLit;
                    token_struct.ivalue = scanint(self);
                }
            },
        }

        return true;
    }

fn scan_file(&mut self) -> Vec<TokenLiteral> {
    let mut token : Token = Token { value: TokenLiteral::TIntLit, ivalue: 0};
    let mut tokens : Vec<TokenLiteral> = Vec::new();

    while self.scan(&mut token) {
        print!("Token: {:?}", &token.value);

        if token.value == TokenLiteral::TIntLit {
            print!("  ||  Integer value of: {}", token.ivalue);
        }

        tokens.push(token.value);
        
        print!("\n");
    }
    return tokens;
}
}


fn scanint(scn : &mut Scanner) -> i32 {
    let mut val : i32 = 0;

    let mut k : i32 = chrpos("0123456789", scn.curr_char);

    while k >= 0 {
            val = val * 10 + k;
            scn.next();
            k = chrpos("0123456789", scn.curr_char);
    }

    scn.previous();
    return val;
}

fn chrpos(s: &str, c: char) -> i32 {
    match s.find(c) {
        Some(value) => value as i32,
        None => -1
    }
}
