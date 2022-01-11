//! Rustlr allows the use of any lexical analyzer (tokenizer) that satisfies
//! the [Tokenizer] trait.  However, a basic tokenizer, [StrTokenizer] is
//! provided that suffices for many examples.  This tokenizer is not
//! maximally efficient (not single-pass) as it uses [regex](https://docs.rs/regex/latest/regex/).
//!
//! The main contents of this module are [TerminalToken], [Tokenizer],
//! [RawToken], [StrTokenizer] and [LexSource].
//! For backwards compatibility with Rustlr version 0.1, [Lexer], [Lextoken]
//! and [charlexer] are retained, for now.

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(unused_doc_comments)]
#![allow(unused_imports)]
use std::str::Chars;
use regex::Regex;
use std::collections::{HashSet};
use crate::RawToken::*;

/// This structure is expected to be returned by the lexical analyzer ([Lexer] objects).
/// Furthermore, the .sym field of a Lextoken *must* match the name of a terminal
/// symbol specified in the grammar that defines the language.  AT is the type of the
/// *value* attached to the token, which is usually some enum that distinguishes between
/// numbers, keywords, alphanumeric symbols and other symbols.  See the [tutorial and examples](<https://cs.hofstra.edu/~cscccl/rustlr_project>)
/// on how to define the right kind of AT type.

pub struct Lextoken<AT:Default> // now separated from Gsym
{
   pub sym: String, // must correspond to terminal symbol
   pub value: AT,         // value of terminal symbol, if any
}
impl<AT:Default> Lextoken<AT>
{
  /// creates a new Lextoken
  pub fn new(name:String, val:AT) -> Lextoken<AT>   
  {
     Lextoken {
       sym : name,
       value : val,
     }
  }//new Lextoken
}//impl Lextoken

/// This trait defines the interace that any lexical analyzer must be adopted
/// to.  The default implementations for linenum, column and
/// current_line *should be replaced.* They're provided only for compatibility.
pub trait Lexer<AT:Default>
{
  /// retrieves the next Lextoken, or None at end-of-stream. 
  fn nextsym(&mut self) -> Option<Lextoken<AT>>;
  /// returns the current line number.  The default implementation
  /// returns 0.
  fn linenum(&self) -> usize { 0 } // line number
  /// returns the current column (character position) on the current line.
  /// The default implementation returns 0;
  fn column(&self) -> usize { 0 }
  /// returns the current line being tokenized.  The
  /// default implementation returns the empty string.
  fn current_line(&self) -> &str  { "" }
/*  
  /// function that modifies a Lextoken
  /// For example, some symbols such as {, } and |
  /// are reserved and cannot be used for terminal symbols.  Lextokens
  /// containing them have to be modified.  The default implementation
  /// returns the given token unmodified.  Note that this function is
  /// **not** called automatically by [crate::RuntimeParser::parse], and it is
  /// up to the implementor of [Lexer::nextsym] to call it.
  fn modify(t:Lextoken<AT>)->Lextoken<AT> { t }

  /// this function takes a functional argument intended to change the
  /// [Lexer::modify] function.  The default implementation does nothing.
  fn set_modify(&mut self,fn(Lextoken<AT>)->Lextoken<AT>) {}
*/  
}//trait Lexer


/// This is a sample Lexer implementation designed to return every character in a
/// string as a separate token, and is used in small grammars for testing and
/// illustration purposes.  It is assumed that the characters read are defined as
/// terminal symbols in the grammar.
pub struct charlexer<'t>
{
   chars: Chars<'t>,
   index: usize,
   len: usize,
   line:usize,
   keep_ws: bool,  // keep whitespace chars
   /// function to modify char returned by nextsym, can be changed.
   /// Both [charlexer::make] and [charlexer::new] sets this function
   /// initially to `|x|{x.to_string()}`.  For example, some characters such
   /// as '{' and '}' cannot be used as terminal symbols of a grammar and must
   /// be translated into something like "LBRACE" and "RBRACE"
   pub modify: fn(char)->String, 
}
impl<'t> charlexer<'t>
{
  /// creates a charlexer that emits only non-whitespace chars
  pub fn new<'u:'t>(input:&'u str) -> charlexer<'u>
  { charlexer {chars:input.chars(), index:0, len:input.len(), line:1, keep_ws:false, modify: |x|{x.to_string()}} }
  /// creates a charlexer with the option of keeping whitespace chars if kws=true
  pub fn make<'u:'t>(input:&'u str, kws:bool) -> charlexer<'u>
  { charlexer {chars:input.chars(), index:0, len:input.len(), line:1, keep_ws:kws, modify:|x|{x.to_string()}} } 
}
impl<'t, AT:Default> Lexer<AT> for charlexer<'t>
{
   fn nextsym(&mut self) -> Option<Lextoken<AT>>
   {
      let mut res = None;
      let mut stop = false;
      while !stop && self.index<self.len
      {
       let nc = self.chars.next();
       res=match nc { //self.chars.next() {
        None => {stop=true; None},
        Some(c) => {
          self.index+=1;
          if c=='\n' {self.line+=1;}
          if c.is_whitespace() && !self.keep_ws {None}
          else {
            stop=true;
            let mc = (self.modify)(c);
            Some(Lextoken::new(mc,AT::default()))}
        },
       }//match
      }//while
      if (self.index<=self.len) {res} else {None}
   }//nextsym
   /// returns current line number starting from 1
   fn linenum(&self) -> usize { self.line }
   /// returns the index of the current char, starting from 1
   fn column(&self) -> usize { self.index }
   /// returns slice of underlying data using [std::str::Chars::as_str]
   fn current_line(&self) -> &str
   { 
     self.chars.as_str()
   }   
}//impl Lexer for lexer




//////////////////////////////////////////////////////////////////////
//////////////////////// new stuff, needs regex
//////////////////////////////////////////////////////////////////////

/// This the token type required by Rustlr while parsing.  A TerminalToken must correspond
/// to a terminal symbol of the grammar being parsed.  The **sym** field of
/// the struct must correspond to the name of the terminal as defined by the
/// grammar and the **value** must be of type AT, which the is abstract syntax
/// type (*absyntype*) of the grammar.
/// It also includes the starting line and column positions of the token.
///
/// This struct is intended to replace Lextoken, and does not use owned strings.
/// Current this structure lives side-by side with Lextoken for compatibility.
pub struct TerminalToken<'t,AT:Default>
{
  pub sym: &'t str,
  pub value: AT,
  pub line:usize,
  pub column:usize,
}
impl<'t,AT:Default> TerminalToken<'t,AT>
{
  ///creates new lexical token with sym s, value v, line ln and column cl
  pub fn new(s:&'t str, v:AT, ln:usize, cl:usize) -> TerminalToken<'t,AT>
  { TerminalToken{sym:s, value:v, line:ln, column:cl} }
  /// transfers lexical information (line/column) to new TerminalToken
  pub fn transfer(&self, s:&'t str, v:AT) -> TerminalToken<'t,AT>
  {  TerminalToken{sym:s, value:v, line:self.line, column:self.column} }
  /// transfers lexical information from a (RawToken,line,column) triple
  /// returned by [StrTokenizer::next_token] to a new TerminalToken with
  /// sym s and value v.
  pub fn from_raw(rt:(RawToken<'t>,usize,usize),s:&'t str,v:AT) -> TerminalToken<'t,AT>
  { TerminalToken{sym:s, value:v, line:rt.1, column:rt.2} }
}

///////////
/// This trait is intended to replace Lexer, and won't use owned strings
pub trait Tokenizer<'t,AT:Default>
{
  /// retrieves the next [TerminalToken], or None at end-of-stream. 
  fn nextsym(&mut self) -> Option<TerminalToken<'t,AT>>;
  /// returns the current line number.  The default implementation
  /// returns 0.
  fn linenum(&self) -> usize { 0 } // line number
  /// returns the current column (character position) on the current line.
  /// The default implementation returns 0;
  fn column(&self) -> usize { 0 }
  /// returns the absolute character position of the tokenizer.  The
  /// default implementation returns 0;
  fn position(&self) -> usize { 0 }
  /// returns the current line being tokenized.  The
  /// default implementation returns the empty string.
  fn current_line(&self) -> &str  { "" }
  /// returns next [TerminalToken].  This provided function calls nextsym but
  /// will return a TerminalToken with sym="EOF" at end of stream, with
  /// value=AT::default()
  fn next_tt(&mut self) -> TerminalToken<'t,AT>
  {
    match self.nextsym() {
      Some(tok) => tok,
      None => TerminalToken::new("EOF",AT::default(),self.linenum(),self.column()),
    }//match
  }//next_tt
}// Trait Tokenizer

///////////////// Basic Tokenizer

/// structure produced by [StrTokenizer].  [TerminalToken]s must be
/// created from RawTokens (in the [Tokenizer::nextsym] function)
/// once the grammar's terminal symbols and abstract syntax type are known.
#[derive(Debug)]
pub enum RawToken<'t>
{
  /// an unsigned integer, though for convenience it is interpreted as
  /// a signed number.  Negative numbers must be recognized by higher-level
  /// parser.  Both decimal and hexadecimal numbers prefixed by 0x are
  /// recognized.
  Num(i64),
//  Hex(u64),
  /// floating point number
  Float(f64),
  /// single character inside single quotes.
  Char(char), 
  /// String literal, allows for nested quotes
  Strlit(&'t str),
  /// Alphanumeric sequence, staring with an alphabetical character or '_',
  /// and followed by arbitrary numbers of alphabetical, numeric or _.
  Alphanum(&'t str),
  /// non-alphanumeric characters, either identified as doubles, singles, or
  /// unrecognized sequences.
  Symbol(&'t str),
  /// newline, returned optionally
  Newline,
  /// number of consecutive whitespaces, returned optionally
  Whitespace(usize), // counts number of non-newline whitespaces
  /// usually used to represent comments, if returned optionally
  Verbatim(&'t str),
  /// tokenizer error
  LexError,
}//RawToken

/// Generic str tokenizer that produces [RawToken]s.  This tokenizer uses
/// [regex](https://docs.rs/regex/latest/regex), although no always.  For
/// example, to allow for string literals that contains escaped quotations,
/// a direct loop is implemented.
/// The tokenizer gives the option of returning newlines, whitespaces (with
/// count) and comments are special tokens.  It recognizes mult-line
/// string literals, multi-line as well as single-line comments, and returns
/// the starting line and column positions of each token.
///
///Example:
///```ignore
///  let mut scanner = StrTokenizer::from_str("while (1) fork();//run at your own risk");
///  scanner.set_line_comment("//");
///  scanner.keep_comment=true;
///  scanner.add_single(';'); // separates ; from following symbols
///  while let Some(token) = scanner.next() {
///     println!("Token,line,column: {:?}",&token);
///  }
///```
/// this code produces output
///```ignore
///  Token,line,column: (Alphanum("while"), 1, 1)
///  Token,line,column: (Symbol("("), 1, 7)
///  Token,line,column: (Num(1), 1, 8)
///  Token,line,column: (Symbol(")"), 1, 9)
///  Token,line,column: (Alphanum("fork"), 1, 11)
///  Token,line,column: (Symbol("("), 1, 15)
///  Token,line,column: (Symbol(")"), 1, 16)
///  Token,line,column: (Symbol(";"), 1, 17)
///  Token,line,column: (Verbatim("//run at your own risk"), 1, 18)
///```

pub struct StrTokenizer<'t>
{
   decuint:Regex,
   hexnum:Regex,
   floatp:Regex,
   //strlit:Regex,
   alphan:Regex,
   nonalph:Regex,
   doubles:HashSet<&'t str>,   
   singles:HashSet<char>,
   //other_syms: Vec<&'t str>,
   input: &'t str,
   position: usize,
   /// flag to toggle whether whitespaces should be returned as Whitespace tokens,
   /// default is false.
   pub keep_whitespace:bool,
   /// flag to toggle whether newline characters ('\n') are returned as Newline
   /// tokens. Default is false.  Note that if this flag is set to true then
   /// newline characters are treated differently from other whitespaces.
   /// For example, when parsing languages like Python, both keep_whitespace
   /// and keep_newline should be set to true.  
   pub keep_newline:bool,
   line:usize,
   line_comment:&'t str,
   ml_comment_start:&'t str,
   ml_comment_end:&'t str,
   /// flag to determine if comments are kept and returned as Verbatim tokens,
   /// default is false.
   pub keep_comment:bool,
   line_start:usize, // keep starting position of line, for column info
}
impl<'t> StrTokenizer<'t>
{
  /// creats a new tokenizer with defaults, *does not* set input.
  pub fn new() -> StrTokenizer<'t>
  {
    let decuint = Regex::new(r"^\d+").unwrap();
    let hexnum = Regex::new(r"^0x[\dABCDEFabcdef]+").unwrap();
    let floatp = Regex::new(r"^\d*\x2E\d+").unwrap();
    //let strlit = Regex::new(r"^\x22(?s)(.*?)\x22").unwrap();
    let alphan = Regex::new(r"^[_a-zA-Z][_\da-zA-Z]*").unwrap();
    let nonalph=Regex::new(r"^[!@#$%\^&*\?\-\+\*/\.,<>=~`';:\|\\]+").unwrap();
    let mut doubles = HashSet::with_capacity(16);    
    let mut singles = HashSet::with_capacity(16);
    for c in ['(',')','[',']','{','}'] {singles.insert(c);}
    //let mut other_syms = Vec::with_capacity(32);
    let input = "";
    let position = 0;
    let keep_whitespace=false;
    let keep_newline=false;
    let line = 1;
    let line_comment = "//";
    let ml_comment_start="/*";
    let ml_comment_end="*/";    
    let keep_comment=false;
    let line_start=0;
    StrTokenizer{decuint,hexnum,floatp,/*strlit,*/alphan,nonalph,doubles,singles,input,position,keep_whitespace,keep_newline,line,line_comment,ml_comment_start,ml_comment_end,keep_comment,line_start}
  }// new
  /// adds a symbol of exactly length two. If the length is not two the function
  /// has no effect.  Note that these symbols override all other types except for
  /// leading whitespaces and comments markers, e.g. "//" will have precedence
  /// over "/" and "==" will have precedence over "=".
  pub fn add_double(&mut self, s:&'t str)
  {
    if s.len()==2 { self.doubles.insert(s); }
  }
  /// add a single-character symbol.  The type of the symbol overrides other
  /// types except for whitespaces, comments and double-character symbols.
  pub fn add_single(&mut self, c:char) { self.singles.insert(c);}
  /*
  /// add symbol of length greater than two. Symbols that are prefixes of
  /// other symbols should be added after the longer symbols.
  pub fn add_symbol(&mut self, s:&'t str) {
    if s.len()>2 {self.other_syms.push(s); }
  }
  */
  /// sets the input str to be parsed, resets position information.  Note:
  /// trailing whitespaces are always trimmed from the input.
  pub fn set_input(&mut self, inp:&'t str)
  {
    self.input=inp.trim_end(); self.position=0; self.line=1; self.line_start=0;
  }
  /// sets the symbol that begins a single-line comment. The default is
  /// "//".  If this is set to the empty string then no line-comments are
  /// recognized.
  pub fn set_line_comment(&mut self,cm:&'t str) {
    self.line_comment=cm;
  }
  /// sets the symbols used to delineate multi-line comments using a
  /// whitespace separated string such as "/* */".  These symbols are
  /// also the default.  Set this to the empty string to disable
  /// multi-line comments.
  pub fn set_multiline_comments(&mut self,cm:&'t str)
  {
    if cm.len()==0 {
      self.ml_comment_start=""; self.ml_comment_end=""; return;
    }
    let split:Vec<_> = cm.split_whitespace().collect();
    if split.len()!=2 {return;}
    self.ml_comment_start = split[0].trim();
    self.ml_comment_end = split[1].trim();
  }
  /// the current line that the tokenizer is on
  pub fn line(&self)->usize {self.line}
  /// the current column of the tokenizer
  pub fn column(&self)->usize {self.position-self.line_start+1}
  /// returns the current absolute byte position of the Tokenizer
  pub fn position(&self)-> usize {self.position}


  /// returns next token, along with starting line and column numbers.
  /// This function will return None at end of stream or LexError along
  /// with a message printed to stderr if a tokenizer error occured.
  pub fn next_token(&mut self) -> Option<(RawToken<'t>,usize,usize)>
  {
   let mut pi = 0;
   let clen = self.line_comment.len();
   let (cms,cme) = (self.ml_comment_start,self.ml_comment_end);
   while self.position<self.input.len()
   {
    pi = self.position;
    //if pi>=self.input.len() {return None;}
    let mut column0 = self.column();
    let mut line0 = self.line;
    let mut lstart0 = self.line_start;
    
    // skip/keep whitespaces
    let mut nextchars = self.input[pi..].chars();
    let mut c = nextchars.next().unwrap();
    //println!("NEXTCHAR is ({}), position {}",c,self.position);
    let mut i = pi;
    while c.is_whitespace() && i < self.input.len() 
    {
       if c=='\n' {
         self.line+=1; lstart0=self.line_start; self.line_start=i+1; line0=self.line;
         if self.keep_newline { self.position = i+1; return Some((Newline,self.line-1,pi-lstart0+1)); }
       }
       i+= 1; 
       if i<self.input.len() {c = nextchars.next().unwrap();}
    }
    self.position = i;
    if (i>pi && self.keep_whitespace) {
      return Some((Whitespace(i-pi),line0,self.column()-(i-pi)));}
    else if i>pi {continue;}
    //if pi>=self.input.len() {return None;}

    // look for line comment
    if clen>0 && pi+clen<=self.input.len() && self.line_comment==&self.input[pi..pi+clen] {
      if let Some(nlpos) = self.input[pi+clen..].find("\n") {
        self.position = nlpos+pi+clen;
        if self.keep_comment {
          return Some((Verbatim(&self.input[pi..pi+clen+nlpos]),self.line,pi-self.line_start+1));
        }
        else {continue;}
      } else { // no newline fould
        self.position = self.input.len(); 
        if self.keep_comment {return Some((Verbatim(&self.input[pi..]),self.line,pi-self.line_start+1));}
        else {break;}
      }
    }// line comment

    // look for multi-line comment (similar to string literals)
    if cms.len()>0 && pi+cms.len()<=self.input.len() && &self.input[pi..pi+cms.len()] == cms {
       if let Some(endpos) = self.input[pi+cms.len()..].find(cme) {
         self.position = pi+cms.len()+endpos+cme.len();
       } else {
         self.position = self.input.len();
         eprintln!("Tokenizer error: unclosed multi-line comment starting on line {}, column {}",line0,pi-self.line_start+1);
         return Some((LexError,line0,pi-self.line_start+1));
       }
       // find newline chars
       let mut ci = pi;
       while let Some(nli) = self.input[ci..self.position].find('\n')
       {
          self.line+=1; ci += nli+1;  self.line_start=ci;
          // Newline token is never returned if inside string literal
       }
       if self.keep_comment {
         return Some((Verbatim(&self.input[pi..self.position]),line0,pi-lstart0+1));
       }
       else {continue;}
    }//multi-line comments


    // look for doubles
    if pi+1<self.input.len() && self.doubles.contains(&self.input[pi..pi+2]) {
      self.position = pi+2;
      return Some((Symbol(&self.input[pi..pi+2]),self.line,self.column()-2));
    }

    // look for singles:
    //c=self.input[pi..pi+1].chars().next().unwrap();
    if self.singles.contains(&c) {
     // println!("ADDING SINGLE {}",c);
      self.position=pi+1;
      return Some((Symbol(&self.input[pi..pi+1]),self.line,self.column()-1));
    }

    // look for char literal
    if c=='\'' && pi+2<self.input.len() && &self.input[pi+2..pi+3]=="\'" {
      self.position = pi+3;
      let thechar = self.input[pi+1..pi+2].chars().next().unwrap();
      return Some((Char(thechar),self.line,self.column()-3));
    }

    // look for string literal, keep track of newlines
    if c=='\"' {
      let mut ci = pi+1;
      while ci<self.input.len()
      {
         if &self.input[ci..ci+1]=="\"" {
            self.position = ci+1;
            return Some((Strlit(&self.input[pi..self.position]),line0,pi-lstart0+1));
         }
         else if &self.input[ci..ci+1] == "\n" {
           self.line+=1; self.line_start=ci+1;
         }
         // else need to try again!
         else if &self.input[ci..ci+1] == "\\" {ci+=1}; // extra skip
         ci+=1;
      }// while ci < input.len()
      // terminated without finding end of string
      self.position = self.input.len();
        eprintln!("Tokenizer error: unclosed string starting on line {}, column {}",line0,pi-self.line_start+1);
        return Some((LexError,line0,pi-lstart0+1)); 
    }//strlit
    /*
    if let Some(mat) = self.strlit.find(&self.input[pi..]) {
       self.position = mat.end()+pi;
       // find newline chars
       let mut ci = pi;
       while let Some(nli) = self.input[ci..self.position].find('\n')
       {
          self.line+=1; ci += nli+1;  self.line_start=ci;
          // Newline token is never returned if inside string literal
       }
       return Some((Strlit(&self.input[pi..self.position]),line0,pi-lstart0+1));
    }//string lits are matched first, so other's aren't part of strings
    */
    
    // look for hex
    if let Some(mat) = self.hexnum.find(&self.input[pi..]) {
        self.position = mat.end()+pi;
        return Some((Num(i64::from_str_radix(&self.input[pi+2..self.position],16).unwrap()),self.line,pi+3-self.line_start));        
    }//hexnum
    // look for alphanum    
    if let Some(mat) = self.alphan.find(&self.input[pi..]) {
        self.position = mat.end()+pi;  
        return Some((Alphanum(&self.input[pi..self.position]),self.line,pi-self.line_start+1));
    }//alphanums
    // decimal ints
    if let Some(mat) = self.decuint.find(&self.input[pi..]) {
        self.position = mat.end()+pi;  
        return Some((Num(self.input[pi..self.position].parse::<i64>().unwrap()),self.line,pi-self.line_start+1));
    }//decuint
    // floats
    if let Some(mat) = self.floatp.find(&self.input[pi..]) {
        self.position = mat.end()+pi;
        return Some((Float(self.input[pi..self.position].parse::<f64>().unwrap()),self.line,pi-self.line_start+1));
    }//floatp

    //check for unclosed string
    if pi<self.input.len() && &self.input[pi..pi+1]=="\"" {
        self.position = self.input.len();
        eprintln!("Tokenizer error: unclosed string starting on line {}, column {}",line0,pi-self.line_start+1);
        return Some((LexError,line0,pi-self.line_start+1));        
    }//unclosed string
      
    // at this point, what remains must be a recognized, non-alphanumeric symbol
    if let Some(mat) = self.nonalph.find(&self.input[pi..]) {
        self.position = mat.end()+pi;
        return Some((Symbol(&self.input[pi..self.position]),self.line,pi-self.line_start+1));	 
    }

    // at this point, must be error
    self.position = self.input.len();
    if pi<self.position {
      eprintln!("Tokenizer error: unrecognized symbols starting on line {}, column {}",line0,pi-self.line_start+1);
     return Some((LexError,line0,pi-self.line_start+1));
    }
    //else { return None; }
   } //while
   return None;
  }//next_token
  
}//impl StrTokenizer

impl<'t> Iterator for StrTokenizer<'t>
{
  type Item = (RawToken<'t>,usize,usize);
  fn next(&mut self) -> Option<(RawToken<'t>,usize,usize)>
  {
     if let Some(tok) = self.next_token() {Some(tok)} else {None}
  }
}//Iterator

/// Structure to hold contents of a source (such as contents of file).
pub struct LexSource<'t>
{
   pathname:&'t str,
   contents:String,
   id:usize,
}
impl<'t> LexSource<'t>
{
  /// creates a new LexSource struct with given source path,
  /// reads contents into struct using [std::fs::read_to_string]
  pub fn new(path:&'t str) -> std::io::Result<LexSource<'t>>
  {
     let tryread=std::fs::read_to_string(path);
     //println!("READTHIS: {:?}",&tryread);
     match tryread {
       Ok(st) => {
         Ok(LexSource {
           pathname:path,
           id:0,
           contents:st,
         })
       },
       Err(e) => {Err(e)}
     }//match
  }//new
  /// sets the numerical id of this source: can be used in conjunction with
  /// [crate::RuntimeParser::set_src_id]
  pub fn set_id(&mut self, id:usize) {self.id=id;}
  pub fn get_id(&self)->usize {self.id}
  /// retrieves entire contents of lexsource
  pub fn get_contents(&self)->&str {&self.contents}
  /// retrieves original path (such as filename) of this source
  pub fn get_path(&self)->&str {self.pathname}  
}//impl LexSource
impl<'t> StrTokenizer<'t>
{
   /// creates a StrTokenizer from a [LexSource] structure that contains
   /// a string representing the contents of the source, and
   /// calls [StrTokenizer::set_input] to reference that string.
   /// To create a tokenizer that reads from, for example, a file is:
   ///   ```ignore
   ///   let source = LexSource::new(source_path).unwrap();
   ///   let mut tokenizer = StrTokenizer::from_source(&source);
   ///   ```
   pub fn from_source(ls:&'t LexSource<'t>) ->StrTokenizer<'t>
   {
      let mut stk = StrTokenizer::new();
      stk.set_input(ls.contents.as_str());
      stk
   }
   /// creates a string tokenizer and sets input to give str.
   pub fn from_str(s:&'t str) -> StrTokenizer<'t>
   {
      let mut stk = StrTokenizer::new();
      stk.set_input(s);
      stk
   }   
}// impl StrTokenizer

/*
// testing an example
impl<'t> Tokenizer<'t,i64> for StrTokenizer<'t>
{
   fn nextsym(&mut self) -> Option<TerminalToken<'t,i64>>
   {
     let tokopt = self.next_token();
     if let None=tokopt {return None;}
     let tok = tokopt.unwrap();
     match tok.0 {
       Alphanum(s) => Some(TerminalToken::new(s,0,tok.1,tok.2)),
       Num(x) => Some(TerminalToken::from_raw(tok,"num",x)),
       Strlit(s) => Some(TerminalToken::from_raw(tok,"strlit",2)),
       Symbol("@") => Some(TerminalToken::from_raw(tok,"ATSYM",3)),
       Symbol(s) => Some(TerminalToken::from_raw(tok,s,3)),
       _ => Some(TerminalToken::new("EOF",0,0,0)),
     }//match
   }
   fn current_line(&self) -> &str {self.input}
   fn linenum(&self) -> usize {self.line}
   fn position(&self) -> usize {self.position}
}
*/
