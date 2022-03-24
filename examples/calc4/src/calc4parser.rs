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
#![allow(unreachable_patterns)]
extern crate rustlr;
use rustlr::{Tokenizer,TerminalToken,ZCParser,ZCRProduction,Stateaction,decode_action};
use rustlr::{StrTokenizer,RawToken,LexSource};
use std::collections::{HashMap,HashSet};
use crate::exprtrees::*; /* ! lines are injected verbatim into parser */
use crate::exprtrees::Expr::*;
use rustlr::{LBox,makelbox};

const SYMBOLS:[&'static str;16] = ["E","ES","+","-","*","/","(",")","=",";","let","in","int","var","START","EOF"];

const TABLE:[u64;177] = [42950000640,12885295104,4295032833,55835033600,196609,51539869696,25769934848,281487862005760,281517926711296,281530811744256,281474977234945,281526516580352,281500746645504,281539401220099,563001493291008,563005788454912,562975723356160,562962838716416,562992903421952,562949954011137,844437815754752,844442110787584,844433520918528,844446405623808,844463585755136,1125917086711810,1125921381679106,1125929971613698,1125947151482882,1125912791744514,1125908496777218,1125938561548290,1125934266580994,1407400653488128,1407426423422976,1407430718586880,1407387768848384,1407374884536321,1407417833553920,1688849861312513,1688892810264576,1688905695297536,1688862745559040,1688901400133632,1688875630198784,1970342016909314,1970354901811202,1970346311876610,1970337721942018,1970372081680386,1970363491745794,1970359196778498,1970333426974722,2251838469505024,2251812699308032,2251821289177088,2251816994340864,2251808404471808,2533287676018688,2533283381182464,2533304856346624,2533296265887744,2533291971051520,2814775537041408,2814792717107200,2814749768351745,2814801306976256,2814805602140160,2814762652401664,3096280578850816,3096250513752064,3096237629112320,3096224745127937,3096267693817856,3096276283686912,3377712605822976,3377751260397568,3377755555561472,3377699721904129,3377725490462720,3377742670528512,3659226237108224,3659200467173376,3659174698680321,3659230532272128,3659217647239168,3659187582533632,3940714099048450,3940701214146562,3940662559440898,3940675444342786,3940705509113858,3940692624211970,4222137536282624,4222146126151680,4222141831315456,4222159011905536,4222133241446400,4503621102862336,4503616808026112,4503608217763842,4503638282534914,4503646872469506,4503633987567618,4503629692600322,4503612512731138,4785087489638402,4785139029245954,4785126144344066,4785130439311362,4785100374540290,4785117554409474,5066566761185282,5066579646087170,5066562466217986,5066571056152578,5066583941054466,5066588236021762,5066558171250690,5066596825956354,5348063212535810,5348054622601218,5348041737699330,5348033147764738,5348037442732034,5348071802470402,5348046032666626,5348058917568514,5629516714868736,5629533894213634,5629512419377154,5629538189180930,5629521009704960,5629546779115522,5629529599246338,5629508124409858,5910995986153474,5910987396218882,5911021755957250,5911008871055362,5910991691186178,5911004576088066,5910983101251586,5911013166022658,6192488142536706,6192483847569410,6192496732471298,6192466668290048,6192479552602114,6192458077765634,6192462372732930,6192470963126272,6473924465917953,6473976004214784,6473950234279936,6473967414345728,6473937349640192,6473980299378688,6755420916547584,6755446687334400,6755412326678528,6755408031842304,6755416621711360,7036887303061504,7036874419470337,7036917367767040,7036925957636096,7036930252800000,7036900187701248,7318366575132672,7318396639248386,7318362280099840,7318379459379202,7318357985263616,7318388049313794,7318383754346498,7318370869968896,];

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
      RawToken::Alphanum(s) => Some(TerminalToken::from_raw(token,s,<Expr<'src_lt>>::default())),
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
