use std::io::{BufRead, BufReader};
use std::fs::File;
use logos::{Logos, Lexer};

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
// token types
enum Token {
  #[regex("'.+")]
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

pub fn parse(contents: BufReader<File>) {
  let mut ast: Vec<crate::project::Object> = vec![];
  for line in contents.lines() {
    let line = &line.unwrap();
    let mut lex = Token::lexer(line);
    while let Some(token) = lex.next() {
      if (lex.slice().eq(":")) {
        lex.next();
      }
      ast.push(parse_object(lex.slice(), &mut lex));
    }
  }
  println!("{:?}", ast);
}

fn parse_attribute(token: &str, lexer: &mut logos::Lexer<'_, Token>) -> crate::project::Attribute {
  match token {
    "pos" => {
      lexer.next();
      let x: i32 = lexer.slice().parse().unwrap();
      lexer.next();
      let y: i32 = lexer.slice().parse().unwrap();
      crate::project::Attribute::Position(x, y)
    },
    "y" => {
      lexer.next();
      let y: i32 = lexer.slice().parse().unwrap();
      crate::project::Attribute::YCoordinate(y)
    },
    &_ => todo!(),
  }
}

fn parse_object(token: &str, lexer: &mut logos::Lexer<'_, Token>) -> crate::project::Object {
  match token {
    "chip" => new_chip(lexer),
    "net" => new_net(lexer),
    &_ => todo!(),
  }
}

fn new_chip(lexer: &mut logos::Lexer<'_, Token>) -> crate::project::Object {
  // TODO: DRY
  lexer.next();
  let t = lexer.slice();
  let mut attributes: Vec<crate::project::Attribute> = vec![];
  // while there are still tokens
  while let Some(token) = lexer.next() {
    // break if the next token is :
    if lexer.slice() == ":" {
      break;
    }
    // push an attribute to the vec
    attributes.push(parse_attribute(lexer.slice(), lexer));
  }
  println!("{:?}", attributes);
  let dummy_position = crate::project::Position(9, 9);
  crate::project::Object::Chip(t.to_string(), dummy_position)
}

fn new_net(lexer: &mut logos::Lexer<'_, Token>) -> crate::project::Object {
  lexer.next();
  let t = lexer.slice();
  let mut attributes: Vec<crate::project::Attribute> = vec![];
  // while there are still tokens
  while let Some(token) = lexer.next() {
    if lexer.slice() == ":" {
      break;
    }
    attributes.push(parse_attribute(lexer.slice(), lexer));
  }
  println!("{:?}", attributes);
  let dummy_y = crate::project::YCoordinate(9);
  crate::project::Object::Net(t.to_string(), dummy_y)
}