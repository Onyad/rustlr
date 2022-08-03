#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(unused_doc_comments)]
#![allow(unused_imports)]
#![allow(non_upper_case_globals)]
//use std::fmt::Display;
//use std::default::Default;
use std::collections::{HashMap,HashSet,BTreeSet};
use std::cell::{RefCell,Ref,RefMut};
use std::hash::{Hash,Hasher};
use std::io::{self,Read,Write,BufReader,BufRead};
use crate::grammar_processor::*;


// implemented marked delaying transformations.
impl Grammar
{
  // this must be called before start symbol, eof and startrule added to grammar!
  pub fn delay_transform(&mut self)
  {
    for (ri, delaymarks) in self.delaymarkers.iter() {
     for (dbegin,dend) in delaymarks.iter() {
       // check if first symbol at marker is a nonterminal
       let NT1 = &self.Rules[*ri].rhs[*dbegin];
       if NT1.terminal {
         panic!("ERROR: STARTING DELAY MARKER MUST PRECEED NONTERMINAL SYMBOL, RULE {} IN GRAMMAR",ri);
       }// NT1 is non-terminal
       // construct suffix delta
       let mut delta = Vec::new();
       for i in dbegin+1..*dend {
         delta.push(self.Rules[*ri].rhs[i].clone());
       }
       // construct new nonterminal name ([Mdelta])
       let mut newntname = format!("NEWDELAYNT_{}",&NT1.sym);
       for s in &delta {newntname.push_str(&format!("_{}",&s.index));}
       // check that no such name already exists
       // construct new nonterminal
       let mut newnt = Gsym::new(&newntname,false);
       if let Some(nti) = self.Symhash.get(&newntname) {
          newnt = self.Symbols[*nti].clone();
       } else { // really new
         newnt.rusttype = self.Absyntype.clone(); // TEMPORARY!
         newnt.index = self.Symbols.len();
         self.Symbols.push(newnt.clone());
         self.Symhash.insert(newntname.clone(),self.Symbols.len()-1);

         let NTrules:Vec<_> = self.Rulesfor.get(&NT1.index).unwrap().iter().collect();
         let mut rset = HashSet::new(); // rules set for newnt (delayed nt)
         for ntri in NTrules {
           // create new rule
           let mut newrule = Grule::from_lhs(&newnt);
           newrule.rhs = self.Rules[*ntri].rhs.clone();
           for d in &delta { newrule.rhs.push(d.clone()); }

//////// NO SEMANTIC ACTION FOR NEW RULE YET!

           if self.tracelev>1 {
             print!("COMBINED DELAY RULE: ");
             printrule(&newrule,self.Rules.len());
           }

           self.Rules.push(newrule);
           rset.insert(self.Rules.len()-1);
         }// for each rule for this NT1 to be delayed, add suffix
         self.Rulesfor.insert(newnt.index,rset);
       } // newnt is actually a new symbol, else it and it's rules exists
       // change original rule ri to refer to newnt
       let mut newrhs = Vec::with_capacity(self.Rules[*ri].rhs.len()-1);
       if *dbegin>0 {
         for i in 0..*dbegin {newrhs.push(self.Rules[*ri].rhs[i].clone());}
       }
       newrhs.push(newnt); // newnt added to rule!
       for i in *dend .. self.Rules[*ri].rhs.len() {
         newrhs.push(self.Rules[*ri].rhs[i].clone());
       }
       self.Rules[*ri].rhs = newrhs; // change rhs of rule
       if self.tracelev>1 {
         print!("TRANSFORMED RULE FOR DELAY: ");
         printrule(&self.Rules[*ri],*ri);
       }
       
     } // for each pair of delay marks assume dend>dbegin+1
    }//for each rule
  }// delay_transform
} // transformation









////////////////////////////////////////////////////////////////////////////
// Experimental module to implement selML(k,1) parsers introduced roughly by
// Bertsch, Nederhof and Schmitz.

// nonterminals consists of a symbol plus a fixed k-size array of symbols.
// symbol unused represents nothing and allows us to use fixed arrays.

// usize is the type of grammar symbols (as an index)
/*
use crate::grammar_processor::*;
use crate::selmlk::GSymbol::*;

//pub struct Nonterminal<const K:usize>(usize,[usize;K]);
#[derive(Copy,Clone,Debug,Hash,Ord,PartialOrd,Eq,PartialEq)]
pub enum GSymbol<const K:usize> {
   Terminal(usize),
   Nonterminal(usize,[usize;K]),
}
impl<const K:usize> GSymbol<K>
{
   fn tostr(&self, Gmr:&Grammar) -> String
   {
      match self {
        Terminal(ti) => Gmr.Symbols[*ti].sym.clone(),
        Nonterminal(ni,D) => {
           let mut s = format!("[{},",&Gmr.Symbols[*ni].sym);
           for ti in D {
             if *ti == Hash {s.push('#');}
             else { s.push_str(&Gmr.Symbols[*ti].sym); s.push(','); }
           }
           s.push(']'); s
        },
      }//match
   }//tostr
}
// a special usize index, perhaps 0 or usize::MAX, will represent a dummy
// filler so we can have fixed size arrays and const generics.

const Hash:usize = usize::MAX;
//const HASH:GSymbol = GSymbol::Terminal(Hash);
//static Hashes<const K:usize> = [Hash;K];

//compile time production
pub struct Production<const K:usize> {
  pub lhs: GSymbol<K>, 
  pub rhs: Vec<GSymbol<K>>,
}

// use these on top of grammar_processor constructs

// semantic values
#[derive(Copy,Clone,Debug)]
pub struct Values<AT:Default, const K:usize>([AT;K]);
*/