//Parser generated by rustlr for grammar calc4

#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(irrefutable_let_patterns)]
extern crate rustlr;
use rustlr::{Tokenizer,TerminalToken,ZCParser,ZCRProduction,Stateaction,decode_action};
use rustlr::{StrTokenizer,RawToken,LexSource};
use std::collections::{HashMap,HashSet};
use crate::exprtrees::*; /* ! lines are injected verbatim into parser */
use crate::exprtrees::Expr::*;
use rustlr::{LBox,makelbox};

const SYMBOLS:[&'static str;16] = ["E","ES","+","-","*","/","(",")","=",";","let","in","int","var","START","EOF"];

const TABLE:[u64;177] = [12885032960,51539935232,55834836992,42949738496,4295360513,196609,25770262528,281530811547648,281487861743616,281500746973184,281526516645888,281517926449152,281474977234945,562975723683840,562992903159808,562962838454272,563005788258304,563001493356544,562949954011137,844433520918528,844463585492992,844437815754752,844446405754880,844442110918656,1125947151548418,1125912791810050,1125917086777346,1125908496842754,1125929971679234,1125934266646530,1125921381744642,1125938561613826,1407404948324354,1407422128193538,1407409243291650,1407396358389762,1407383473487874,1407387768455170,1407392063422466,1407413538258946,1688849861246977,1688862745296896,1688892810002432,1688901400199168,1688875630526464,1688914284773379,1688905695100928,1970367786713088,1970350607237120,1970337722007552,1970376376909824,1970324838023169,1970380671811584,2251821289308160,2251816994471936,2251812699308032,2251808404471808,2251834174537728,2533291971182592,2533296266018816,2533309150593026,2533287675756546,2533322035494914,2533313445560322,2533304855625730,2533283380789250,2814792717369346,2814775537500162,2814814192205826,2814801307303938,2814762652598274,2814805602271234,3096280578654208,3096276283752448,3096250514079744,3096237628850176,3096224744996865,3096267693555712,3377699721773057,3377712605560832,3377755555364864,3377725490790400,3377751260463104,3377742670266368,3659174698549249,3659200467501056,3659230532075520,3659187582271488,3659217646977024,3659226237173760,3940675444211712,3940662558982144,3940649675325441,3940705508786176,3940701213884416,3940692623687680,4222146126282752,4222163306807296,4222137536282624,4222141831446528,4222133241446400,4503629693648896,4503612512993280,4503616808157184,4503621102993408,4503608218157056,4785126144016384,4785117553819648,4785087489114112,4785100374343680,4785074605654017,4785130438918144,5066562465955842,5066596825694210,5066571056414720,5066579645825026,5066566761578496,5066558170988546,5066583940792322,5066588235759618,5348054622601218,5348071802470402,5348058917568514,5348041737699330,5348063212535810,5348046032666626,5348033147764738,5348037442732034,5629538189115394,5629546779049986,5629521009836032,5629533894148098,5629512419311618,5629529599180802,5629508124344322,5629516714999808,5911021755957250,5910987396218882,5911008871055362,5911004576088066,5910983101251586,5910991691186178,5911013166022658,5910995986153474,6192462373191682,6192475258093570,6192501027897346,6192505322864642,6192513912799234,6192492437962754,6473963119575042,6473958824607746,6473945939705858,6473933054803970,6473941644738562,6473971709509634,6473937349771266,6473954529640450,6755446687334400,6755420916678656,6755408031842304,6755416621842432,6755412326678528,7036887302799360,7036925957701632,7036900188028928,7036874419470337,7036930252603392,7036917367504896,7318379459379202,7318366575263744,7318396639248386,7318388049313794,7318370870099968,7318357985263616,7318362280099840,7318383754346498,];

pub fn make_parser<'src_lt>() -> ZCParser<Expr<'src_lt>,i64>
{
 let mut parser1:ZCParser<Expr<'src_lt>,i64> = ZCParser::new(12,27);
 let mut rule = ZCRProduction::<Expr<'src_lt>,i64>::new_skeleton("start");
 rule = ZCRProduction::<Expr<'src_lt>,i64>::new_skeleton("E");
 rule.Ruleaction = |parser|{ let mut m = parser.popstack();  m.value };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<Expr<'src_lt>,i64>::new_skeleton("E");
 rule.Ruleaction = |parser|{ let mut s = parser.popstack(); 
  if let (Var(v),)=(&mut s.value,) {  s.value }  else {parser.bad_pattern("(Var(v),)")} };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<Expr<'src_lt>,i64>::new_skeleton("E");
 rule.Ruleaction = |parser|{ let mut b = parser.popstack(); let mut _item4_ = parser.popstack(); let mut e = parser.popstack(); let mut _item2_ = parser.popstack(); let mut _item1_ = parser.popstack(); let mut _item0_ = parser.popstack(); 
  if let (Var(x),)=(_item1_.value,) { Letexp(x,e.lbox(),b.lbox())}  else {parser.bad_pattern("(Var(x),)")} };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<Expr<'src_lt>,i64>::new_skeleton("E");
 rule.Ruleaction = |parser|{ let mut e2 = parser.popstack(); let mut _item1_ = parser.popstack(); let mut e1 = parser.popstack();  Plus(e1.lbox(), e2.lbox()) };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<Expr<'src_lt>,i64>::new_skeleton("E");
 rule.Ruleaction = |parser|{ let mut e2 = parser.popstack(); let mut _item1_ = parser.popstack(); let mut e1 = parser.popstack();  Minus(e1.lbox(), parser.lbx(2,e2.value))};
 parser1.Rules.push(rule);
 rule = ZCRProduction::<Expr<'src_lt>,i64>::new_skeleton("E");
 rule.Ruleaction = |parser|{ let mut e2 = parser.popstack(); let mut _item1_ = parser.popstack(); let mut e1 = parser.popstack();  Divide(e1.lbox(), e2.lbox())};
 parser1.Rules.push(rule);
 rule = ZCRProduction::<Expr<'src_lt>,i64>::new_skeleton("E");
 rule.Ruleaction = |parser|{ let mut e2 = parser.popstack(); let mut _item1_ = parser.popstack(); let mut e1 = parser.popstack();  Times(e1.lbox(), e2.lbox())};
 parser1.Rules.push(rule);
 rule = ZCRProduction::<Expr<'src_lt>,i64>::new_skeleton("E");
 rule.Ruleaction = |parser|{ let mut e = parser.popstack(); let mut _item0_ = parser.popstack();  Negative(e.lbox()) };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<Expr<'src_lt>,i64>::new_skeleton("E");
 rule.Ruleaction = |parser|{ let mut _item2_ = parser.popstack(); let mut e = parser.popstack(); let mut _item0_ = parser.popstack();  e.value };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<Expr<'src_lt>,i64>::new_skeleton("ES");
 rule.Ruleaction = |parser|{ let mut _item1_ = parser.popstack(); let mut n = parser.popstack();  Seq(vec![n.lbox()]) };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<Expr<'src_lt>,i64>::new_skeleton("ES");
 rule.Ruleaction = |parser|{ let mut _item2_ = parser.popstack(); let mut e = parser.popstack(); let mut _item0_ = parser.popstack(); 
  if let (Seq(mut v),)=(_item0_.value,) { 
   v.push(e.lbox());
   Seq(v)
   }  else {parser.bad_pattern("(Seq(mut v),)")} };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<Expr<'src_lt>,i64>::new_skeleton("START");
 rule.Ruleaction = |parser|{ let mut _item0_ = parser.popstack(); <Expr<'src_lt>>::default()};
 parser1.Rules.push(rule);
 parser1.Errsym = "";
 parser1.resynch.insert(";");

 for i in 0..177 {
   let symi = ((TABLE[i] & 0x0000ffff00000000) >> 32) as usize;
   let sti = ((TABLE[i] & 0xffff000000000000) >> 48) as usize;
   parser1.RSM[sti].insert(SYMBOLS[symi],decode_action(TABLE[i]));
 }

 for s in SYMBOLS { parser1.Symset.insert(s); }

 load_extras(&mut parser1);
 return parser1;
} //make_parser


// Lexical Scanner using RawToken and StrTokenizer
pub struct calc4lexer<'t> {
   stk: StrTokenizer<'t>,
   keywords: HashSet<&'static str>,
}
impl<'t> calc4lexer<'t> 
{
  pub fn from_str(s:&'t str) -> calc4lexer<'t>  {
    Self::new(StrTokenizer::from_str(s))
  }
  pub fn from_source(s:&'t LexSource<'t>) -> calc4lexer<'t>  {
    Self::new(StrTokenizer::from_source(s))
  }
  pub fn new(mut stk:StrTokenizer<'t>) -> calc4lexer<'t> {
    let mut keywords = HashSet::with_capacity(16);
    for kw in ["let","in",] {keywords.insert(kw);}
    for c in ['+','-','*','/','(',')','=',';',] {stk.add_single(c);}
    for d in [] {stk.add_double(d);}
    stk.set_line_comment("#");
    calc4lexer {stk,keywords}
  }
}
impl<'src_lt> Tokenizer<'src_lt,Expr<'src_lt>> for calc4lexer<'src_lt>
{
   fn nextsym(&mut self) -> Option<TerminalToken<'src_lt,Expr<'src_lt>>> {
    let tokopt = self.stk.next_token();
    if let None = tokopt {return None;}
    let token = tokopt.unwrap();
    match token.0 {
      RawToken::Alphanum(sym) if self.keywords.contains(sym) => Some(TerminalToken::from_raw(token,sym,<Expr<'src_lt>>::default())),
      RawToken::Num(n) => Some(TerminalToken::from_raw(token,"int",Val(n))),
      RawToken::Alphanum(x) => Some(TerminalToken::from_raw(token,"var",Var(x))),
      RawToken::Symbol(s) => Some(TerminalToken::from_raw(token,s,<Expr<'src_lt>>::default())),
      _ => Some(TerminalToken::from_raw(token,"<LexicalError>",<Expr<'src_lt>>::default())),
    }
  }
   fn linenum(&self) -> usize {self.stk.line()}
   fn column(&self) -> usize {self.stk.column()}
   fn position(&self) -> usize {self.stk.current_position()}
}//impl Tokenizer

fn load_extras<'src_lt>(parser:&mut ZCParser<Expr<'src_lt>,i64>)
{
}//end of load_extras: don't change this line as it affects augmentation
