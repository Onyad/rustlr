//Parser generated by RustLr

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
extern crate rustlr;
use rustlr::{RuntimeParser,RProduction,Stateaction,decode_action};

const SYMBOLS:[&'static str;10] = ["E","T","F","+","*","(",")","num","START","EOF"];

const TABLE:[u64;71] = [4295098369,30064836608,262145,8590131201,21475164160,281492156907522,281513631744002,281487861940226,562988608192514,562962838388738,562967133683712,844463585034242,844442110197762,844437815230466,1125912792203264,1125938561548291,1407404948848640,1407374884143105,1407379179175937,1407396359176192,1407383474208769,1688871335428096,1688879925100544,1688858451050497,1970354901811200,1970333427105793,1970346312138752,1970329132859393,2251812698914818,2251816993882114,2251825583816706,2533300561182720,2533287676346368,2814762652073986,2814775536975874,2814766948089856,3096241923883010,3096237628915714,3096250513817602,3377704016150529,3377699721707521,3377721196150784,3377729785823232,3377708311183361,3659213352075266,3659191877238786,3659187582271490,3940662558851074,3940688328654850,3940666854211584,4222141830791170,4222163305627650,4222137535823874,4503608218025985,4503603923582977,4503621102993408,4503629692665856,4785083195326465,4785096079704064,4785104669376512,5066562466742272,5066575351971840,5348050327306242,5348041738485760,5348037442404354,5629516714213378,5629512419246082,5629525304147970,5910987396087810,5911000280989698,5910991691055106,];

pub fn make_parser() -> RuntimeParser<i32,i32>
{
 let mut parser1:RuntimeParser<i32,i32> = RuntimeParser::new(7,22);
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

 for i in 0..71 {
   let symi = ((TABLE[i] & 0x0000ffff00000000) >> 32) as usize;
   let sti = ((TABLE[i] & 0xffff000000000000) >> 48) as usize;
   parser1.RSM[sti].insert(SYMBOLS[symi],decode_action(TABLE[i]));
 }

 return parser1;
} //make_parser
