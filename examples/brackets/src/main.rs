//Parser generated by rustlr for grammar brackets

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
//anything on a ! line is injected verbatim into the generated parser

fn main() {
  let argv:Vec<String> = std::env::args().collect(); // command-line args
  let mut parser1 = make_parser();
  let mut lexer1 = bracketslexer::from_str(&argv[1]);
  let result = parser1.parse(&mut lexer1);
  if !parser1.error_occurred() {
    println!("parsed successfully with result {:?}",&result);
  }
  else {println!("parsing failed; partial result is {:?}",&result);}
}//main

const SYMBOLS:[&'static str;12] = ["E","S","WS","(",")","[","]","LBRACE","RBRACE","Whitespace","START","EOF"];

const TABLE:[u64;92] = [38655033346,21475164162,12885229570,8590000129,47244967938,4295098369,30065098754,281500746711042,281509336645634,281505041678338,281513631612928,281487861809154,281522221547522,281496451743746,281492156776450,562962838716416,562980018454528,562949953748993,562997198061571,562971428716544,844454995296258,844472175165442,844463585230850,844446405361666,844442110394370,844437815427074,844450700328962,844459290263554,1125934266908674,1125904202334209,1125912792072194,1125929971941378,1125921382006786,1125908496842753,1125938561875970,1407392063684610,1407409243553794,1407422128455682,1407387768717314,1407396358651906,1407404948586498,1407400653619202,1688879925362690,1688871335428098,1688854155821057,1688862745493506,1688888515297282,1688858450264065,1688867040460802,1970350607106050,1970354902073346,1970329132597249,1970346312138754,1970337722204162,1970363492007938,1970333426974721,2251821288980480,2251799814012929,2251829878718464,2251834174144512,2251812698980352,2533274790723585,2533296265691136,2533291971051520,2533304855429120,2533287675691008,2814749767434241,2814762652401664,2814775537762304,2814779832139776,2814771242401792,3096254808719362,3096246218784770,3096259103686658,3096250513752066,3096271988588546,3096241923817474,3096237628850178,3377734080266242,3377725490331650,3377716900397058,3377746965168130,3377721195364354,3377712605429762,3377729785298946,3659204762075138,3659209057042434,3659187582205954,3659191877173250,3659196172140546,3659221941944322,3659200467107842,];

pub fn make_parser() -> ZCParser<(u32,u32,u32),(u32,u32,u32)>
{
 let mut parser1:ZCParser<(u32,u32,u32),(u32,u32,u32)> = ZCParser::new(8,14);
 let mut rule = ZCRProduction::<(u32,u32,u32),(u32,u32,u32)>::new_skeleton("start");
 rule = ZCRProduction::<(u32,u32,u32),(u32,u32,u32)>::new_skeleton("E");
 rule.Ruleaction = |parser|{ let mut _item2_ = parser.popstack(); let mut _item1_ = parser.popstack(); let mut _item0_ = parser.popstack(); 
  if let ((a,b,c),)=(_item1_.value,) { (a+1,b,c)}  else {parser.bad_pattern("((a,b,c),)")} };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<(u32,u32,u32),(u32,u32,u32)>::new_skeleton("E");
 rule.Ruleaction = |parser|{ let mut _item2_ = parser.popstack(); let mut _item1_ = parser.popstack(); let mut _item0_ = parser.popstack(); 
  if let ((a,b,c),)=(_item1_.value,) { (a,b+1,c)}  else {parser.bad_pattern("((a,b,c),)")} };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<(u32,u32,u32),(u32,u32,u32)>::new_skeleton("E");
 rule.Ruleaction = |parser|{ let mut _item2_ = parser.popstack(); let mut _item1_ = parser.popstack(); let mut _item0_ = parser.popstack(); 
  if let ((a,b,c),)=(_item1_.value,) { (a,b,c+1)}  else {parser.bad_pattern("((a,b,c),)")} };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<(u32,u32,u32),(u32,u32,u32)>::new_skeleton("S");
 rule.Ruleaction = |parser|{ let mut _item0_ = parser.popstack();  (0,0,0) };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<(u32,u32,u32),(u32,u32,u32)>::new_skeleton("S");
 rule.Ruleaction = |parser|{ let mut _item1_ = parser.popstack(); let mut _item0_ = parser.popstack(); 
  if let ((p,q,r),(a,b,c),)=(_item1_.value,_item0_.value,) { (a+p,b+q,c+r)}  else {parser.bad_pattern("((p,q,r),(a,b,c),)")} };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<(u32,u32,u32),(u32,u32,u32)>::new_skeleton("WS");
 rule.Ruleaction = |parser|{ <(u32,u32,u32)>::default()};
 parser1.Rules.push(rule);
 rule = ZCRProduction::<(u32,u32,u32),(u32,u32,u32)>::new_skeleton("WS");
 rule.Ruleaction = |parser|{ let mut _item1_ = parser.popstack(); let mut _item0_ = parser.popstack(); <(u32,u32,u32)>::default()};
 parser1.Rules.push(rule);
 rule = ZCRProduction::<(u32,u32,u32),(u32,u32,u32)>::new_skeleton("START");
 rule.Ruleaction = |parser|{ let mut _item0_ = parser.popstack(); <(u32,u32,u32)>::default()};
 parser1.Rules.push(rule);
 parser1.Errsym = "";
 parser1.resynch.insert("Whitespace");

 for i in 0..92 {
   let symi = ((TABLE[i] & 0x0000ffff00000000) >> 32) as usize;
   let sti = ((TABLE[i] & 0xffff000000000000) >> 48) as usize;
   parser1.RSM[sti].insert(SYMBOLS[symi],decode_action(TABLE[i]));
 }

 for s in SYMBOLS { parser1.Symset.insert(s); }

 load_extras(&mut parser1);
 return parser1;
} //make_parser


// Lexical Scanner using RawToken and StrTokenizer
pub struct bracketslexer<'t> {
   stk: StrTokenizer<'t>,
   keywords: HashSet<&'static str>,
}
impl<'t> bracketslexer<'t> 
{
  pub fn from_str(s:&'t str) -> bracketslexer<'t>  {
    Self::new(StrTokenizer::from_str(s))
  }
  pub fn from_source(s:&'t LexSource<'t>) -> bracketslexer<'t>  {
    Self::new(StrTokenizer::from_source(s))
  }
  pub fn new(mut stk:StrTokenizer<'t>) -> bracketslexer<'t> {
    let mut keywords = HashSet::with_capacity(16);
    for kw in [] {keywords.insert(kw);}
    for c in ['(',')','[',']','{','}',] {stk.add_single(c);}
    for d in [] {stk.add_double(d);}
    stk.keep_whitespace = true;
    bracketslexer {stk,keywords}
  }
}
impl<'t> Tokenizer<'t,(u32,u32,u32)> for bracketslexer<'t>
{
   fn nextsym(&mut self) -> Option<TerminalToken<'t,(u32,u32,u32)>> {
    let tokopt = self.stk.next_token();
    if let None = tokopt {return None;}
    let token = tokopt.unwrap();
    match token.0 {
      RawToken::Alphanum(sym) if self.keywords.contains(sym) => Some(TerminalToken::from_raw(token,sym,<(u32,u32,u32)>::default())),
      RawToken::Whitespace(_) => Some(TerminalToken::from_raw(token,"Whitespace",(0,0,0))),
      RawToken::Symbol(r"{") => Some(TerminalToken::from_raw(token,"LBRACE",<(u32,u32,u32)>::default())),
      RawToken::Symbol(r"}") => Some(TerminalToken::from_raw(token,"RBRACE",<(u32,u32,u32)>::default())),
      RawToken::Symbol(s) => Some(TerminalToken::from_raw(token,s,<(u32,u32,u32)>::default())),
      _ => Some(TerminalToken::from_raw(token,"<LexicalError>",<(u32,u32,u32)>::default())),
    }
  }
   fn linenum(&self) -> usize {self.stk.line()}
   fn column(&self) -> usize {self.stk.column()}
   fn position(&self) -> usize {self.stk.current_position()}
}//impl Tokenizer

fn load_extras(parser:&mut ZCParser<(u32,u32,u32),(u32,u32,u32)>)
{
}//end of load_extras: don't change this line as it affects augmentation
