use crate::tokens::TokenLiteral;
use crate::tokens::TokenLiteral::*;

pub struct Node<'a> {
    pub index : i128,
    pub left : Option<&'a Node<'a >>,
    pub right: Option<&'a Node<'a>>
}

