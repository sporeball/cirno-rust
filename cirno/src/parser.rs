use std::io::{self, BufRead, BufReader};
use std::fs::File;
use logos::{Logos};

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

pub fn parse_cip(filename: &str) -> Result<Vec<crate::project::Object>, io::Error> {
  let file = File::open(filename)?;
  let contents = BufReader::new(file);
  let mut ast: Vec<crate::project::Object> = vec![];
  for line in contents.lines() {
    let line = &line.unwrap();
    let mut lex = Token::lexer(line);
    while let Some(_token) = lex.next() {
      if lex.slice().eq(":") {
        lex.next();
      }
      ast.push(parse_object(lex.slice(), &mut lex));
    }
  }
  println!("{:#?}", ast);
  Ok(ast)
}

fn parse_attribute(token: &str, lexer: &mut logos::Lexer<'_, Token>) -> crate::project::Attribute {
  lexer.next();
  match token {
    "pos" => {
      let x: i32 = lexer.slice().parse().unwrap();
      lexer.next();
      let y: i32 = lexer.slice().parse().unwrap();
      crate::project::Attribute::Position(x, y)
    },
    "type" => {
      let t: String = lexer.slice().to_string();
      crate::project::Attribute::Type(t)
    },
    "value" => crate::project::Attribute::Value(parse_attribute_value(lexer)),
    "y" => {
      let y: i32 = lexer.slice().parse().unwrap();
      crate::project::Attribute::YCoordinate(y)
    },
    &_ => todo!(),
  }
}

fn parse_attribute_value(lexer: &mut logos::Lexer<'_, Token>) -> crate::project::Value {
  match lexer.slice() {
    "and" => {
      let mut values: Vec<String> = vec![];
      while let Some(_token) = lexer.next() {
        if lexer.slice() == "." {
          break;
        }
        values.push(lexer.slice().to_string());
      }
      crate::project::Value::And(values)
    },
    "gnd" => crate::project::Value::Gnd,
    "vcc" => crate::project::Value::Vcc,
    &_ => todo!(),
  }
}

fn parse_object(token: &str, lexer: &mut logos::Lexer<'_, Token>) -> crate::project::Object {
  let mut attributes: Vec<crate::project::Attribute> = vec![];
  while let Some(_token) = lexer.next() {
    if lexer.slice() == ":" {
      break;
    }
    attributes.push(parse_attribute(lexer.slice(), lexer));
  }
  match token {
    "chip" => crate::project::Object::Chip(attributes),
    "net" => crate::project::Object::Net(attributes),
    &_ => todo!(),
  }
}