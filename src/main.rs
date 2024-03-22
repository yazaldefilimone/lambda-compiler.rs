mod lexer;

use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;

enum Expression {
  Variable(String),
  Abstraction(String, Box<Expression>),
  Application(Box<Expression>, Box<Expression>),
}

impl Display for Expression {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    match self {
      Expression::Variable(name) => write!(f, "{}", name),
      Expression::Abstraction(param, body) => write!(f, "λ{}.{}", param, body),
      // lhs = left hand side, rhs = right hand side
      Expression::Application(lhs, rhs) => write!(f, "({} {})", lhs, rhs),
    }
  }
}

fn main() {
  let id = Expression::Abstraction("x".to_string(), Box::new(Expression::Variable("x".to_string())));
  println!("{}", id);
}
