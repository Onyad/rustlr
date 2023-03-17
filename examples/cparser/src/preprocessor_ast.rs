//Abstract syntax types generated by rustlr for grammar preprocessor
    
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(dead_code)]
extern crate rustlr;
use rustlr::LBox;

#[derive(Debug)]
pub enum Stuff<'lt> {
  junk,
  directive(&'lt str),
  Stuff_Nothing,
}
impl<'lt> Default for Stuff<'lt> { fn default()->Self { Stuff::Stuff_Nothing } }

#[derive(Default,Debug)]
pub struct S<'lt>(pub Vec<LBox<Stuff<'lt>>>,);

