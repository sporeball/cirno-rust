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

pub fn parse(filename: &str) -> Result<crate::project::ParseResult, io::Error> {
  // create default result
  let mut result = match &filename[filename.len()-4..] { // extension
    ".cic" => crate::project::ParseResult::Cic(crate::project::Cic::default()),
    ".cip" => crate::project::ParseResult::Cip(crate::project::Cip::default()),
    x => panic!("invalid filetype: {}", x),
  };
  // open file
  let file = File::open(filename)?;
  let contents = BufReader::new(file);
  let mut ast: Vec<crate::project::Object> = vec![];
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

fn parse_attribute(token: &str, lexer: &mut logos::Lexer<'_, Token>) -> crate::project::Attribute {
  lexer.next();
  match token {
    "label" => {
      let label: String = lexer.slice().to_string();
      crate::project::Attribute::Label(label)
    }
    "num" => {
      let num: i32 = lexer.slice().parse().unwrap();
      crate::project::Attribute::Num(num)
    },
    "pos" => {
      let x: i32 = lexer.slice().parse().unwrap();
      lexer.next();
      let y: i32 = lexer.slice().parse().unwrap();
      crate::project::Attribute::Position(crate::project::Position { x, y })
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
  // create uninitialized object
  let mut object = match token {
    "chip" => crate::project::Object::Chip(crate::project::Chip::default()),
    "net" => crate::project::Object::Net(crate::project::Net::default()),
    "pin" => crate::project::Object::Pin(crate::project::Pin::default()),
    _ => todo!(), // do we need?
  };
  // get attributes
  let mut attributes: Vec<crate::project::Attribute> = vec![];
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