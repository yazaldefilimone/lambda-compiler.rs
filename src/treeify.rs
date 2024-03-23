use crate::{lexer::Token, Expression};

enum TreeifyError {
  UnclosedParen(usize),
  UnopenedParen(usize),
  MissingLambdaVar(usize),
  MissingLambdaBody(usize),
  EmptyExprList,
}

pub fn treeify(tokens: &[Token]) -> Result<Expression, TreeifyError> {
  for token in tokens.iter() {
    match &token {
      Token::Lambda(_) => {}
    }
  }

  match tokens {
    [] => Err(TreeifyError::EmptyExprList),
    _ => Ok(Expression::Variable("".to_string())),
  }
}
