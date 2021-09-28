use std::fmt::{Debug, Error, Formatter};

pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Error,
}

pub enum ExprSymbol<'input> {
    NumSymbol(&'input str),
    Op(Box<ExprSymbol<'input>>, Opcode, Box<ExprSymbol<'input>>),
    Error,
}

#[derive(Copy, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

impl Expr {
    pub fn evaluate(&self) -> Option<i32> {
        use self::Opcode::*;
        use self::Expr::*;
        match *self {
            Number(n) => Some(n),
            Op(ref l, Mul, ref r) => Some(l.evaluate().unwrap() * r.evaluate().unwrap()),
            Op(ref l, Div, ref r) => Some(l.evaluate().unwrap() / r.evaluate().unwrap()),
            Op(ref l, Add, ref r) => Some(l.evaluate().unwrap() + r.evaluate().unwrap()),
            Op(ref l, Sub, ref r) => Some(l.evaluate().unwrap() - r.evaluate().unwrap()),
            Error => None
        }
    }
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Number(n) => write!(fmt, "{:?}", n),
            Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Error => write!(fmt, "error"),
        }
    }
}

impl<'input> Debug for ExprSymbol<'input> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::ExprSymbol::*;
        match *self {
            NumSymbol(n) => write!(fmt, "{:?}", n),
            Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Error => write!(fmt, "error"),
        }
    }
}

impl Debug for Opcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Opcode::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
        }
    }
}
