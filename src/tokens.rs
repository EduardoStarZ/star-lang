pub enum TokenLiteral {
    TIntLit,
    TPlus,
    TMinus,
    TStar,
    TSlash
}

pub struct Token {
    pub value : TokenLiteral,
    pub ivalue : i32
}
