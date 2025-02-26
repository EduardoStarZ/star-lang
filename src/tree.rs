use crate::tokens::TokenLiteral;

pub struct Node<'a> {
    pub value : TokenLiteral,
    pub left : Option<&'a Node<'a >>,
    pub right: Option<&'a Node<'a>>
}

pub trait NextNode<'a> {
    fn has_next_node(&self) -> bool;
    fn is_valid_node(&self) -> bool;
    fn new(token : TokenLiteral) -> Node<'a>;
}

impl<'a> NextNode<'a> for Node<'a> {
    fn has_next_node(&self) -> bool {
        return match self.right {
            Some(_) => true,
            None => false
        };
    }

    fn is_valid_node(&self) -> bool {
        if self.left.is_some() && self.right.is_some() {
            return true;
        }
        return false;
    }

    fn new(token : TokenLiteral) -> Node<'a> {
        return Node{
            value: token,
            right: None,
            left: None
        }; 
    }
        
}
