//Parser generated by rustlr

#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
extern crate rustlr;
use rustlr::{RuntimeParser,RProduction,Stateaction};

pub fn make_parser() -> RuntimeParser<i32,i32>
{
 let mut parser1:RuntimeParser<i32,i32> = RuntimeParser::new(7,12);
 let mut rule = RProduction::<i32,i32>::new_skeleton("start");
 rule = RProduction::<i32,i32>::new_skeleton("E");
 rule.Ruleaction = |parser|{  let t:i32=parser.stack.pop().unwrap().value;  parser.stack.pop();   let e:i32=parser.stack.pop().unwrap().value;   return e+t; };
 parser1.Rules.push(rule);
 rule = RProduction::<i32,i32>::new_skeleton("E");
 rule.Ruleaction = |parser|{  let t:i32=parser.stack.pop().unwrap().value;   return t; };
 parser1.Rules.push(rule);
 rule = RProduction::<i32,i32>::new_skeleton("T");
 rule.Ruleaction = |parser|{  let f:i32=parser.stack.pop().unwrap().value;  parser.stack.pop();   let t:i32=parser.stack.pop().unwrap().value;   return t*f; };
 parser1.Rules.push(rule);
 rule = RProduction::<i32,i32>::new_skeleton("T");
 rule.Ruleaction = |parser|{  let f:i32=parser.stack.pop().unwrap().value;   return f; };
 parser1.Rules.push(rule);
 rule = RProduction::<i32,i32>::new_skeleton("F");
 rule.Ruleaction = |parser|{ parser.stack.pop();   let e:i32=parser.stack.pop().unwrap().value;  parser.stack.pop();   return e; };
 parser1.Rules.push(rule);
 rule = RProduction::<i32,i32>::new_skeleton("F");
 rule.Ruleaction = |parser|{  let n:i32=parser.stack.pop().unwrap().value;   return n; };
 parser1.Rules.push(rule);
 rule = RProduction::<i32,i32>::new_skeleton("START");
 rule.Ruleaction = |parser|{ parser.stack.pop();   return i32::default();};
 parser1.Rules.push(rule);
 parser1.Errsym = "";
 parser1.RSM[0].insert("num",Stateaction::Shift(5));
 parser1.RSM[0].insert("T",Stateaction::Gotonext(1));
 parser1.RSM[0].insert("E",Stateaction::Gotonext(3));
 parser1.RSM[0].insert("F",Stateaction::Gotonext(4));
 parser1.RSM[0].insert("(",Stateaction::Shift(2));
 parser1.RSM[1].insert(")",Stateaction::Reduce(1));
 parser1.RSM[1].insert("EOF",Stateaction::Reduce(1));
 parser1.RSM[1].insert("+",Stateaction::Reduce(1));
 parser1.RSM[1].insert("*",Stateaction::Shift(6));
 parser1.RSM[2].insert("F",Stateaction::Gotonext(4));
 parser1.RSM[2].insert("T",Stateaction::Gotonext(1));
 parser1.RSM[2].insert("(",Stateaction::Shift(2));
 parser1.RSM[2].insert("num",Stateaction::Shift(5));
 parser1.RSM[2].insert("E",Stateaction::Gotonext(7));
 parser1.RSM[3].insert("+",Stateaction::Shift(8));
 parser1.RSM[3].insert("EOF",Stateaction::Accept);
 parser1.RSM[4].insert("*",Stateaction::Reduce(3));
 parser1.RSM[4].insert(")",Stateaction::Reduce(3));
 parser1.RSM[4].insert("EOF",Stateaction::Reduce(3));
 parser1.RSM[4].insert("+",Stateaction::Reduce(3));
 parser1.RSM[5].insert("EOF",Stateaction::Reduce(5));
 parser1.RSM[5].insert("+",Stateaction::Reduce(5));
 parser1.RSM[5].insert(")",Stateaction::Reduce(5));
 parser1.RSM[5].insert("*",Stateaction::Reduce(5));
 parser1.RSM[6].insert("F",Stateaction::Gotonext(9));
 parser1.RSM[6].insert("num",Stateaction::Shift(5));
 parser1.RSM[6].insert("(",Stateaction::Shift(2));
 parser1.RSM[7].insert(")",Stateaction::Shift(10));
 parser1.RSM[7].insert("+",Stateaction::Shift(8));
 parser1.RSM[8].insert("T",Stateaction::Gotonext(11));
 parser1.RSM[8].insert("F",Stateaction::Gotonext(4));
 parser1.RSM[8].insert("num",Stateaction::Shift(5));
 parser1.RSM[8].insert("(",Stateaction::Shift(2));
 parser1.RSM[9].insert("*",Stateaction::Reduce(2));
 parser1.RSM[9].insert("+",Stateaction::Reduce(2));
 parser1.RSM[9].insert("EOF",Stateaction::Reduce(2));
 parser1.RSM[9].insert(")",Stateaction::Reduce(2));
 parser1.RSM[10].insert("EOF",Stateaction::Reduce(4));
 parser1.RSM[10].insert(")",Stateaction::Reduce(4));
 parser1.RSM[10].insert("*",Stateaction::Reduce(4));
 parser1.RSM[10].insert("+",Stateaction::Reduce(4));
 parser1.RSM[11].insert("+",Stateaction::Reduce(0));
 parser1.RSM[11].insert("EOF",Stateaction::Reduce(0));
 parser1.RSM[11].insert("*",Stateaction::Shift(6));
 parser1.RSM[11].insert(")",Stateaction::Reduce(0));
 return parser1;
} //make_parser
