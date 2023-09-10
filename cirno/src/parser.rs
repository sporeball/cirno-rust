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
  // for each line
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
  println!("{:#?}", ast);
  // match &ast[0] {
  //   crate::project::Object::Chip { t, position } => println!("{}", t),
  //   _ => todo!(),
  // }
  // for attribute in &ast[0].attributes {
  //   println!("{:#?}", attribute);
  // }
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
  // create uninitialized object
  let mut object;
  match token {
    "chip" => {
      object = crate::project::Object::Chip { t: None, position: None };
    },
    "net" => {
      object = crate::project::Object::Net { t: None, y: None };
    },
    &_ => todo!(),
  }
  // get attributes
  let mut attributes: Vec<crate::project::Attribute> = vec![];
  while let Some(_token) = lexer.next() {
    if lexer.slice() == ":" {
      break;
    }
    attributes.push(parse_attribute(lexer.slice(), lexer));
  }
  // apply attributes to the object
  for attribute in &attributes {
    match attribute {
      crate::project::Attribute::Position(new_x, new_y) => {
        match object {
          crate::project::Object::Chip { t, position } => {
            object = crate::project::Object::Chip { t, position: Some(crate::project::Position { x: *new_x, y: *new_y }) };
          },
          _ => panic!("attempted to assign y-coordinate to object which does not take one"),
        }
      },
      crate::project::Attribute::Type(new_t) => {
        match object {
          crate::project::Object::Chip { t, position } => {
            object = crate::project::Object::Chip { t: Some(new_t.to_string()), position };
          },
          crate::project::Object::Net { t, y } => {
            object = crate::project::Object::Net { t: Some(new_t.to_string()), y };
          }
        }
      },
      crate::project::Attribute::YCoordinate(new_y) => {
        match object {
          crate::project::Object::Net { t, y } => {
            object = crate::project::Object::Net { t, y: Some(*new_y) }
          },
          _ => panic!("attempted to assign y-coordinate to object which does not take one"),
        }
      },
      _ => todo!(),
    }
  }
  // return the object
  object
}