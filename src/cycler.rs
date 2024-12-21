use crate::tokens::{Scoper, TokenLiteral::{self, *}};

struct Vigilant {
    pub openers : Vec<TokenLiteral>
}

pub fn check(content : Vec<TokenLiteral>) -> bool {
    let mut watcher  : Vigilant = Vigilant {openers: Vec::new()};

    for token in content {
        match token {
            TParOpen | TBraOpen | TKeyOpen => watcher.openers.push(token),
            TParClose | TBraClose | TKeyClose => {
                if !watcher.openers.pop().unwrap().eq(token) {
                    return false;
                }
            },
            _ => (),
        }
    }

    return true; 
}
