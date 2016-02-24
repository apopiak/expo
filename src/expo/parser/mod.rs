#[cfg(test)] pub mod test;

pub use nom::IResult;

use utils::vec_to_i64;
use ast::*;

named!(sign <&[u8], i64>, map!(tag!("-"), |_| -1 ));

named!(integer  <&[u8], Lit>, map!(
    many1!(one_of!(b"0123456789")),
    | vector |  Lit::Int(Int(vec_to_i64(vector)))
));

named!(operator <&[u8], Op>, alt!(
    map!(tag!("+"), |_| Op::Add) |
    map!(tag!("-"), |_| Op::Sub) |
    map!(tag!("*"), |_| Op::Mul) |
    map!(tag!("/"), |_| Op::Div)
));

named!(number <&[u8], Lit>, chain!(
        pref: opt!(sign) ~
        int:  integer,
        || {
            match int.eval() {
                Result::Ok(num) => Lit::Int((Int(pref.unwrap_or(1)) * num).unwrap()),
                Result::Err(_) => panic!(),
            }
        }
));

named!(arguments <&[u8], Vec<Expr> >, many1!(
    chain!(
        tag!(" ") ~
        exp: expression,
        || { exp }
    )
));

named!(open_brace <&[u8], char>, char!('('));
named!(close_brace <&[u8], char>, char!(')'));

named!(expression <&[u8], Expr>, alt!(
    chain!(
        num: alt!(number | delimited!(open_brace, number, close_brace)),
        || { Expr::Lit(num) }
    ) |
    delimited!(
        open_brace,
        chain!(
            op: operator ~
            args: arguments,
            || { Expr::Call(op, args) }),
        close_brace
    )
));

named!(line_ending, alt!(tag!("\r") | tag!("\r\n")));

named!(pub expo <&[u8], Expr>,
    chain!(
        expo: expression ~
        endl: line_ending,
        || { expo }
    )
);

pub fn parse(s: &mut String) -> IResult<&[u8], Expr> {
    expo(s.as_bytes())
}
