use crate::project::*;
use std::io::{self, BufRead, BufReader};
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

pub fn parse(filename: &str) -> Result<ParseResult, io::Error> {
  // create default result
  let mut result = match &filename[filename.len()-4..] { // extension
    ".cic" => ParseResult::Cic(Cic::default()),
    ".cip" => ParseResult::Cip(Cip::default()),
    x => panic!("invalid filetype: {}", x),
  };
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
      ast.push(parse_object(lex.slice(), &mut lex));
    }
  }
  // apply the AST to the result
  result.apply(ast);
  // match &ast[0] {
  //   crate::project::Object::Chip(chip) => println!("{}", chip.t),
  //   _ => todo!(),
  // }
  // for attribute in &ast[0].attributes {
  //   println!("{:#?}", attribute);
  // }
  Ok(result)
}

fn parse_attribute(token: &str, lexer: &mut logos::Lexer<'_, Token>) -> Attribute {
  lexer.next();
  match token {
    "bounds" => {
      let x: i32 = lexer.slice().parse().unwrap();
      lexer.next();
      let y: i32 = lexer.slice().parse().unwrap();
      Attribute::Bounds(Vector2 { x, y })
    },
    "label" => {
      let label: String = lexer.slice().to_string();
      Attribute::Label(label)
    }
    "num" => {
      let num: i32 = lexer.slice().parse().unwrap();
      Attribute::Num(num)
    },
    "pos" => {
      let x: i32 = lexer.slice().parse().unwrap();
      lexer.next();
      let y: i32 = lexer.slice().parse().unwrap();
      Attribute::Position(Vector2 { x, y })
    },
    "type" => {
      let t: String = lexer.slice().to_string();
      Attribute::Type(t)
    },
    "value" => Attribute::Value(parse_attribute_value(lexer)),
    "y" => {
      let y: i32 = lexer.slice().parse().unwrap();
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

fn parse_object(token: &str, lexer: &mut logos::Lexer<'_, Token>) -> Object {
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
    object.apply_attribute(attribute);
  }
  // return the object
  object
}
