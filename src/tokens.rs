#[derive(Debug, Clone, Copy)]
#[derive(PartialEq)]
pub enum TokenLiteral {
    TIntLit,
    TPlus,
    TMinus,
    TStar,
    TSlash,
    TParOpen,
    TParClose,
    TBraOpen,
    TBraClose,
    TKeyOpen,
    TKeyClose
}

pub struct Token {
    pub value : TokenLiteral,
    pub ivalue : i32
}

pub trait Scoper {
    fn eq(self, token : TokenLiteral) -> bool;
}

impl Scoper for TokenLiteral {
    fn eq(self, token : TokenLiteral) -> bool {
        if self == TokenLiteral::TParOpen && token == TokenLiteral::TParClose {
            return true;
        }

        if self == TokenLiteral::TBraOpen && token == TokenLiteral::TBraClose {
            return true;
        }

        if self == TokenLiteral::TKeyOpen && token == TokenLiteral::TKeyClose {
            return true;
        }

        return false;
    }
}
