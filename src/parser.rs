// use TSPL::new_parser;
pub use TSPL::Parser;
// typed lambda calculus

pub enum Type {
  Function { argument: Box<Type>, result: Box<Type> },
  Variable { name: String },
}
pub enum Term {
  Variable { name: String, _type: Option<Type> },
  Application { function: Box<Term>, argument: Box<Term> },
  Abstraction { variable: Box<Term>, body: Box<Term> },
}

TSPL::new_parser!(TermParser);

impl<'a> TermParser<'a> {
  // make sure that new fun is a public function in TSPL, but don't duplicate that here

  pub fn parse(&mut self) -> Result<Term, String> {
    self.skip_trivia();
    match self.peek_one() {
      Some('(') => self.parse_application(),
      Some('λ') => self.parse_abstraction(),
      _ => self.parse_variable(),
    }
  }

  fn parse_application(&mut self) -> Result<Term, String> {
    self.consume("(")?;
    let function = self.parse()?;
    self.skip_trivia();
    let argument = self.parse()?;
    self.consume(")")?;
    Ok(Term::Application {
      function: Box::new(function),
      argument: Box::new(argument),
    })
  }

  fn parse_abstraction(&mut self) -> Result<Term, String> {
    self.consume("λ")?;
    let variable = self.parse_variable()?;
    self.consume(".")?;
    let body = self.parse()?;

    Ok(Term::Abstraction {
      variable: Box::new(variable),
      body: Box::new(body),
    })
  }
  fn parse_variable(&mut self) -> Result<Term, String> {
    let name = self.parse_name()?;
    let _type = self.parse_type()?;
    Ok(Term::Variable { name, _type })
  }
  fn parse_type(&mut self) -> Result<Option<Type>, String> {
    self.skip_trivia();
    if !self.starts_with(":") {
      return Ok(None);
    }
    println!("name: {:?}", self.starts_with(":"));
    self.consume(":")?;
    self.skip_trivia();
    if self.peek_one() == Some('(') {
      return Ok(self.parse_function_type()?);
    }
    let mut name = self.parse_name()?;
    Ok(Some(Type::Variable { name }))
  }
  fn parse_function_type(&mut self) -> Result<Option<Type>, String> {
    self.skip_trivia();
    self.consume("(")?;
    let argument = self.parse_type()?.expect("Expected type");
    self.consume("->")?;
    let result = self.parse_type()?.expect("Expected type");
    self.skip_trivia();
    self.consume(")")?;
    Ok(Some(Type::Function {
      argument: Box::new(argument),
      result: Box::new(result),
    }))
  }
}

fn main() {
  let mut parser = TermParser::new("λx. x");
  let term = parser.parse().unwrap();
  println!("{:?}", term);
}
