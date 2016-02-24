use nom::IResult;

use ast::*;
use parser::parse;

#[test]
fn test_simple_parse() {
    let expected = Expression::Call(Operator::Plus,
        vec![
            Expression::Literal(Literal::Integer(Int(1))),
            Expression::Literal(Literal::Integer(Int(2))),
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
    let expected = Expression::Call(Operator::Plus,
        vec![
            Expression::Literal(Literal::Integer(Int(1))),
            Expression::Call(Operator::Times,
                vec![
                    Expression::Literal(Literal::Integer(Int(2))),
                    Expression::Literal(Literal::Integer(Int(3))),
                ]
            ),
            Expression::Call(Operator::Minus,
                vec![
                    Expression::Literal(Literal::Integer(Int(4))),
                    Expression::Call(Operator::Divide,
                        vec![
                            Expression::Literal(Literal::Integer(Int(5))),
                            Expression::Literal(Literal::Integer(Int(6))),
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
