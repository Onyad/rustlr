//Parser generated by rustlr

#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(irrefutable_let_patterns)]
use std::any::Any;
extern crate rustlr;
use rustlr::{Tokenizer,TerminalToken,ZCParser,ZCRProduction,Stateaction,decode_action,LBox,lbdown,lbup,lbget,unbox};
use crate::exprtrees::*;
use crate::exprtrees::Expr::*;

const SYMBOLS:[&'static str;12] = ["E","ES","+","-","*","/","(",")",";","int","START","EOF"];

const TABLE:[u64;99] = [25770131456,65537,4295098369,12885098496,38654967808,281509337038848,281483567104000,281492157104128,281487862005760,281496452202496,562949954142209,562962838519808,562975723552768,562988608389120,562997198061571,844437815230464,844450700263424,844463585099776,844424930918401,1125934266580994,1125908496777218,1125929971613698,1125917086711810,1125912791744514,1125921381679106,1407387768651776,1407413538521088,1407374884405249,1407400653684736,1688875630395392,1688862745362432,1688849861181441,1688888515231744,1970337722073088,1970350607106048,1970324837957633,1970363491942400,2251799814733825,2251812698783744,2251825583816704,2251838468653056,2533322035494914,2533300560658434,2533287675756546,2533313445560322,2814749768220673,2814788422074368,2814762652205056,2814775537238016,3096233334210560,3096241924210688,3096246219309056,3096259104735232,3096237629112320,3377729785626626,3377721196019712,3377712605757442,3377716900921344,3377734080593922,3377708310790146,3659183287631872,3659196172730368,3659191877632000,3659204763254784,3659187582533632,3940671149441024,3940662558982146,3940658264014850,3940684033818626,3940666854342656,3940679738851330,4222141831053312,4222154715496450,4222159010463746,4222137535627266,4222146126151680,4222133240659970,4503629692403714,4503608217567234,4503633987371010,4503612512534530,4503621102469122,4503616807501826,4785104669048834,4785096079114242,4785083194212354,4785091784146946,4785108964016130,4785087489179650,5066588236021762,5066575351119874,5066596825956354,5066562466217986,5348054622666754,5348037442797570,5348058917634050,5348033147830274,5348046032732162,5348041737764866,];

fn _semaction_for_0_(parser:&mut ZCParser<LBox<dyn Any>,i64>) -> Expr {
let mut m = lbdown!(parser.popstack().value,Expr);  unbox!(m) }
fn _semaction_for_1_(parser:&mut ZCParser<LBox<dyn Any>,i64>) -> Expr {
let mut e2 = lbdown!(parser.popstack().value,Expr); let mut _item1_ = parser.popstack().value; let mut e1 = lbdown!(parser.popstack().value,Expr);  Plus(e1,e2) }
fn _semaction_for_2_(parser:&mut ZCParser<LBox<dyn Any>,i64>) -> Expr {
let mut e2 = lbdown!(parser.popstack().value,Expr); let mut _item1_ = parser.popstack().value; let mut e1 = lbdown!(parser.popstack().value,Expr);  Minus(e1,e2) }
fn _semaction_for_3_(parser:&mut ZCParser<LBox<dyn Any>,i64>) -> Expr {
let mut e2 = lbdown!(parser.popstack().value,Expr); let mut _item1_ = parser.popstack().value; let mut e1 = lbdown!(parser.popstack().value,Expr);  Divide(e1,e2) }
fn _semaction_for_4_(parser:&mut ZCParser<LBox<dyn Any>,i64>) -> Expr {
let mut e2 = lbdown!(parser.popstack().value,Expr); let mut _item1_ = parser.popstack().value; let mut e1 = lbdown!(parser.popstack().value,Expr);  Times(e1,e2) }
fn _semaction_for_5_(parser:&mut ZCParser<LBox<dyn Any>,i64>) -> Expr {
let mut e = lbdown!(parser.popstack().value,Expr); let mut _item0_ = parser.popstack().value;  Negative(e) }
fn _semaction_for_6_(parser:&mut ZCParser<LBox<dyn Any>,i64>) -> Expr {
let mut _item2_ = parser.popstack().value; let mut e = lbdown!(parser.popstack().value,Expr); let mut _item0_ = parser.popstack().value;  *e.exp }
fn _semaction_for_7_(parser:&mut ZCParser<LBox<dyn Any>,i64>) -> Vec<LBox<Expr>> {
let mut _item1_ = parser.popstack().value; let mut n = lbdown!(parser.popstack().value,Expr);  vec![n] }
fn _semaction_for_8_(parser:&mut ZCParser<LBox<dyn Any>,i64>) -> Vec<LBox<Expr>> {
let mut _item2_ = parser.popstack().value; let mut e = lbdown!(parser.popstack().value,Expr); let mut v = lbdown!(parser.popstack().value,Vec<LBox<Expr>>); 
   v.push(e);
   unbox!(v)
   }
fn _semaction_for_9_(parser:&mut ZCParser<LBox<dyn Any>,i64>) -> LBox<dyn Any> {
let mut _item0_ = lbdown!(parser.popstack().value,Vec<LBox<Expr>>); <LBox<dyn Any>>::default()}

pub fn create_parser<'t>(tokenizer:&'t mut dyn Tokenizer<'t,LBox<dyn Any>>) -> ZCParser<'t,LBox<dyn Any>,i64>
{
 let mut parser1:ZCParser<LBox<dyn Any>,i64> = ZCParser::new(10,20,tokenizer);
 let mut rule = ZCRProduction::<LBox<dyn Any>,i64>::new_skeleton("start");
 rule = ZCRProduction::<LBox<dyn Any>,i64>::new_skeleton("E");
 rule.Ruleaction = |parser|{  lbup!( LBox::new(_semaction_for_0_(parser),parser.linenum,parser.column)) };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<LBox<dyn Any>,i64>::new_skeleton("E");
 rule.Ruleaction = |parser|{  lbup!( LBox::new(_semaction_for_1_(parser),parser.linenum,parser.column)) };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<LBox<dyn Any>,i64>::new_skeleton("E");
 rule.Ruleaction = |parser|{  lbup!( LBox::new(_semaction_for_2_(parser),parser.linenum,parser.column)) };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<LBox<dyn Any>,i64>::new_skeleton("E");
 rule.Ruleaction = |parser|{  lbup!( LBox::new(_semaction_for_3_(parser),parser.linenum,parser.column)) };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<LBox<dyn Any>,i64>::new_skeleton("E");
 rule.Ruleaction = |parser|{  lbup!( LBox::new(_semaction_for_4_(parser),parser.linenum,parser.column)) };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<LBox<dyn Any>,i64>::new_skeleton("E");
 rule.Ruleaction = |parser|{  lbup!( LBox::new(_semaction_for_5_(parser),parser.linenum,parser.column)) };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<LBox<dyn Any>,i64>::new_skeleton("E");
 rule.Ruleaction = |parser|{  lbup!( LBox::new(_semaction_for_6_(parser),parser.linenum,parser.column)) };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<LBox<dyn Any>,i64>::new_skeleton("ES");
 rule.Ruleaction = |parser|{  lbup!( LBox::new(_semaction_for_7_(parser),parser.linenum,parser.column)) };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<LBox<dyn Any>,i64>::new_skeleton("ES");
 rule.Ruleaction = |parser|{  lbup!( LBox::new(_semaction_for_8_(parser),parser.linenum,parser.column)) };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<LBox<dyn Any>,i64>::new_skeleton("START");
 rule.Ruleaction = |parser|{  lbup!( LBox::new(_semaction_for_9_(parser),parser.linenum,parser.column)) };
 parser1.Rules.push(rule);
 parser1.Errsym = "";
 parser1.resynch.insert(";");

 for i in 0..99 {
   let symi = ((TABLE[i] & 0x0000ffff00000000) >> 32) as usize;
   let sti = ((TABLE[i] & 0xffff000000000000) >> 48) as usize;
   parser1.RSM[sti].insert(SYMBOLS[symi],decode_action(TABLE[i]));
 }

 for s in SYMBOLS { parser1.Symset.insert(s); }

 load_extras(&mut parser1);
 return parser1;
} //make_parser

fn load_extras(parser:&mut ZCParser<LBox<dyn Any>,i64>)
{
}//end of load_extras: don't change this line as it affects augmentation
