use std::fmt;
// use TSPL::new_parser;
pub use TSPL::Parser;
// typed lambda calculus

#[derive(Clone)]
pub enum Type {
  Function { argument: Box<Type>, result: Box<Type> },
  Variable { name: String },
}
#[derive(Clone)]
pub enum Term {
  Variable { name: String, _type: Option<Type> },
  Application { function: Box<Term>, argument: Box<Term> },
  Abstraction { variable: Box<Term>, body: Box<Term> },
}

TSPL::new_parser!(TermParser);

impl<'a> TermParser<'a> {
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
    let argument = self.parse()?;
    self.skip_trivia();
    self.consume(")")?;
    Ok(Term::Application { function: Box::new(function), argument: Box::new(argument) })
  }

  fn parse_abstraction(&mut self) -> Result<Term, String> {
    self.consume("λ")?;
    let variable = self.parse_variable()?;
    self.consume(".")?;
    let body = self.parse()?;

    Ok(Term::Abstraction { variable: Box::new(variable), body: Box::new(body) })
  }
  fn parse_variable(&mut self) -> Result<Term, String> {
    let name = self.parse_name()?;
    self.skip_trivia();
    if self.starts_with(":") {
      self.consume(":")?;
      self.skip_trivia();
      let _type = Some(self.parse_type()?);
      return Ok(Term::Variable { name, _type });
    }

    Ok(Term::Variable { name, _type: None })
  }

  fn parse_type(&mut self) -> Result<Type, String> {
    self.skip_trivia();
    if self.starts_with("(") {
      return self.parse_function_type();
    }
    self.skip_trivia();
    self.parse_base_type()
  }

  fn parse_function_type(&mut self) -> Result<Type, String> {
    self.consume("(")?;
    let mut argument = Box::new(self.parse_type()?);
    self.skip_trivia();
    while self.starts_with("-") {
      self.consume("->")?;
      let result = Box::new(self.parse_type()?);
      argument = Box::new(Type::Function { argument, result });
    }
    self.skip_trivia();
    self.consume(")")?;
    Ok(*argument)
  }
  fn parse_base_type(&mut self) -> Result<Type, String> {
    let name = self.parse_name()?;
    Ok(Type::Variable { name })
  }

  fn parse_name(&mut self) -> Result<String, String> {
    Ok(self.take_while(|c| c.is_ascii_alphanumeric()).to_string())
  }
}

// ----------------------------------------
impl fmt::Debug for Type {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &self {
      Type::Function { argument, result } => {
        write!(f, "({:?} -> {:?})", argument, result)
      }
      Type::Variable { name } => {
        write!(f, "{}", name)
      }
    }
  }
}
impl fmt::Debug for Term {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &self {
      Term::Variable { name, _type } => {
        if let Some(_type) = _type {
          write!(f, "{}: {:?}", name, _type)
        } else {
          write!(f, "{}", name)
        }
      }
      Term::Application { function, argument } => {
        write!(f, "({:?} {:?})", function, argument)
      }
      Term::Abstraction { variable, body } => {
        write!(f, "λ{:?}. {:?}", variable, body)
      }
    }
  }
}

// --- normalize ---
fn normalize_abstraction(abs: Term) -> Term {
  match abs {
    Term::Abstraction { variable, body } => {
      let new_body = normalize(*body);
      Term::Abstraction { variable, body: Box::new(new_body) }
    }
    _ => abs,
  }
}
fn normalize_application(app: Term) -> Term {
  match app {
    Term::Application { function, argument } => {
      let new_function = normalize(*function);
      let new_argument = normalize(*argument);
      if let Term::Abstraction { variable: _, body } = new_function {
        return apply_abstraction(&*body, &new_argument);
      }
      Term::Application { function: Box::new(new_function), argument: Box::new(new_argument) }
    }
    _ => app,
  }
}

fn apply_abstraction(abs: &Term, arg: &Term) -> Term {
  if let Term::Abstraction { variable, body } = abs {
    return substitute(&*body, &*variable, &arg);
  }
  return abs.clone();
}

fn get_variable_name<'a>(term: &'a Term) -> &'a str {
  match term {
    Term::Variable { name, _type: _ } => name,
    _ => "",
  }
}
fn substitute(term: &Term, variable: &Term, replacement: &Term) -> Term {
  match term {
    // -- variable --
    Term::Variable { name, _type } => {
      if name == get_variable_name(replacement) {
        return replacement.clone();
      }
      Term::Variable { name: name.clone(), _type: _type.clone() }
    }
    // -- application or function call --
    Term::Application { function, argument } => {
      let new_function = substitute(&*function, variable, replacement);
      let new_argument = substitute(&*argument, variable, replacement);
      Term::Application { function: Box::new(new_function), argument: Box::new(new_argument) }
    }
    // -- abstraction or lambda --
    Term::Abstraction { variable: abs_variable, body } => {
      if get_variable_name(&*abs_variable) == get_variable_name(variable) {
        return Term::Abstraction { variable: abs_variable.clone(), body: body.clone() };
      }
      let new_body = substitute(&*body, variable, replacement);
      Term::Abstraction { variable: abs_variable.clone(), body: Box::new(new_body) }
    }
  }
}

fn normalize(term: Term) -> Term {
  match term {
    Term::Variable { name, _type } => Term::Variable { name, _type },
    Term::Application { function, argument } => {
      return normalize_application(Term::Application { function, argument });
    }
    Term::Abstraction { variable, body } => {
      return normalize_abstraction(Term::Abstraction { variable, body });
    }
  }
}

fn main() {
  let lam = "(λf. λx. (f (f x)) λg. λy. (g (g y)))";
  let _lam_type = "(λf: (int -> int). λx: int. (f (f x)))";
  let mut parser = TermParser::new(lam);

  match parser.parse() {
    Ok(term) => {
      println!("P: {:?}", term);
      // I need to implement a type checker? huhhhhhh :(, maybe no!
      let norm = normalize(term);
      println!("N: {:?}", norm);
    }
    Err(error) => {
      println!("Error: {}", error);
    }
  };
}
