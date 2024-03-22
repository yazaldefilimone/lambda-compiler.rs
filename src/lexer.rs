use std::str::Chars;

pub enum Token {
  Term(usize, String),
  Lambda(usize),
  RightParen(usize),
  LeftParen(usize),
  Dot(usize),
}

pub fn lexer(input: &mut Chars) -> Vec<Token> {
  let mut tokens = Vec::new();
  let mut cursor: usize = 0;
  let mut current_term = String::new();

  while let Some(current_character) = input.next() {
    cursor += 1;
    let next_token = match current_character {
      'λ' | '\\' => Token::Lambda(cursor),
      '(' => Token::LeftParen(cursor),
      ')' => Token::RightParen(cursor),
      '.' => Token::Dot(cursor),
      ' ' => {
        if !current_term.is_empty() {
          continue;
        }
        Token::Term(cursor, current_term.clone())
      }
      _ => {
        current_term.push(current_character);
        continue;
      }
    };

    if !current_term.is_empty() {
      tokens.push(Token::Term(cursor, current_term.clone()));
      current_term.clear();
    }

    tokens.push(next_token);
  }

  tokens
}
