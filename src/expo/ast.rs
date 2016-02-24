use std::fmt;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;
use std::ops::Div;

#[derive(PartialEq)]
pub struct Int(pub i64);

impl fmt::Debug for Int {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    Unknown,
    DivByZero,
}

pub trait Eval {
    fn eval(&self) -> Result<Int, Error>;
}

pub trait Combine<T> {
    fn combine<F>(self: Self, fun: F, m: Self) -> Self
        where F: Fn(T, T) -> Self;
}

impl<T> Combine<T> for Result<T, Error>
    where T: Sized {
    fn combine<F>(self: Self, fun: F, m: Self) -> Self
        where F: Fn(T, T) -> Self {

        match self {
            Result::Ok(t1) => {
                match m {
                    Result::Ok(t2) => {
                        match fun(t1, t2) {
                            Result::Ok(res) => Result::Ok(res),
                            Result::Err(error) => Result::Err(error),
                        }
                    },
                    Result::Err(error) => Result::Err(error),
                }
            },
            Result::Err(error) => Result::Err(error),
        }
    }
}

impl Add for Int {
    type Output = Result<Int, Error>;

    fn add(self, rhs: Int) -> Result<Int, Error> {
        Result::Ok(Int(self.0 + rhs.0))
    }
}

impl Mul for Int {
    type Output = Result<Int, Error>;

    fn mul(self, rhs: Int) -> Result<Int, Error> {
        Result::Ok(Int(self.0 * rhs.0))
    }
}

impl Sub for Int {
    type Output = Result<Int, Error>;

    fn sub(self, rhs: Int) -> Result<Int, Error> {
        Result::Ok(Int(self.0 - rhs.0))
    }
}

impl Div for Int {
    type Output = Result<Int, Error>;

    fn div(self, rhs: Int) -> Result<Int, Error> {
        let Int(int) = rhs;
        if int == 0 {
            Result::Err(Error::DivByZero)
        }
        else {
            Result::Ok(Int(self.0 / rhs.0))
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus,
    Times,
    Minus,
    Divide,
}

impl Operator {
    fn eval_op(&self, args: &Vec<Expression>) -> Result<Int, Error> {
        match *self {
            Operator::Plus => args.iter().fold(Result::Ok(Int(0)),
                |acc, expr: &Expression| {
                    acc.combine(Add::add, expr.eval())
            }),
            Operator::Times => args.iter().fold(Result::Ok(Int(1)),
                |acc, expr: &Expression| {
                    acc.combine(Mul::mul, expr.eval())
            }),
            Operator::Minus => {
                let mut iter = args.iter();
                let first = iter.next().expect("first argument has to exist");
                iter.fold(first.eval(),
                    |acc, expr: &Expression| {
                        acc.combine(Sub::sub, expr.eval())
                    }
                )
            },
            Operator::Divide => {
                let mut iter = args.iter();
                let first = iter.next().expect("first argument has to exist");
                iter.fold(first.eval(),
                    |acc, expr: &Expression| {
                        acc.combine(Div::div, expr.eval())
                    }
                )
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Integer(Int)
}

impl Eval for Literal {
    fn eval(&self) -> Result<Int, Error> {
        match *self {
            Literal::Integer(Int(int)) => Result::Ok(Int(int)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Call(Operator, Vec<Expression>),
}

impl Eval for Expression {
    fn eval(&self) -> Result<Int, Error> {
        match *self {
            Expression::Literal(ref lit) => lit.eval(),
            Expression::Call(ref op, ref args) => op.eval_op(args),
        }
    }
}
