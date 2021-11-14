// grammar processing module
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(unused_doc_comments)]
#![allow(unused_imports)]
//use std::fmt::Display;
//use std::default::Default;
use std::collections::{HashMap,HashSet,BTreeSet};
use std::cell::{RefCell,Ref,RefMut};
use std::hash::{Hash,Hasher};
use std::io::{self,Read,Write,BufReader,BufRead};
use std::fs::File;
use std::io::prelude::*;

pub const DEFAULTPRECEDENCE:i32 = 20;
pub const TRACE:usize = 0;

#[derive(Clone)]
pub struct Gsym // struct for a grammar symbol
{
  pub sym : String,
  pub rusttype : String,  //used only to indicate "mut"
  pub terminal : bool,
  pub label : String,  // object-level variable holding value
  pub precedence : i32,   // negatives indicate right associativity
}

impl Gsym
{
  pub fn new(s:&str,isterminal:bool) -> Gsym // compile time
  {
    Gsym {
      sym : s.to_owned(),
      terminal : isterminal,
      label : String::default(),
      rusttype : String::from("String"),
      precedence : DEFAULTPRECEDENCE, // + means left, - means right
    }
  }
  pub fn setlabel(&mut self, la:&str)
  { self.label = String::from(la); }
  pub fn settype(&mut self, rt:&str)
  { self.rusttype = String::from(rt); }
  pub fn setprecedence(&mut self, p:i32)
  { self.precedence = p; }
}// impl for Gsym


//Grammar Rule structure
// This will be used only statically: the action is a string.
// The Gsym structures are repeated on the right-hand side because each
// one can have a different label
pub struct Grule  // struct for a grammar rule
{
  pub lhs : Gsym,  // left-hand side of rule
  pub rhs : Vec<Gsym>, // right-hand side symbols (cloned from Symbols)
  pub action : String, //string representation of Ruleaction
  pub precedence : i32, // set to rhs symbol with highest |precedence|
}
impl Grule
{
  pub fn new_skeleton(lh:&str) -> Grule
  {
     Grule {
       lhs : Gsym::new(lh,false),
       rhs : Vec::new(),
       action : String::default(),
       precedence : 0,   
     }
  }
}//impl Grule

pub fn printrule(rule:&Grule)  //independent function
{
   print!("PRODUCTION: {} --> ",rule.lhs.sym);
   for s in &rule.rhs {
      print!("{}",s.sym);
      if s.label.len()>0 {print!(":{}",s.label);}
      print!(" ");
   }
   println!("{{ {}, preclevel {}",rule.action,rule.precedence);  // {{ is \{
}

/////main global class, roughly corresponds to "metaparser"
pub struct Grammar
{
  pub name : String,
  pub Symbols : Vec<Gsym>,
  pub Symhash : HashMap<String,usize>,
  pub Rules: Vec<Grule>,
  pub topsym : String,
  pub Nullable : HashSet<String>,
  pub First : HashMap<String,HashSet<String>>,
  pub Rulesfor: HashMap<String,HashSet<usize>>,  //rules for a non-terminal
  pub Absyntype : String,     // string name of abstract syntax type
  pub Externtype : String,    // type of external structure
  pub Resynch : HashSet<String>, // resynchronization terminal symbols, ordered
  pub Errsym : String,        // error recovery terminal symbol
  pub Lexnames : HashMap<String,String>, // print names of terminals
  pub Extras : String,        // indicated by {% .. %}, mostly  use ...
}

impl Grammar
{
  pub fn new() -> Grammar
  {
     Grammar {
       name : String::from(""),       // name of grammar
       Symbols: Vec::new(),           // grammar symbols
       Symhash: HashMap::new(),       
       Rules: Vec::new(),                 // production rules
       topsym : String::default(),        // top symbol
       Nullable : HashSet::new(),
       First : HashMap::new(),
       Rulesfor: HashMap::new(),
       Absyntype:String::from("i64"), //default(),
       Externtype:String::from(""),   // default unused field
//       Recover : HashSet::new(),
       Resynch : HashSet::new(),
       Errsym : String::new(),
       Lexnames : HashMap::new(),
       Extras: String::new(),
     }
  }//new grammar

  pub fn nonterminal(&self,s:&str) -> bool
  {
     match self.Symhash.get(s) {
        Some(symi) => !self.Symbols[*symi].terminal,
	_ => false,
     }
  }
  pub fn terminal(&self,s:&str) -> bool
  {
     match self.Symhash.get(s) {
        Some(symi) => self.Symbols[*symi].terminal,
	_ => false,
     }
  }

////// meta (grammar) parser
  pub fn parse_grammar(&mut self, filename:&str)
  {
     let mut reader =  match File::open(filename) {
       Ok(f) => { Some(BufReader::new(f)) },
       _ => { println!("cannot open file, reading from stdin..."); None},
     };//match

     let mut line=String::from("");
     let mut atEOF = false;
     let mut linenum = 0;
     let mut linelen = 0;
     let mut stage = 0;
     let mut multiline = false;  // multi-line mode with ==>, <==
     let mut foundeol = false;
     while !atEOF
     {
       if !multiline {line = String::new();}
       if foundeol { multiline=false;} //use current line
       else {
         let result = if let Some(br)=&mut reader {br.read_line(&mut line)}
                      else {std::io::stdin().read_line(&mut line)};
         match result {
            Ok(0) | Err(_) => { line = String::from("EOF"); },
  	    Ok(n) => {linenum+=1;},
         }//match
       }// did not find line
       
       linelen = line.len();
       
       if multiline && linelen>1 && &line[0..1]!="#" {
          // keep reading until <== found
          if linelen==3 && &line[0..3]=="EOF" {
            panic!("MULTI-LINE GRAMMAR PRODUCTION DID NOT END WITH <==");
          }
          match line.rfind("<==") {
            None => {}, // keep reading, add to line buffer
            Some(eoli) => {
               line.truncate(eoli);
               foundeol = true;
            }
          }//match
       }
       else if linelen>1 && &line[0..1]=="!" {
           self.Extras.push_str(&line[1..]);
       }
       else if linelen>1 && &line[0..1]!="#" {
         let toksplit = line.split_whitespace();
         let stokens:Vec<&str> = toksplit.collect();
         if stokens.len()<1 {continue;}
         match stokens[0] {
            "use" => {
              self.Extras.push_str("use ");
              self.Extras.push_str(stokens[1]);
              self.Extras.push_str("\n");
            },
            "extern" if stokens.len()>2 && stokens[1]=="crate" => {
              self.Extras.push_str("extern crate ");
              self.Extras.push_str(stokens[2]);
              self.Extras.push_str("\n");              
            },
            "!" => {
               let pbi = line.find('!').unwrap();
               self.Extras.push_str(&line[pbi+1..]);
               self.Extras.push_str("\n");                             
            },
            "grammarname" | "grammar_name" => {
               self.name = String::from(stokens[1]);
            },
            "EOF" => {atEOF=true},
            ("terminal" | "terminals") if stage==0 => {
               for i in 1..stokens.len() {
	          let newterm = Gsym::new(stokens[i],true);
                  self.Symhash.insert(stokens[i].to_owned(),self.Symbols.len());
                  self.Symbols.push(newterm);
                  //self.Symbols.insert(stokens[i].to_owned(),newterm);
		  if TRACE>2 {println!("terminal {}",stokens[i]);}
               }
            }, //terminals
	    "typedterminal" if stage==0 => {
	       let mut newterm = Gsym::new(stokens[1],true);
	       if stokens.len()>2 {newterm.settype(stokens[2]);}
               self.Symhash.insert(stokens[1].to_owned(),self.Symbols.len());
               self.Symbols.push(newterm);               
	       //self.Symbols.insert(stokens[1].to_owned(),newterm);
   	       //if TRACE>2 {println!("typedterminal {}:{}",stokens[1],stokens[2]);}
	    }, //typed terminals
	    "nonterminal" if stage==0 => {
	       let mut newterm = Gsym::new(stokens[1],false);
	       if stokens.len()>2 {newterm.settype(stokens[2]);}
               self.Symhash.insert(stokens[1].to_owned(),self.Symbols.len());
               self.Symbols.push(newterm);                              
	    }, //nonterminals
            "nonterminals" if stage==0 => {
               for i in 1..stokens.len() {
	          let newterm = Gsym::new(stokens[i],false);
                  self.Symhash.insert(stokens[i].to_owned(),self.Symbols.len());
                  self.Symbols.push(newterm);
		  if TRACE>2 {println!("nonterminal {}",stokens[i]);}
               }
            },
	    "topsym" | "startsymbol" if stage==0 => {
               match self.Symhash.get(stokens[1]) {
                 Some(tsi) if *tsi<self.Symbols.len() && !self.Symbols[*tsi].terminal => {
              	    self.topsym = String::from(stokens[1]);
                 },
                 _ => { panic!("top symbol {} not found in declared non-terminals; check ordering of declarations, line {}",stokens[1],linenum);
                 },
               }//match
	       //if TRACE>4 {println!("top symbol is {}",stokens[1]);}
	    }, //topsym
            "errsym" | "errorsymbol" => {
               if stage>1 {
                 panic!("!!! Error recover symbol must be declared before production rules, line {}",linenum);
               }
               if stage==0 {stage=1;}
               if !self.terminal(stokens[1]) {
                 panic!("!!!Error recover symbol {} is not a terminal, line {} ",stokens[1],linenum);
               }
               self.Errsym = stokens[1].to_owned();
            },
            /*
            "recover"  => {
               if stage==0 {stage=1;}
               for i in 1..stokens.len()
               {
                  if !self.nonterminal(stokens[i]) {
                     panic!("!!!Error recovery symbol {} is not a declared non-terminal, line {}",stokens[i],linenum);
                  }
                  self.Recover.insert(stokens[i].to_owned());
               } // for each subsequent token
            },
            */
            "resynch" | "resync"  => {
               if stage==0 {stage=1;}
               for i in 1..stokens.len()
               {
                  if !self.terminal(stokens[i]) {
                     panic!("!!!Error recovery re-synchronization symbol {} is not a declared terminal, line {}",stokens[i],linenum);
                  }
                  self.Resynch.insert(stokens[i].trim().to_owned());
               } // for each subsequent token
            },
            "absyntype" | "valuetype" if stage==0 => {
               let pos = line.find(stokens[0]).unwrap() + stokens[0].len();
               self.Absyntype = String::from(line[pos..].trim());
               //self.Absyntype = String::from(stokens[1]);
	       if TRACE>2 {println!("abstract syntax type is {}",stokens[1]);}
            },
            "externtype" | "externaltype" if stage==0 => {
               let pos = line.find(stokens[0]).unwrap() + stokens[0].len();
               self.Externtype = String::from(line[pos..].trim());            
               //self.Externtype = String::from(stokens[1]);
	       if TRACE>2 {println!("external structure type is {}",stokens[1]);}
            },            
	    "left" | "right" if stage<2 => {
               if stage==0 {stage=1;}
               if stokens.len()<3 {continue;}
	       let mut preclevel:i32 = 0;
	       if let Ok(n)=stokens[2].parse::<i32>() {preclevel = n;}
               else {panic!("did not read precedence level on line {}",linenum);}
	       if stokens[0]=="right" && preclevel>0 {preclevel = -1 * preclevel;}
               if let Some(index) = self.Symhash.get(stokens[1]) {
	         //let gsym = self.Symbols.get_mut(index);
	         //if let Some(sym)=gsym { sym.precedence = preclevel; }
                 self.Symbols[*index].precedence = preclevel;
               }
	    }, // precedence and associativity
	    "flexname" | "lexname"  => {
               if stokens.len()<3 {continue;}
               self.Lexnames.insert(stokens[1].to_string(),stokens[2].to_string());
            },
	    LHS if (stokens[1]=="-->" || stokens[1]=="::=" || stokens[1]=="==>") => {
              if !foundeol && stokens[1]=="==>" {multiline=true; continue;}
              else if foundeol {foundeol=false;}
              // println!("RULE {}",&line); 
              if stage<2 {stage=2;}
              
	    // construct lhs symbol
              let symindex = match self.Symhash.get(LHS) {
                Some(smi) if *smi<self.Symbols.len() && !self.Symbols[*smi].terminal => smi,
                _ => {panic!("unrecognized non-terminal symbol {}, line {}",LHS,linenum);},
              };
              let lhsym = &self.Symbols[*symindex]; //.clone();

              // split by | into separate rules

              let pos0 = line.find(stokens[1]).unwrap() + stokens[1].len();
              let mut linec = line[pos0..].to_string();
//              let semaction = if let Some(pos) = line.find('{') {
//                 linec.truncate(pos-pos0);
//                 &line[pos+1..]
//              } else {"}"};
              let barsplit:Vec<_> = linec.split('|').collect();
              
              for rul in &barsplit 
              { //if rul.trim().len()>0 {  // must include empty productions!
              //println!("see rule seg ({})",rul);
              let bstokens:Vec<_> = rul.trim().split_whitespace().collect();
              let mut rhsyms:Vec<Gsym> = Vec::new();
              let mut semaction = "}";
	      let mut i:usize = 0;
              let mut maxprec:i32 = 0;
              let mut seenerrsym = false;
              while i<bstokens.len() {
	        let strtok = bstokens[i];
		i+=1;
                if strtok.len()>0 && &strtok[0..1]=="{" {
                   let position = rul.find('{').unwrap();
                   semaction = rul.split_at(position+1).1;
		   break;
                }
		let toks:Vec<&str> = strtok.split(':').collect();
if TRACE>2&&toks.len()>1 {println!("see labeled token {}",strtok);}		
		match self.Symhash.get(toks[0]) {
		   None => {panic!("unrecognized grammar symbol {}, line {}",toks[0],linenum); },
		   Some(symi) => {
                     let sym = &self.Symbols[*symi];
                     if self.Errsym.len()>0 && &sym.sym == &self.Errsym {
                       if !seenerrsym { seenerrsym = true; }
                       else { panic!("Error symbol {} can only appear once in a production, line {}",&self.Errsym,linenum); }
                     }
                     if !sym.terminal && seenerrsym {
                       panic!("Only terminal symbols may follow the error recovery symbol {}, line {}",&self.Errsym, linenum);
                     }
		     let mut newsym = sym.clone();
		     if toks.len()>1 && toks[1].trim().len()>0
                        { newsym.setlabel(toks[1].trim()); }
                     if maxprec.abs() < newsym.precedence.abs()  {
                        maxprec=newsym.precedence;
                     }
		     rhsyms.push(newsym);
                   }
                }//match
	      } // while there are tokens on rhs
	      // form rule
	      let rule = Grule {
	        lhs : lhsym.clone(),
		rhs : rhsyms,
		action: semaction.to_owned(),
		precedence : maxprec,
	      };
	      if TRACE>2 {printrule(&rule);}
	      self.Rules.push(rule);
            //} 
            } // for rul
            }, 
            _ => {panic!("error parsing grammar on line {}, grammar stage {}",linenum,stage);},  
         }//match first word
       }// not an empty or comment line
     } // while !atEOF
     if self.Symhash.contains_key("START") || self.Symhash.contains_key("EOF")
     {
        panic!("Error in grammar: START and EOF are reserved symbols");
     }
     // add start,eof and starting rule:
     let startnt = Gsym::new("START",false);
     let eofterm =  Gsym::new("EOF",true);
     self.Symhash.insert(String::from("START"),self.Symbols.len());
     self.Symhash.insert(String::from("EOF"),self.Symbols.len()+1);     
     self.Symbols.push(startnt.clone());
     self.Symbols.push(eofterm.clone());
     //self.Symbols.insert(String::from("START"),startnt.clone());
     //self.Symbols.insert(String::from("EOF"),eofterm.clone());
     let topgsym = &self.Symbols[*self.Symhash.get(&self.topsym).unwrap()];
     let startrule = Grule {  // START-->topsym EOF
        lhs:startnt,
        rhs:vec![topgsym.clone()], //,eofterm],  //eofterm is lookahead
        action: String::default(),
        precedence : 0,
//        Ruleaction: |p|{AT::default()}, //{p.Parsestack.pop().unwrap().value},
     };
     self.Rules.push(startrule);  // last rule is start rule
     if TRACE>0 {println!("{} rules in grammar",self.Rules.len());}
     if self.Externtype.len()<1 {self.Externtype = self.Absyntype.clone();} ////***
  }//parse_grammar
}// impl Grammar
// last rule is always start rule and first state is start state


//////////////////////  Nullable

//// also sets the RulesFor map for easy lookup of all the rules for
//// a non-terminal
impl Grammar
{
  pub fn compute_NullableRf(&mut self)
  {
     let mut changed = true;
     let mut rulei:usize = 0;
     while changed 
     {
       changed = false;
       rulei = 0;
       for rule in &self.Rules 
       {
          let mut addornot = true;
          for gs in &rule.rhs 
          {
             if gs.terminal || !self.Nullable.contains(&gs.sym) {addornot=false;}
          } // for each rhs symbol
	  if (addornot) {
             changed = self.Nullable.insert(rule.lhs.sym.clone()) || changed;
             if TRACE>3 {println!("{} added to Nullable",rule.lhs.sym);}
          }
          // add rule index to Rulesfor map:
          if let None = self.Rulesfor.get(&rule.lhs.sym) {
             self.Rulesfor.insert(rule.lhs.sym.clone(),HashSet::new());
          }
          let ruleset = self.Rulesfor.get_mut(&rule.lhs.sym).unwrap();
          ruleset.insert(rulei);
          rulei += 1;
       } // for each rule
     } //while changed
  }//nullable

  // calculate the First set of each non-terminal  (not used- use compute_FirstIM)
// with interior mutability, no need to clone HashSets. // USE THIS ONE!
  pub fn compute_FirstIM(&mut self)
  {
     let mut FIRST:HashMap<String,RefCell<HashSet<String>>> = HashMap::new();
     let mut changed = true;
     while changed 
     {
       changed = false;
       for rule in &self.Rules
       {
         let ref nt = rule.lhs.sym; // left symbol of rule is non-terminal
	 if !FIRST.contains_key(nt) {
            changed = true;
	    FIRST.insert(String::from(nt),RefCell::new(HashSet::new()));
         } // make sure set exists for this non-term
	 let mut Firstnt = FIRST.get(nt).unwrap().borrow_mut();
	 // now look at rhs
	 let mut i = 0;
	 let mut isnullable = true;
 	 while i< rule.rhs.len() && isnullable
         {
            let gs = &rule.rhs[i];
	    if gs.terminal {
	      changed=Firstnt.insert(gs.sym.clone()) || changed;
//if TRACE>2 {println!("{} added to First set of {}",gs.sym,nt);}
              isnullable = false;
            }
            else if &gs.sym!=nt {   // non-terminal
              if let Some(firstgs) = FIRST.get(&gs.sym) {
                  let firstgsb = firstgs.borrow();
                  for sym in firstgsb.iter() {
                    changed=Firstnt.insert(sym.clone())||changed;
                  }
              } // if first set exists for gs
            } // non-terminal 
           if gs.terminal || !self.Nullable.contains(&gs.sym) {isnullable=false;}
	    i += 1;
         } // while loop look at rhs until not nullable
       } // for each rule
     } // while changed
     // Eliminate RefCells and place in self.First
     for nt in FIRST.keys() {
        if let Some(rcell) = FIRST.get(nt) {
          self.First.insert(nt.to_owned(),rcell.take());
        }
     }
  }//compute_FirstIM


  // First set of a sequence of symbols
  pub fn Firstseq(&self, Gs:&[Gsym], la:&str) -> HashSet<String>
  {
     let mut Fseq = HashSet::new();
     let mut i = 0;
     let mut nullable = true;
     while nullable && i<Gs.len() 
     {
         if (Gs[i].terminal) {Fseq.insert(Gs[i].sym.clone()); nullable=false; }
	 else  // Gs[i] is non-terminal
         {
            let firstgsym = self.First.get(&Gs[i].sym).unwrap();
	    for s in firstgsym { Fseq.insert(s.to_owned()); }
	    if !self.Nullable.contains(&Gs[i].sym) {nullable=false;}
         }
	 i += 1;
     }//while
     if nullable {Fseq.insert(la.to_owned());}
     Fseq
  }//FirstSeq

}//impl Grammar continued

