use crate::calc::error::CalcResult;
use rust_decimal::Decimal;

mod ast;
mod error;
mod parser;
mod token;
mod tokenizer;

pub fn calculate(expression: &str) -> CalcResult<Decimal> {
    let mut parser = parser::Parser::new(expression)?;
    let ast = parser.parse()?;
    Ok(ast.eval())
}
