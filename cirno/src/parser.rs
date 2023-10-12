use crate::{CirnoError, project::*};
use std::io::{BufRead, BufReader};
use std::fs::File;
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
// token types
pub enum Token {
  #[regex("'[a-z0-9]+")]
  Identifier,
  #[regex("[a-z][a-z0-9/]*")]
  Keyword,
  #[regex("[0-9]*")]
  Number,
  #[token(".")]
  Ender,
  #[token(":")]
  Separator
}

impl std::fmt::Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
    write!(f, "{:?}", self)
  }
}

pub fn parse(filename: &str) -> Result<ParseResult, anyhow::Error> {
  // create default result
  let mut result = parseresult_default(filename)?;
  // open file
  let file = File::open(filename)?;
  let contents = BufReader::new(file);
  let mut ast: Vec<Object> = vec![];
  // for each line in the file
  for line in contents.lines() {
    // tokenize the line
    let line = &line.unwrap();
    let mut lex = Token::lexer(line);
    // while there are still tokens
    while let Some(token) = lex.next() {
      if let Err(_) = token {
        return Err(CirnoError::UnrecognizedToken.into())
      }
      // skip colons
      if lex.slice().eq(":") {
        lex.next();
      }
      // parse an object into the AST
      match parse_object(lex.slice(), &mut lex) {
        Ok(object) => ast.push(object),
        Err(e) => return Err(e.into()),
      }
    }
  }
  // apply the AST to the result
  result.apply(ast);
  Ok(result)
}

fn parseresult_default(filename: &str) -> Result<ParseResult, CirnoError> {
  match &filename[filename.len()-4..] { // extension
    ".cic" => Ok(ParseResult::Cic(Cic::default())),
    ".cip" => Ok(ParseResult::Cip(Cip::default())),
    x => Err(CirnoError::InvalidFiletype(x.to_string())),
  }
}

// TODO: since parse() creates a new Token::lexer for each line,
// the phrasing of the None arms in the below macros is a bit weird

/// expect_number!(lexer)
macro_rules! expect_number {
  ($x:expr) => {{
    match $x.next() {
      Some(Ok(Token::Number)) => Ok($x.slice().parse().unwrap()),
      Some(Ok(u)) => Err(CirnoError::UnexpectedTokenExpectedNumber(u)),
      Some(Err(_e)) => Err(CirnoError::UnrecognizedToken),
      None => Err(CirnoError::OutOfTokensExpectedNumber),
    }
  }}
}

/// expect_token!(lexer, Token::[...])
macro_rules! expect_token {
  ($x:expr, $p:path) => {{
    match $x.next() {
      Some(Ok($p)) => Ok($x.slice().to_string()),
      Some(Ok(u)) => Err(CirnoError::UnexpectedToken($p, u)),
      Some(Err(_e)) => Err(CirnoError::UnrecognizedToken),
      None => Err(CirnoError::OutOfTokens($p)),
    }
  }}
}

fn parse_attribute(token: &str, lexer: &mut logos::Lexer<'_, Token>) -> Result<Attribute, CirnoError> {
  match token {
    "bounds" => {
      let x: u16 = expect_number!(lexer)?;
      let y: u16 = expect_number!(lexer)?;
      Ok(Attribute::Bounds(Vector2 { x, y }))
    },
    "label" => {
      let label = expect_token!(lexer, Token::Identifier)?;
      Ok(Attribute::Label(label))
    }
    "num" => {
      let num: u16 = expect_number!(lexer)?;
      Ok(Attribute::Num(num))
    },
    "pos" => {
      let x: u16 = expect_number!(lexer)?;
      let y: u16 = expect_number!(lexer)?;
      Ok(Attribute::Position(Vector2 { x, y }))
    },
    "type" => {
      let t = expect_token!(lexer, Token::Keyword)?;
      Ok(Attribute::Type(t))
    },
    "value" => {
      let v = parse_attribute_value(lexer)?;
      Ok(Attribute::Value(v))
    },
    "y" => {
      let y: u16 = expect_number!(lexer)?;
      Ok(Attribute::YCoordinate(y))
    },
    a => Err(CirnoError::InvalidAttribute(a.to_string())),
  }
}

fn parse_attribute_value(lexer: &mut logos::Lexer<'_, Token>) -> Result<Value, CirnoError> {
  lexer.next();
  match lexer.slice() {
    "and" => {
      let mut values: Vec<String> = vec![];
      while let Some(_token) = lexer.next() {
        if lexer.slice() == "." {
          break;
        }
        values.push(lexer.slice().to_string());
      }
      Ok(Value::And(values))
    },
    "gnd" => Ok(Value::Gnd),
    "vcc" => Ok(Value::Vcc),
    v => Err(CirnoError::InvalidValueAttribute(v.to_string())),
  }
}

fn object_default(token: &str) -> Result<Object, CirnoError> {
  match token {
    "chip" => Ok(Object::Chip(Chip::default())),
    "meta" => Ok(Object::Meta(Meta::default())),
    "net" => Ok(Object::Net(Net::default())),
    "pin" => Ok(Object::Pin(Pin::default())),
    t => Err(CirnoError::InvalidObjectType(t.to_string())),
  }
}

fn parse_object(token: &str, lexer: &mut logos::Lexer<'_, Token>) -> Result<Object, anyhow::Error> {
  // create uninitialized object
  let mut object = object_default(token)?;
  // get attributes
  let mut attributes: Vec<Attribute> = vec![];
  while let Some(_token) = lexer.next() {
    if lexer.slice() == ":" {
      break;
    }
    let attribute = parse_attribute(lexer.slice(), lexer)?;
    attributes.push(attribute);
  }
  // apply attributes to the object
  for attribute in attributes {
    object.apply_attribute(attribute)?;
  }
  // return the object
  Ok(object)
}
