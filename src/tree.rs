use crate::tokens::TokenLiteral;

pub enum Operator {
    Add,
    Subtract,
    Divide,
    Multiply,
    Module
}

pub enum Node {
     Int(isize),
     UnaryExpr {
        value : isize,
        operator : Operator
     },
     BinaryExpr {
        operator: Operator,
        left_node : Box<Node>,
        right_node : Box<Node>
     }
}


