use nom::IResult;

use ast::*;
use parser::parse;

#[test]
fn test_simple_parse() {
    let expected = Expr::Call(Op::Add,
        vec![
            Expr::Lit(Lit::Int(Int(1))),
            Expr::Lit(Lit::Int(Int(2))),
        ]);
    let mut expo = "(+ 1 2)\r\n".to_string();
    let result = parse(&mut expo);
    if let IResult::Done(_, output) = result {
        assert_eq!(output, expected);
    }
    else {
        println!("{:?}", result);
        assert!(false);
    }
}

#[test]
fn test_parse() {
    let expected = Expr::Call(Op::Add,
        vec![
            Expr::Lit(Lit::Int(Int(1))),
            Expr::Call(Op::Mul,
                vec![
                    Expr::Lit(Lit::Int(Int(2))),
                    Expr::Lit(Lit::Int(Int(3))),
                ]
            ),
            Expr::Call(Op::Sub,
                vec![
                    Expr::Lit(Lit::Int(Int(4))),
                    Expr::Call(Op::Div,
                        vec![
                            Expr::Lit(Lit::Int(Int(5))),
                            Expr::Lit(Lit::Int(Int(6))),
                        ]
                    ),
                ]
            ),
        ]);
    let mut expo = "(+ 1 (* 2 3) (- 4 (/ 5 6)))\r\n".to_string();
    let result = parse(&mut expo);
    if let IResult::Done(_, output) = result {
        assert_eq!(output, expected);
    }
    else {
        println!("{:?}", result);
        assert!(false);
    }
}
