use crate::{CirnoError, project::*};
use std::io::{BufRead, BufReader};
use std::fs::File;
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
// token types
enum Token {
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
    while let Some(_token) = lex.next() {
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

// TODO: is there any other way to do this?
fn parseresult_default(filename: &str) -> Result<ParseResult, CirnoError> {
  match &filename[filename.len()-4..] { // extension
    ".cic" => Ok(ParseResult::Cic(Cic::default())),
    ".cip" => Ok(ParseResult::Cip(Cip::default())),
    x => Err(CirnoError::InvalidFiletype(x.to_string())),
  }
}

fn parse_attribute(token: &str, lexer: &mut logos::Lexer<'_, Token>) -> Attribute {
  lexer.next();
  match token {
    "bounds" => {
      let x: u16 = lexer.slice().parse().unwrap();
      lexer.next();
      let y: u16 = lexer.slice().parse().unwrap();
      Attribute::Bounds(Vector2 { x, y })
    },
    "label" => {
      let label: String = lexer.slice().to_string();
      Attribute::Label(label)
    }
    "num" => {
      let num: u16 = lexer.slice().parse().unwrap();
      Attribute::Num(num)
    },
    "pos" => {
      let x: u16 = lexer.slice().parse().unwrap();
      lexer.next();
      let y: u16 = lexer.slice().parse().unwrap();
      Attribute::Position(Vector2 { x, y })
    },
    "type" => {
      let t: String = lexer.slice().to_string();
      Attribute::Type(t)
    },
    "value" => Attribute::Value(parse_attribute_value(lexer)),
    "y" => {
      let y: u16 = lexer.slice().parse().unwrap();
      Attribute::YCoordinate(y)
    },
    &_ => todo!(),
  }
}

fn parse_attribute_value(lexer: &mut logos::Lexer<'_, Token>) -> Value {
  match lexer.slice() {
    "and" => {
      let mut values: Vec<String> = vec![];
      while let Some(_token) = lexer.next() {
        if lexer.slice() == "." {
          break;
        }
        values.push(lexer.slice().to_string());
      }
      Value::And(values)
    },
    "gnd" => Value::Gnd,
    "vcc" => Value::Vcc,
    &_ => todo!(),
  }
}

fn parse_object(token: &str, lexer: &mut logos::Lexer<'_, Token>) -> Result<Object, anyhow::Error> {
  // create uninitialized object
  let mut object = match token {
    "chip" => Object::Chip(Chip::default()),
    "meta" => Object::Meta(Meta::default()),
    "net" => Object::Net(Net::default()),
    "pin" => Object::Pin(Pin::default()),
    _ => todo!(), // do we need?
  };
  // get attributes
  let mut attributes: Vec<Attribute> = vec![];
  while let Some(_token) = lexer.next() {
    if lexer.slice() == ":" {
      break;
    }
    attributes.push(parse_attribute(lexer.slice(), lexer));
  }
  // apply attributes to the object
  for attribute in attributes {
    match object.apply_attribute(attribute) {
      Ok(()) => {},
      Err(e) => return Err(e.into()),
    }
  }
  // return the object
  Ok(object)
}
