use std::char;

use crate::tokens::{Token, TokenLiteral};

pub struct Scanner {
    pub file: String,
    pub char_idx: usize,
    pub curr_char: char,
    file_size: usize
}

pub trait Scanning {
    fn next(&mut self) -> Option<char>;
    fn previous(&mut self) -> Option<char>;
    fn skip(&mut self) -> Option<char>;
    fn scan(&mut self, token_struct : &mut Token) -> bool;
}

impl Scanning for Scanner {
    fn next(&mut self) -> Option<char> {
        if self.file_size > self.char_idx {
            let c : char = self.file.chars().collect::<Vec<char>>()[self.char_idx+1];

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
            _ => {
                if c.is_numeric() {
                    token_struct.value = TokenLiteral::TIntLit;
                    token_struct.ivalue = scan
                }
            },
        }

        return true;
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
