use nom::IResult;

use utils::vec_to_i64;
use ast::*;

named!(sign <&[u8], i64>, map!(tag!("-"), |_| -1 ));

named!(integer  <&[u8], Literal>, map!(
    many1!(one_of!(b"0123456789")),
    | vector |  Literal::Integer(Int(vec_to_i64(vector)))
));

named!(operator <&[u8], Operator>, alt!(
    map!(tag!("+"), |_| Operator::Plus) |
    map!(tag!("-"), |_| Operator::Minus) |
    map!(tag!("*"), |_| Operator::Times) |
    map!(tag!("/"), |_| Operator::Divide)
));

named!(number <&[u8], Literal>, chain!(
        pref: opt!(sign) ~
        int:  integer,
        || {
            match int.eval() {
                Result::Ok(num) => Literal::Integer((Int(pref.unwrap_or(1)) * num).unwrap()),
                Result::Err(_) => panic!(),
            }
        }
));

named!(arguments <&[u8], Vec<Expression> >, many1!(
    chain!(
        tag!(" ") ~
        exp: expression,
        || { exp }
    )
));

named!(open_brace <&[u8], char>, char!('('));
named!(close_brace <&[u8], char>, char!(')'));

named!(expression <&[u8], Expression>, alt!(
    chain!(
        num: alt!(number | delimited!(open_brace, number, close_brace)),
        || { Expression::Literal(num) }
    ) |
    delimited!(
        open_brace,
        chain!(
            op: operator ~
            args: arguments,
            || { Expression::Call(op, args) }),
        close_brace
    )
));

named!(line_ending, alt!(tag!("\r") | tag!("\r\n")));

named!(pub expo <&[u8], Expression>,
    chain!(
        expo: expression ~
        endl: line_ending,
        || { expo }
    )
);

pub fn parse(s: &mut String) {
    let expr = expo(s.as_bytes());
    if let IResult::Done(_, output) = expr {
        println!("{:?}", output.eval());
    }
    else {
        println!("error while parsing: {:?}", expr);
    }
}
