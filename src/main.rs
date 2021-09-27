#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub calculator4);
mod ast;

fn main() {
    let expr = calculator4::ExprParser::new()
    .parse("22 * 44 + 66")
    .unwrap();

    println!("{:?}", expr);
}
