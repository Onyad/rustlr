#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(unused_doc_comments)]
#![allow(unused_imports)]
use std::fmt::Display;
use std::default::Default;
use std::collections::{HashMap,HashSet,BTreeSet};
use std::io::{self,Read,Write,BufReader,BufRead};
use std::cell::{RefCell,Ref,RefMut};
use std::hash::{Hash,Hasher};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::mem;
use crate::{TRACE,Lexer,Lextoken,Stateaction,Statemachine,augment_file};
use crate::{LBox,LRc};
use crate::Stateaction::*;

/// this structure is only exported because it is required by the generated parsers.
/// There is no reason to use it in other programs.
#[derive(Clone)]
pub struct RProduction<AT:Default,ET:Default>  // runtime rep of grammar rule
{
  pub lhs: &'static str, // left-hand side nonterminal of rule
  pub Ruleaction : fn(&mut RuntimeParser<AT,ET>) -> AT, //parser as arg
}
impl<AT:Default,ET:Default> RProduction<AT,ET>
{
  pub fn new_skeleton(lh:&'static str) -> RProduction<AT,ET>
  {
     RProduction {
       lhs : lh,
       Ruleaction : |p|{<AT>::default()},
     }
  }
}//impl RProduction

pub struct Stackelement<AT:Default>
{
   pub si : usize, // state index
   pub value : AT, // semantic value (don't clone grammar symbols)
}

/// this is the structure created by the generated parser.  The generated parser
/// program will contain a make_parser function that returns this structure.
/// Most of the pub items are, however, only exported to support the operation
/// of the parser, and should not be accessed directly.  Only the functions
/// [RuntimeParser::parse], [RuntimeParser::report], [RuntimeParser::abort]
/// and [RuntimeParser::error_occurred] should be called directly 
/// from user programs.  Only the field [RuntimeParser::exstate] should be accessed
/// by user programs.
pub struct RuntimeParser<AT:Default,ET:Default>  
{
  /// this is the "external state" structure, with type ET defined by the grammar.
  /// The semantic actions associated with each grammar rule, which are written
  /// in the grammar, have ref mut access to the RuntimeParser structure, which
  /// allows them to read and change the external state object.  This gives
  /// the parsers greater flexibility and capability, including the ability to
  /// parse some non-context free languages.  See 
  /// [this sample grammar](<https://cs.hofstra.edu/~cscccl/rustlr_project/ncf.grammar>).
  /// The exstate is initialized to ET::default().
  pub exstate : ET,  // external state structure, usage optional
  /// used only by generated parser: do not reference
  pub RSM : Vec<HashMap<&'static str,Stateaction>>,  // runtime state machine
  // do not reference
  //pub Expected : Vec<Vec<&'static str>>,
  /// do not reference
  pub Rules : Vec<RProduction<AT,ET>>, //rules with just lhs and delegate function
  ////// this value should be set through abort or report
  stopparsing : bool,
  /// do not reference  
  pub stack :  Vec<Stackelement<AT>>, // parse stack
//  pub recover : HashSet<&'static str>, // for error recovery
  pub resynch : HashSet<&'static str>,
  pub Errsym : &'static str,
  err_occurred : bool,
  pub linenum : usize,
  pub column : usize,
  pub src_id : usize,
  report_line : usize,
  training : bool,
  pub trained: HashMap<(usize,String),String>,
  /// Hashset containing all grammar symbols (terminal and non-terminal). This is used for error reporting and training.
  pub Symset : HashSet<&'static str>,
}//struct RuntimeParser

impl<AT:Default,ET:Default> RuntimeParser<AT,ET>
{
    /// this is only called by the make_parser function in the machine-generated
    /// parser program.  *Do not call this function in other places* as it
    /// only generates a skeleton.
    pub fn new(rlen:usize, slen:usize) -> RuntimeParser<AT,ET>
    {  // given number of rules and number states
       let mut p = RuntimeParser {
         RSM : Vec::with_capacity(slen),
         //Expected : Vec::with_capacity(slen),
         Rules : Vec::with_capacity(rlen),
         stopparsing : false,
         exstate : ET::default(),
         stack : Vec::with_capacity(1024),
         Errsym : "",
         err_occurred : false,
         linenum : 0,
         column : 0,
         src_id : 0,
         report_line : 0,
         resynch : HashSet::new(),
         //added for training
         training : false,
         trained : HashMap::new(),
         Symset : HashSet::with_capacity(64),
       };
       for _ in 0..slen {
         p.RSM.push(HashMap::with_capacity(16));
         //p.Expected.push(Vec::new());
       }
       return p;
    }//new

    /// this function can be called from with the "semantic" actions attached
    /// to grammar production rules that are executed for each
    /// "reduce" action of the parser.
    pub fn abort(&mut self, msg:&str)
    {
       eprintln!("\n!!!Parsing Aborted: {}",msg);
       self.err_occurred = true;
       self.stopparsing=true;
    }

    /// may be called from grammar semantic actions to report error.
    /// this report function will print to stdout. 
    pub fn report(&mut self, errmsg:&str)  
    {      // linenum must be set prior to call
       if (self.report_line != self.linenum || self.linenum==0)  {
//         print!("ERROR on line {}, column {}:\n{}\n",self.linenum,self.column,tokenizer.current_line());         
         print!("ERROR on line {}, column {}: {}",self.linenum,self.column,errmsg);
         self.report_line = self.linenum;
       }
       else {
         print!(" {} ",errmsg);
       }
       self.err_occurred = true;
    }

    /// sets an index that index source information, such as the source file
    /// when compiling multiple sources. This must be maintained externally.
    /// The source id will also be passed on to the [LBox] smartpointers by
    /// the [RuntimeParser::lb] function.
    pub fn set_src_id(&mut self, id:usize)
    { self.src_id =id; }

    //called to simulate a shift
    fn errshift(&mut self, sym:&str) -> bool
    {
       let csi = self.stack[self.stack.len()-1].si; // current state
       let actionopt = self.RSM[csi].get(sym);
       if let Some(Shift(ni)) = actionopt {
         self.stack.push(Stackelement{si:*ni,value:AT::default()}); true
       }
       else {false}
    }

    fn reduce(&mut self, ri:&usize)
    {
              let rulei = &self.Rules[*ri];
              let ruleilhs = rulei.lhs; // &'static : Copy
              let val = (rulei.Ruleaction)(self); // calls delegate function
              let newtop = self.stack[self.stack.len()-1].si; 
              let goton = self.RSM[newtop].get(ruleilhs).unwrap();
//              if TRACE>1 {println!(" ..performing Reduce({}), new state {}, action on {}: {:?}..",ri,newtop,ruleilhs,goton);}
              if let Stateaction::Gotonext(nsi) = goton {
                self.stack.push(Stackelement{si:*nsi,value:val});
                // DO NOT CHANGE LOOKAHEAD AFTER REDUCE!
              }// goto next state after reduce
              else {
                self.report("state transition table corrupted: no suitable action after reduce");
                self.stopparsing=true;
              }
    }//reduce

    /// can be called to determine if an error occurred during parsing.  The parser
    /// will not panic.
    pub fn error_occurred(&self) -> bool {self.err_occurred}

    fn nexttoken(&self, tokenizer:&mut dyn Lexer<AT>) -> Lextoken<AT>
    {
       if let Some(tok) = tokenizer.nextsym() {tok}
        else { Lextoken{sym:"EOF".to_owned(),  value:AT::default()} } 
    }

    /// Parse in training mode: when an error occurs, the parser will
    /// ask the human trainer for an appropriate error message: it will
    /// then insert an entry into its state transition table to
    /// give the same error message on future errors of the same type.
    /// If the error is caused by an unexpected token that is recognized
    /// as a terminal symbol of the grammar, the trainer can select to
    /// enter the entry 
    /// under the reserved ANY_ERROR symbol. If the unexpected token is
    /// not recognized as a grammar symbol, then the entry will always
    /// be entered under ANY_ERROR.  ANY_ERROR entries for a state will match
    /// all future unexpected symbols for that state: however, entries for
    /// valid grammar symbols will still override the generic entry.
    ///
    /// Example: with the parser for this [toy grammar](https://cs.hofstra.edu/~cscccl/rustlr_project/cpm.grammar), parse_train can run as follows:
    ///```ignore
    ///  Write something in C+- : cout << x y ;   
    ///  ERROR on line 1, column 0: unexpected symbol y ..
    ///  >>>TRAINER: is this error message adequate? If not, enter a better one: need another <<                   
    ///  >>>TRAINER: should this message be given for all unexpected symbols in the current state? (default yes) yes
    ///```
    /// (ignore the column number as the lexer for this toy language does not implement it)
    ///
    /// parse_train will then modify [the parser file](https://cs.hofstra.edu/~cscccl/rustlr_project/augmented_cpmparser.rs) as specified
    /// by the filename (path) argument.  When the augmented parser is used, it will
    /// give a more helpful error message:
    ///```
    /// Write something in C+- : cout << x endl
    /// ERROR on line 1, column 0: unexpected symbol endl, **need another <<** ..
    ///```
    ///
    /// parse_train calls parse, which uses stdin/stdout for user interface.
    pub fn parse_train(&mut self, tokenizer:&mut dyn Lexer<AT>, filename:&str) -> AT
    {
      self.training = true;
      let result = self.parse(tokenizer);
      if let Err(m) = augment_file(filename,self) {
        eprintln!("Error in augmenting parser: {:?}",m)
      }
      self.training = false;
      return result;
    }//parse_train

    /// creates a [LBox] smart pointer that includes line/column/src information;
    /// should be called from the semantic actions of a grammar rule, e.g.
    ///```ignore
    ///   E --> E:a + E:b {PlusExpr(parser.lb(a),parser.lb(b))}
    ///```
    pub fn lb(&self,e:AT) -> LBox<AT> { LBox::new(e,self.linenum,self.column,self.src_id) }
    /// similar to [RuntimeParser::lb], but creates a [LRc] instead of [LBox]
    pub fn lrc(&self,e:AT) -> LRc<AT> { LRc::new(e,self.linenum,self.column,self.src_id) }    
}// impl RuntimeParser


//////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////
//// new version of write_fsm:

impl Statemachine
{
  pub fn writeparser(&self, filename:&str)->Result<(),std::io::Error>
  {
    let mut fd = File::create(filename)?;
    write!(fd,"//Parser generated by rustlr\n
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
extern crate rustlr;
use rustlr::{{RuntimeParser,RProduction,Stateaction,decode_action}};\n")?;

    write!(fd,"{}\n",&self.Gmr.Extras)?; // use clauses

    // write static array of symbols
    write!(fd,"const SYMBOLS:[&'static str;{}] = [",self.Gmr.Symbols.len())?;
    for i in 0..self.Gmr.Symbols.len()-1
    {
      write!(fd,"\"{}\",",&self.Gmr.Symbols[i].sym)?;
    }
    write!(fd,"\"{}\"];\n\n",&self.Gmr.Symbols[self.Gmr.Symbols.len()-1].sym)?;
    // position of symbols must be inline with self.Gmr.Symhash

    // record table entries in a static array
    let mut totalsize = 0;
    for i in 0..self.FSM.len() { totalsize+=self.FSM[i].len(); }
    write!(fd,"const TABLE:[u64;{}] = [",totalsize)?;
    // generate table to represent FSM
    let mut encode:u64 = 0;
    for i in 0..self.FSM.len() // for each state index i
    {
      let row = &self.FSM[i];
      for key in row.keys()
      { // see function decode for opposite translation
        let k = *self.Gmr.Symhash.get(key).unwrap(); // index of symbol
        encode = ((i as u64) << 48) + ((k as u64) << 32);
        match row.get(key) {
          Some(Shift(statei)) => { encode += (*statei as u64) << 16; },
          Some(Gotonext(statei)) => { encode += ((*statei as u64) << 16)+1; },
          Some(Reduce(rulei)) => { encode += ((*rulei as u64) << 16)+2; },
          Some(Accept) => {encode += 3; },
          _ => {encode += 4; },  // 4 indicates Error
        }//match
        write!(fd,"{},",encode)?;
      } //for symbol index k
    }//for each state index i
    write!(fd,"];\n\n")?;

    // must know what absyn type is when generating code.
    let ref absyn = self.Gmr.Absyntype;
    let ref extype = self.Gmr.Externtype;
    write!(fd,"pub fn make_parser() -> RuntimeParser<{},{}>",absyn,extype)?; 
    write!(fd,"\n{{\n")?;
    // write code to pop stack, assign labels to variables.
    write!(fd," let mut parser1:RuntimeParser<{},{}> = RuntimeParser::new({},{});\n",absyn,extype,self.Gmr.Rules.len(),self.States.len())?;
    // generate rules and Ruleaction delegates, must pop values from runtime stack
    write!(fd," let mut rule = RProduction::<{},{}>::new_skeleton(\"{}\");\n",absyn,extype,"start")?;
    for i in 0..self.Gmr.Rules.len() 
    {
      write!(fd," rule = RProduction::<{},{}>::new_skeleton(\"{}\");\n",absyn,extype,self.Gmr.Rules[i].lhs.sym)?;      
      write!(fd," rule.Ruleaction = |parser|{{ ")?;
      let mut k = self.Gmr.Rules[i].rhs.len();
      while k>0
      {
        let gsym = &self.Gmr.Rules[i].rhs[k-1];
        if gsym.label.len()>0 && &gsym.rusttype[0..3]=="mut"
          { write!(fd," let mut {}:{}=",gsym.label,absyn)?; }        
        else if gsym.label.len()>0
          { write!(fd," let {}:{}=",gsym.label,absyn)?; }
        write!(fd,"parser.stack.pop()")?; 
        if gsym.label.len()>0 { write!(fd,".unwrap().value;  ")?;}
        else {write!(fd,";  ")?;}
        k -= 1;
      } // for each symbol on right hand side of rule  
      let mut semaction = &self.Gmr.Rules[i].action; //this is a string
      //if semaction.len()<1 {semaction = "}}";}
      //if al>1 {semaction = semaction.substring(0,al-1);}
      if semaction.len()>1 {write!(fd,"{};\n",semaction.trim_end())?;}
      else {write!(fd," return <{}>::default();}};\n",absyn)?;}
      write!(fd," parser1.Rules.push(rule);\n")?;
    }// for each rule
    write!(fd," parser1.Errsym = \"{}\";\n",&self.Gmr.Errsym)?;
    // resynch vector
    for s in &self.Gmr.Resynch {write!(fd," parser1.resynch.insert(\"{}\");\n",s)?;}

    // generate code to load RSM from TABLE
    write!(fd,"\n for i in 0..{} {{\n",totalsize)?;
    write!(fd,"   let symi = ((TABLE[i] & 0x0000ffff00000000) >> 32) as usize;\n")?;
    write!(fd,"   let sti = ((TABLE[i] & 0xffff000000000000) >> 48) as usize;\n")?;
    write!(fd,"   parser1.RSM[sti].insert(SYMBOLS[symi],decode_action(TABLE[i]));\n }}\n\n")?;
//    write!(fd,"\n for i in 0..{} {{for k in 0..{} {{\n",rows,cols)?;
//    write!(fd,"   parser1.RSM[i].insert(SYMBOLS[k],decode_action(TABLE[i*{}+k]));\n }}}}\n",cols)?;
    write!(fd," for s in SYMBOLS {{ parser1.Symset.insert(s); }}\n\n")?;

    write!(fd," load_extras(&mut parser1);\n")?;
    write!(fd," return parser1;\n")?;
    write!(fd,"}} //make_parser\n\n")?;

    ////// Augment!
    write!(fd,"fn load_extras(parser:&mut RuntimeParser<{},{}>)\n{{\n",absyn,extype)?;
    write!(fd,"}}//end of load_extras: don't change this line as it affects augmentation\n")?;
    Ok(())
  }//writeparser


//////////////
///////////////// non-binary version (no augmentation) //////////////////
pub fn write_verbose(&self, filename:&str)->Result<(),std::io::Error>
  {
    let mut fd = File::create(filename)?;
    write!(fd,"//Parser generated by rustlr\n
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
extern crate rustlr;
use rustlr::{{RuntimeParser,RProduction,Stateaction}};\n")?;

    write!(fd,"{}\n",&self.Gmr.Extras)?; // use clauses
    let ref absyn = self.Gmr.Absyntype;
    let ref extype = self.Gmr.Externtype;
    write!(fd,"pub fn make_parser() -> RuntimeParser<{},{}>",absyn,extype)?; 
    write!(fd,"\n{{\n")?;
    // write code to pop stack, assign labels to variables.
    write!(fd," let mut parser1:RuntimeParser<{},{}> = RuntimeParser::new({},{});\n",absyn,extype,self.Gmr.Rules.len(),self.States.len())?;
    // generate rules and Ruleaction delegates, must pop values from runtime stack
    write!(fd," let mut rule = RProduction::<{},{}>::new_skeleton(\"{}\");\n",absyn,extype,"start")?;
    for i in 0..self.Gmr.Rules.len() 
    {
      write!(fd," rule = RProduction::<{},{}>::new_skeleton(\"{}\");\n",absyn,extype,self.Gmr.Rules[i].lhs.sym)?;      
      write!(fd," rule.Ruleaction = |parser|{{ ")?;
      let mut k = self.Gmr.Rules[i].rhs.len();
      while k>0
      {
        let gsym = &self.Gmr.Rules[i].rhs[k-1];
        if gsym.label.len()>0 && &gsym.rusttype[0..3]=="mut"
          { write!(fd," let mut {}:{}=",gsym.label,absyn)?; }        
        else if gsym.label.len()>0
          { write!(fd," let {}:{}=",gsym.label,absyn)?; }
        write!(fd,"parser.stack.pop()")?; 
        if gsym.label.len()>0 { write!(fd,".unwrap().value;  ")?;}
        else {write!(fd,";  ")?;}
        k -= 1;
      } // for each symbol on right hand side of rule  
      let mut semaction = &self.Gmr.Rules[i].action; //this is a string
      //if semaction.len()<1 {semaction = "}}";}
      //if al>1 {semaction = semaction.substring(0,al-1);}
      if semaction.len()>1 {write!(fd,"{};\n",semaction.trim_end())?;}
      else {write!(fd," return <{}>::default();}};\n",absyn)?;}
      write!(fd," parser1.Rules.push(rule);\n")?;
    }// for each rule
    write!(fd," parser1.Errsym = \"{}\";\n",&self.Gmr.Errsym)?;
    // resynch vector
    for s in &self.Gmr.Resynch {write!(fd," parser1.resynch.insert(\"{}\");\n",s)?;}
    
    for i in 0..self.FSM.len()
    {
      let row = &self.FSM[i];
      for key in row.keys()
      {
        write!(fd," parser1.RSM[{}].insert(\"{}\",Stateaction::{:?});\n",i,key,row.get(key).unwrap())?;
      } //for each string key in row
    }//for each state index i

    write!(fd," return parser1;\n")?;
    write!(fd,"}} //make_parser\n")?;
    Ok(())
  }//write_verbose

} // impl Statemachine


//// independent function
    fn iserror(actionopt:&Option<&Stateaction>) -> bool
    {
       match actionopt {
           None => true,
           Some(Error(_)) => true,
           _ => false,
         }
    }//iserror





///////////////////////////////////////////////////////////////////////////
////// reimplementing the parsing algorithm more modularly, with aim of
////// allowing custom parsers
//////////// errors should compile a report

/// In case one wishes to construct a parser error-reporting interface
/// that's different from the supplied [RuntimeParser::parse] function,
/// which prints to stdout, a function of ErrorReporter type can be defined
/// and used in conjuction with [RuntimeParser::parse_core.
pub type ErrorReporter<AT,ET> =
  fn(&mut RuntimeParser<AT,ET>, &Lextoken<AT>, &Option<Stateaction>);
  

impl<AT:Default,ET:Default> RuntimeParser<AT,ET>
{

  // reduce already implemented
  // no separate function for gotonext - part of reduce

  /// this is the LR parser shift action: push the next state, along with the
  /// value of the current lookahead token onto the parse stack, returns the
  /// next token
  fn shift(&mut self, nextstate:usize, lookahead:Lextoken<AT>, tokenizer:&mut dyn Lexer<AT>) -> Lextoken<AT>
  {
     self.stack.push(Stackelement{si:nextstate,value:lookahead.value});
     self.nexttoken(tokenizer)
  }

  /// This is the core parser, which expects a ErrorReporter function to be
  /// passed in as an argument.
  pub fn parse_core(&mut self, tokenizer:&mut dyn Lexer<AT>, err_reporter:ErrorReporter<AT,ET>) -> AT
  {
    self.stack.clear();
    self.err_occurred = false;
    let mut result = AT::default();
    self.stack.push(Stackelement {si:0, value:AT::default()});
    self.stopparsing = false;
    let mut action = Stateaction::Error("");
    let mut lookahead = Lextoken{sym:"EOF".to_owned(),value:AT::default()};
    if let Some(tok) = tokenizer.nextsym() {lookahead=tok;}
    else {self.stopparsing=true;}

    while !self.stopparsing
    {
      self.linenum = tokenizer.linenum(); self.column=tokenizer.column();
      let currentstate = self.stack[self.stack.len()-1].si;
      let mut actionopt = self.RSM[currentstate].get(lookahead.sym.as_str());
      let actclone:Option<Stateaction> = match actionopt {
        Some(a) => Some(*a),
        None => None,
      };
      if iserror(&actionopt) {  // either None or Error
        if !self.err_occurred {self.err_occurred = true;}
        err_reporter(self,&lookahead,&actclone);
        match self.error_recover(&mut lookahead,tokenizer) {
          None => { self.stopparsing=true; break; }
          Some(act) => {action = act;},
        }//match
      }// iserror
      else { action = actclone.unwrap(); }
      match &action {
        Shift(nextstate) => {
           lookahead = self.shift(*nextstate,lookahead,tokenizer);
        },
        Reduce(rulei) => { self.reduce(rulei); },
        Accept => {
          self.stopparsing=true;
          if self.stack.len()>0 {result = self.stack.pop().unwrap().value;}
          else {self.err_occurred=true;}
        },
        _ => {}, // continue
      }//match action
    }// main parse loop
    return result;
  }//parse_core

  /// this function is used to invoke the generated parser returned by
  /// the generated parser program's make_parser function.  This
  /// function invokes parse_core with err_report_train as the ErrorReporter
  /// function.
  pub fn parse(&mut self, tokenizer:&mut dyn Lexer<AT>) -> AT
  {
     self.parse_core(tokenizer,err_report_train)
  }

  /// Error recovery routine of rustlr, separate from error_reporter.
  /// This function will modify the parser and lookahead symbol and return
  /// either the next action the parser should take (if recovery succeeded)
  /// or None if recovery failed.
  pub fn error_recover<'t>(&mut self, lookahead:&mut Lextoken<AT>,tokenizer:&mut dyn Lexer<AT>) -> Option<Stateaction>
  {
    let mut erraction = None;
    ///// prefer to ue Errsym method
    if self.Errsym.len()>0 {
      let errsym = self.Errsym;
      // lookdown stack for state with trainsiton on Errsym
      // but that could be current state too (start at top)
      let mut k = self.stack.len(); // offset by 1 because of usize
      let mut spos = k+1;
      while k>0 && spos>k
      {
        let ksi = self.stack[k-1].si;
        erraction = self.RSM[ksi].get(errsym);
        if let None = erraction {k-=1;} else {spos=k;}
      }//while k>0
      if spos==k { self.stack.truncate(k); } // new current state revealed
      // run all reduce actions that are valid before the Errsym:
      while let Some(Reduce(ri)) = erraction // keep reducing
      {
       //self.reduce(ri); // borrow error- only need mut self.stack
              let rulei = &self.Rules[*ri];
              let ruleilhs = rulei.lhs; // &'static : Copy
              let val = (rulei.Ruleaction)(self); // calls delegate function
              let newtop = self.stack[self.stack.len()-1].si; 
              let gotonopt = self.RSM[newtop].get(ruleilhs);
              match gotonopt {
                Some(Gotonext(nsi)) => { 
                  self.stack.push(Stackelement{si:*nsi,value:val});
                },// goto next state after reduce
                _ => {self.abort("recovery failed"); },
              }//match
              // end reduce
              let tos=self.stack[self.stack.len()-1].si;
              erraction = self.RSM[tos].get(self.Errsym);
      } // while let erraction is reduce
      // remaining defined action on Errsym must be shift
      if let Some(Shift(i)) = erraction { // simulate shift errsym 
          self.stack.push(Stackelement{si:*i,value:AT::default()});
          // keep lookahead until action is found that transitions from
          // current state (i). but skipping ahead without reducing
          // the error production is not a good idea
          while let None = self.RSM[*i].get(&lookahead.sym[..]) {
            if &lookahead.sym[..]=="EOF" {break;}
            *lookahead = self.nexttoken(tokenizer);
          }//while let
          // either at end of input or found action on next symbol
          erraction = self.RSM[*i].get(&lookahead.sym[..]);
      } // if shift action found down under stack
    }//errsym exists

    // at this point, if erraction is None, then Errsym failed to recover,
    // try the resynch symbol method next ...
    if iserror(&erraction) && self.resynch.len()>0 {
      while &lookahead.sym!="EOF" &&
        !self.resynch.contains(&lookahead.sym[..]) {
        self.linenum = tokenizer.linenum(); self.column = tokenizer.column();
        *lookahead = self.nexttoken(tokenizer);
      }//while
      if &lookahead.sym!="EOF" {
        // look for state on stack that has action defined on next symbol
        self.linenum = tokenizer.linenum(); self.column = tokenizer.column();    
        *lookahead = self.nexttoken(tokenizer); // skipp err-causing symbol
      }
      let mut k = self.stack.len()-1; // offset by 1 because of usize
      let mut position = 0;
      while k>0 && erraction==None
      {
         let ksi = self.stack[k-1].si;
         erraction = self.RSM[ksi].get(&lookahead.sym[..]);
         if let None=erraction {k-=1;}
      }//while k>0 && erraction==None
      match erraction {
        None => {}, // do nothing, whill shift next symbol
        _ => { self.stack.truncate(k);},//pop stack
      }//match
   }// there are resync symbols

   // at this point, if erraction is None, then resynch recovery failed too.
   // only action left is to skip ahead...
   let mut eofcx = 0;
   while iserror(&erraction) && eofcx<1 { //skip input
      self.linenum = tokenizer.linenum(); self.column = tokenizer.column();
      *lookahead = self.nexttoken(tokenizer);
      if &lookahead.sym=="EOF" {eofcx+=1;}
      let csi =self.stack[self.stack.len()-1].si;
      erraction = self.RSM[csi].get(&lookahead.sym[..]);
   }// skip ahead
   match erraction {
     None => None,
     Some(act) => Some(*act),
   }//return match
  }//error_recover function

}//impl RuntimeParser 2


/// default ErrorReporter, with training ability
pub fn err_report_train<AT:Default,ET:Default>(parser:&mut RuntimeParser<AT,ET>, lookahead:&Lextoken<AT>, erropt:&Option<Stateaction>)
{
  // known that actionop is None or Some(Error(_))
  let cstate = parser.stack[parser.stack.len()-1].si;
  let mut actionopt = if let Some(act)=erropt {Some(act)} else {None};
  let lksym = &lookahead.sym[..];
  // is lookahead recognized as a grammar symbol?
  // if actionopt is NONE, check entry for ANY_ERROR            
  if parser.Symset.contains(lksym) {
     if let None=actionopt {
        actionopt = parser.RSM[cstate].get("ANY_ERROR");
     }
  }// lookahead is recognized grammar sym
  else {
     actionopt = parser.RSM[cstate].get("ANY_ERROR");
  }// lookahead is not a grammar sym
  let errmsg = if let Some(Error(em)) = &actionopt {
    format!("unexpected symbol {}, ** {} ** ..",lksym,em)
  } else {format!("unexpected symbol {} .. ",lksym)};

  parser.report(&errmsg);
         
  if parser.training {  /////// TRAINING MODE:
    let cstate = parser.stack[parser.stack.len()-1].si;
    let csym = lookahead.sym.clone();
    let mut inp = String::from("");
    print!("\n>>>TRAINER: if this message is not adequate (for state {}), enter a replacement (default no change): ",cstate);
    let rrrflush = io::stdout().flush();
    if let Ok(n) = io::stdin().read_line(&mut inp) {
       if inp.len()>5 && parser.Symset.contains(lksym) {
         print!(">>>TRAINER: should this message be given for all unexpected symbols in the current state? (default yes) ");
        let rrrflush2 = io::stdout().flush();
        let mut inp2 = String::new();
        if let Ok(n) = io::stdin().read_line(&mut inp2) {
            if inp2.trim()=="no" || inp2.trim()=="No" {
               parser.trained.insert((cstate,csym),inp);
            }
            else  {// insert for any error
                       parser.trained.insert((cstate,String::from("ANY_ERROR")),inp);
            }
        }// read ok
       }// unexpected symbol is grammar sym
       else if inp.len()>5 && !parser.Symset.contains(lksym) {
         parser.trained.insert((cstate,String::from("ANY_ERROR")),inp);
       }
    }// process user response
  }//if training   //// END TRAINING MODE

}// default errorreporter function - conforms to type ErrorReporter


/////////////// new approach using more flexible trait object
pub trait ErrHandler<AT:Default,ET:Default> // not same as error recovery
{
  fn err_reporter(&mut self, parser:&mut RuntimeParser<AT,ET>, lookahead:&Lextoken<AT>, erropt:&Option<Stateaction>);
//  fn training_mode(&self, parser:&RuntimeParser<AT,ET>) -> bool {false}
//  fn interactive_mode(&self, parser:&RuntimeParser<AT,ET>) -> bool {false}
}// ErrReporter trait


////  Default trainer, can train interactively or from script
pub struct StandardReporter
{
  pub  training : bool,
//    pub  interactive : bool,  scriptinopt==None
  pub  trained : HashMap<(usize,String),String>,
  pub  scriptinopt:  Option<BufReader<File>>,   // for training from script
  pub  scriptoutopt: Option<File>,   // created during interactive training
}
impl StandardReporter
{
  /// creates default standard reporter, used by parse_stdio (does not train)
  pub fn new() -> StandardReporter
  {
    StandardReporter {
      training:false, trained:HashMap::new(), scriptinopt:None, scriptoutopt:None,}
  }
  /// creates a stdio error handler with interactive training, takes as
  /// argument parser file name, to create script for future retraining.
  pub fn new_interactive_training(existingparser:&str) -> StandardReporter
  {
    let outfile =  format!("{}_script.txt", existingparser);
    let mut fout = File::create(outfile).expect("failed to create training script file");
    let _ = write!(fout,"# Rustlr training script for {}\n\n",existingparser);
    StandardReporter {
      training:true, trained:HashMap::with_capacity(8),
      scriptoutopt:Some(fout), scriptinopt:None,}     
  }
  /// creates a stdio error handler that trains (non-interactively) from
  /// a previously created script.  It's the user's responsibility to match
  /// the script file with the input source.
  pub fn new_script_training(existingparser:&str,scriptfile:&str) -> StandardReporter
  {
    let fin = BufReader::new(File::open(scriptfile).expect("failed to open training script file"));
    StandardReporter {
      training:true, trained:HashMap::with_capacity(32),
      scriptoutopt:None,
      scriptinopt:Some(fin), }     
  }  
  // augment_train implemented in augmenter.rs
}//impl StandardReporter

impl<AT:Default,ET:Default> ErrHandler<AT,ET> for StandardReporter
{
  // this function will be able to write training script to file
  fn err_reporter(&mut self, parser:&mut RuntimeParser<AT,ET>, lookahead:&Lextoken<AT>, erropt:&Option<Stateaction>)
 { 
  let mut wresult:std::io::Result<()> = Err(std::io::Error::new(std::io::ErrorKind::Other,"")); // dummy
  // known that actionop is None or Some(Error(_))
  let cstate = parser.stack[parser.stack.len()-1].si; // current state
  let mut actionopt = if let Some(act)=erropt {Some(act)} else {None};
  let lksym = &lookahead.sym[..];
  // is lookahead recognized as a grammar symbol?
  // if actionopt is NONE, check entry for ANY_ERROR            
  if parser.Symset.contains(lksym) {
     if let None=actionopt {
        actionopt = parser.RSM[cstate].get("ANY_ERROR");
     }
  }// lookahead is recognized grammar sym
  else {
     actionopt = parser.RSM[cstate].get("ANY_ERROR");
  }// lookahead is not a grammar sym
  let errmsg = if let Some(Error(em)) = &actionopt {
    format!("unexpected symbol {}, ** {} ** ..",lksym,em.trim())
  } else {format!("unexpected symbol {} .. ",lksym)};

  parser.report(&errmsg);

  if self.training {          ////// Training mode
    let csym = lookahead.sym.clone();
    let mut inp = String::from("");    
   if let None=self.scriptinopt {  // interactive mode
   if let Some(outfd1) = &self.scriptoutopt {
    let mut outfd = outfd1;
    print!("\n>>>TRAINER: if this message is not adequate (for state {}), enter a replacement (default no change): ",cstate);
    let rrrflush = io::stdout().flush();
    if let Ok(n) = io::stdin().read_line(&mut inp) {
       if inp.len()>5 && parser.Symset.contains(lksym) {
         print!(">>>TRAINER: should this message be given for all unexpected symbols in the current state? (default yes) ");
        let rrrflush2 = io::stdout().flush();
        let mut inp2 = String::new();
        if let Ok(n) = io::stdin().read_line(&mut inp2) {
            if inp2.trim()=="no" || inp2.trim()=="No" {
               wresult = write!(outfd,"{}\t{}\t{} ::: {}\n",parser.linenum,parser.column,&csym,inp.trim());
               self.trained.insert((cstate,csym),inp);
            }
            else  {// insert for any error
               wresult = write!(outfd,"{}\t{}\t{} ::: {}\n",parser.linenum,parser.column,"ANY_ERROR",inp.trim());
               self.trained.insert((cstate,String::from("ANY_ERROR")),inp);
            }
        }// read ok
       }// unexpected symbol is grammar sym
       else if inp.len()>5 && !parser.Symset.contains(lksym) {
         wresult = write!(outfd,"{}\t{}\t{} ::: {}\n",parser.linenum,parser.column,"ANY_ERROR",inp.trim());
         self.trained.insert((cstate,String::from("ANY_ERROR")),inp);
       }
    }// process user response
   }}// interactive mode
   else { // training from script mode (non-interactive)
    if let Some(brfd) = &mut self.scriptinopt {
     let mut scin = brfd;
     let mut readn = 0;
     while readn < 1
     {
       inp = String::new();
       match scin.read_line(&mut inp) {
         Ok(n) if n>1 && &inp[0..1]!="#" && inp.trim().len()>0 => {readn=n;},
         Ok(n) if n>0 => { readn=0; }, // keep reading
         _ => {readn = 1; } // stop - this means End of Stream
       }//match
       if readn>1 { // read something
         let inpsplit:Vec<&str> = inp.split_whitespace().collect();
         if inpsplit.len()>4 && inpsplit[3].trim()==":::" {
           let inline = inpsplit[0].trim().parse::<usize>().unwrap();
           let incolumn = inpsplit[1].trim().parse::<usize>().unwrap();
           let insym = inpsplit[2].trim();
           if parser.linenum==inline && parser.column==incolumn {
             if &csym==insym || insym=="ANY_ERROR" {
               let posc = inp.find(":::").unwrap()+4;
               println!("\n>>>Found matching entry from training script for {}, error message: {}",insym,&inp[posc..]);
               self.trained.insert((cstate,String::from(insym)),String::from(&inp[posc..]));
             } // unexpected symbol match
           }// line/column match
         }//inpsplit check
       }// valid training line read
     }//while readn<2
   }}//training from script mode
  }//if training   //// END TRAINING MODE
  
 }// standardreporter function
}// impl ErrHandler for StandardReporter


//////////////// temporary: live side by side with parse_core
impl<AT:Default,ET:Default> RuntimeParser<AT,ET>
{
  /// core parser (temporarily lives side by side with parse_core) that
  /// takes dynamic trait objects for lexical scanner and err_reporting.
  /// This design makes it possible to create a custom error reporting
  /// interface, such as a graphical IDE interface, while still using the
  /// base parser state machine generated by rustlr.
  pub fn parse_base(&mut self, tokenizer:&mut dyn Lexer<AT>, err_handler:&mut dyn ErrHandler<AT,ET>) -> AT
  {
    self.stack.clear();
    self.err_occurred = false;
    let mut result = AT::default();
    self.stack.push(Stackelement {si:0, value:AT::default()});
    self.stopparsing = false;
    let mut action = Stateaction::Error("");
    let mut lookahead = Lextoken{sym:"EOF".to_owned(),value:AT::default()};
    if let Some(tok) = tokenizer.nextsym() {lookahead=tok;}
    else {self.stopparsing=true;}

    while !self.stopparsing
    {
      self.linenum = tokenizer.linenum(); self.column=tokenizer.column();
      let currentstate = self.stack[self.stack.len()-1].si;
      let mut actionopt = self.RSM[currentstate].get(lookahead.sym.as_str());
      let actclone:Option<Stateaction> = match actionopt {
        Some(a) => Some(*a),
        None => None,
      };
      if iserror(&actionopt) {  // either None or Error
        if !self.err_occurred {self.err_occurred = true;}
        
        err_handler.err_reporter(self,&lookahead,&actclone);
        //err_reporter(self,&lookahead,&actclone);
        
        match self.error_recover(&mut lookahead,tokenizer) {
          None => { self.stopparsing=true; break; }
          Some(act) => {action = act;},
        }//match
      }// iserror
      else { action = actclone.unwrap(); }
      match &action {
        Shift(nextstate) => {
           lookahead = self.shift(*nextstate,lookahead,tokenizer);
        },
        Reduce(rulei) => { self.reduce(rulei); },
        Accept => {
          self.stopparsing=true;
          if self.stack.len()>0 {result = self.stack.pop().unwrap().value;}
          else {self.err_occurred=true;}
        },
        _ => {}, // continue
      }//match action
    }// main parse loop
    return result;
  }//parse_base

  ///provided generic parsing function that reports errors on std::io
  pub fn parse_stdio(&mut self, tokenizer:&mut dyn Lexer<AT>) -> AT
  {
    let mut stdeh = StandardReporter::new();
    self.parse_base(tokenizer,&mut stdeh) 
  }//parse_stdio

  ///parses in interactive training mode with provided path to parserfile.
  ///parser file will be modified and a training script file will be
  ///created for future retraining after grammar is modified. 
  pub fn parse_stdio_train(&mut self, tokenizer:&mut dyn Lexer<AT>, parserfile:&str) -> AT
    {
      let mut stdtrainer = StandardReporter::new_interactive_training(parserfile);
      let result = self.parse_base(tokenizer,&mut stdtrainer);
      if let Err(m) = stdtrainer.augment_training(parserfile) {
        eprintln!("Error in augmenting parser: {:?}",m)
      }

      return result;
    }//parse_stdio_train

  /// trains parser from training script created by interactive training.  this
  /// is intended to be used after a grammar has been modified and the parser
  /// is regenerated with different state numbers.  It is the user's
  /// responsibility to keep consistent the parser file, script file, and sample
  /// input that was used when the script was created.  The script contains
  /// the line and column numbers of each error encountered, along with either
  /// unexpected symbol that caused the error, or the reserved ANY_ERROR
  /// symbol if the error message is to be applied to all unexpected symbols.
  /// These entries must match, in sequence, the errors encountered during
  /// retraining - it is therefore recommended that the same tokenizer be used
  /// during retraining so that the same line/column information are given.
  /// The trainer will augment the parser (parserfile) with new Error
  /// entries, overriding any previous ones.  It is also recommended that the
  /// user examines the "load_extras" function that appears at the end of
  /// the augmented parser.  The train_from_script function does not return
  /// a value, unlike [RuntimeParser::parse_stdio] and [RuntimeParser::parse_stdio_train].
  pub fn train_from_script(&mut self, tokenizer:&mut dyn Lexer<AT>, parserfile:&str, scriptfile:&str)
  {
      let mut stdtrainer = StandardReporter::new_script_training(parserfile,scriptfile);
      let result = self.parse_base(tokenizer,&mut stdtrainer);
      if let Err(m) = stdtrainer.augment_training(parserfile) {
        eprintln!("Error in augmenting parser: {:?}",m)
      }
      if !self.err_occurred {println!("no errors encountered during parsing");}
  }//train_from_script

}// 3rd impl RuntimeParser
