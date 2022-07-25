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
use std::io::{self,Read,Write,BufReader,BufRead,Result,Error,ErrorKind,SeekFrom};
use std::fs::File;
use std::io::prelude::*;
use crate::{TRACE,Lexer,Lextoken,Stateaction,RuntimeParser,ErrHandler,StandardReporter};


/////////////////////////////////////////////////////
// For new ErrHandler trait and StandardReporter.           version 3
/////////////////////////////////////////////////////

impl StandardReporter
{
 pub fn augment_training(&mut self, filepath:&str) -> std::io::Result<()>
 {
   if self.trained.len()<1 {return Ok(());}
   let fopen = std::fs::OpenOptions::new().write(true).read(true).open(filepath);
   match &fopen   {
     Ok(f) => {},
     _ => {
       return Err(Error::new(ErrorKind::Other,"augmenter can't find file"));
     },
   }//match
   let mut fio = fopen.unwrap();
   let finopen = File::open(filepath);
   if let Err(_) = finopen {
       return Err(Error::new(ErrorKind::Other,"augmenter can't find file"));   
   }
   let mut fin = BufReader::new(finopen.unwrap());
   let mut position:u64 = 0;
   let mut line = String::new();
   fin.read_line(&mut line)?;
   //if line.trim()!="//Parser generated by rustlr" {
   if !line.contains("//Parser generated by rustlr") {
     return Err(Error::new(ErrorKind::Other, "input file was not created by rustlr"));
   }
   //write!(fout,"{}",&line)?;
   let mut stop = false;
   let mut oktoaugment = true;
   while !stop
   {
     line = String::new();
     position = fin.stream_position()?;     
     match fin.read_line(&mut line) {
       Ok(n) if n>0 => {},
       _ => {stop=true; oktoaugment=false;}
     }
     if line.trim().len()>21 && &line.trim()[..21]=="}//end of load_extras" {
       stop=true;
       //       println!("FOUND LINEMARKER!!!!");
     }
   }//while !stop
   //// now augment
   if oktoaugment {
    fio.seek(SeekFrom::Start(position))?;
    for key in self.trained.keys()
    {
     let (state,sym) = key;
     let enter = self.trained.get(key).unwrap().trim();
     write!(fio,"  parser.RSM[{}].insert(\"{}\",Stateaction::Error(\"{}\"));\n",state,sym,enter)?;
    }
   write!(fio,"}}//end of load_extras: don't change this line as it affects augmentation\n")?;
  } //ok to augment
  else {return Err(Error::new(ErrorKind::Other,"given file cannot be augmented"));}
   Ok(())
 }// new augment_training
}
