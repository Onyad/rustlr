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
use crate::lr_statemachine::*;
use crate::Stateaction::*;

pub const MAXK:usize = 3;

pub struct MLState // emulates LR1/oldlalr engine
{
   index: usize, // index into vector
   items:Itemset,
   lhss: BTreeSet<usize>,  // set of left-side non-terminal indices
   lr0kernel: HashSet<(usize,usize)>, // used only by lalr
   conflicts:Itemset,
   deprecated:Itemset,
   lrkernel : HashSet<LRitem>,
}
impl MLState
{
  pub fn new() -> MLState
  {
     MLState {
        index : 0,   // need to change
        items : HashSet::with_capacity(512),
        lhss: BTreeSet::new(), // for quick lookup
        lr0kernel : HashSet::with_capacity(64),
        conflicts: HashSet::new(),
        deprecated:HashSet::new(),
        lrkernel : HashSet::new(),
     }
  }
  pub fn insert(&mut self, item:LRitem, lhs:usize) -> bool
  {
     let inserted = self.items.insert(item);
     self.lhss.insert(lhs);
     inserted
  }
  pub fn insert_kernel(&mut self, item:LRitem, lhs:usize) -> bool
  {
     let inserted = self.lrkernel.insert(item);
     self.lhss.insert(lhs);
     inserted
  }  
  fn contains(&self, x:&LRitem) -> bool {self.items.contains(x)}
  
  fn hashval(&self) -> usize  // note: NOT UNIQUE
  { 
    let mut key=self.items.len()+ self.lhss.len()*10000;
    let limit = usize::MAX/1000 -1;
    let mut cx = 8;
    for s in &self.lhss {key+=s*1000; cx-=1; if cx==0  || key>=limit {break;}}
    key 
    //self.items.len() + self.lhss.len()*10000
  } //


// new closure method, based on kernels, NO SR/RR CONFLICT DETECTION
// returns false on failure. (?)
  fn close_all(&mut self, Gmr:&mut Grammar, combing:&mut Bimap<usize,Vec<usize>>) -> bool
  {  let mut answer = true;
     // start with kernel items.
     self.items.clear();
     let mut closure = Vec::new();
     let mut closed = 0;
     for item in self.lrkernel.iter() {closure.push(*item);} //copy
     while closed < closure.len()
     {
        let item = closure[closed]; //copy
        //let item = closure.pop().unwrap();
        closed+=1;
        if self.deprecated.contains(&item) {continue;}
        self.items.insert(item); // maybe duplicate
        let (ri,pi,la) = (&item.ri,&item.pi,&item.la);
        if *pi<Gmr.Rules[*ri].rhs.len() && !Gmr.Rules[*ri].rhs[*pi].terminal {
             let lookaheads = &Gmr.Firstseq(&Gmr.Rules[*ri].rhs[pi+1..],*la);
             for rulent in
                 Gmr.Rulesfor.get(&Gmr.Rules[*ri].rhs[*pi].index).unwrap() {
               for lafollow in lookaheads {
                 let newitem = LRitem {
                   ri: *rulent,
                   pi: 0,
                   la: *lafollow,
                 };
                 if !self.deprecated.contains(&newitem) && !self.items.contains(&newitem) {
                  closure.push(newitem);
                 }
               } //for each possible la
             }//for rulent
        }// add new, non-closureitem (if)
        // detect conflicts -- when should this be done?  on the fly
        // add conflict due to conflict propagation -these can't lead to
        // new closure items because they can't be extended
        let mut newconflicts = HashSet::new();
        //let mut oldconflicts = HashSet::new();
        let clas = if pi+1<Gmr.Rules[*ri].rhs.len() {Gmr.Firstseq(&Gmr.Rules[*ri].rhs[pi+1..],*la)} else {HashSet::new()};
//print!("...CHECKING item "); printitem(&item,Gmr); print!("...AGAINST CONFLICTS: ");
        for citem@LRitem{ri:cri,pi:cpi,la:cla} in self.conflicts.iter() {
            if self.deprecated.contains(citem) {continue;}

//printitem(citem,Gmr);

            if *cpi==0 && pi+1==Gmr.Rules[*ri].rhs.len() && Gmr.Rules[*cri].lhs.index==Gmr.Rules[*ri].rhs[*pi].index && cla==la{ //conflict propagation
              newconflicts.insert(item);
//print!("CONFLICT PROPAGATION: ");  printitem(&item,Gmr);
            }
            else if *cpi==0 && pi+1<Gmr.Rules[*ri].rhs.len() && Gmr.Rules[*cri].lhs.index==Gmr.Rules[*ri].rhs[*pi].index && clas.contains(cla) { 
              let nti = Gmr.Rules[*ri].rhs[*pi].index;
              let defaultcomb = vec![nti];
              let comb = combing.get(&nti).unwrap_or(&defaultcomb);
              if comb.len()>MAXK+1 {answer= false;}
             else {

//print!("EXTENSION OF: "); printitem(&item,Gmr);

              // deprecate "shorter" items
              self.deprecated.insert(item);
              for dri in Gmr.Rulesfor.get(&nti).unwrap().iter() {
                for dla in clas.iter() {
                  self.deprecated.insert(LRitem{ri:*dri,pi:0,la:*dla});
                }
              }// deprecate closure
              // why not just remove deprecated items on the fly?

              /////// got to create new symbol, rule, then insert into
              /////// items (closed), and deprecate others.
              // maybe dynamically create new symbol, change rule here...***
              // extend one at a time              
              let eri=Gmr.delay_extend(*ri,*pi,pi+2,combing);
              // this return index of new rule with longer delay
              closure.push(LRitem{ri:eri, pi:*pi, la:*la});
              self.lrkernel.insert(LRitem{ri:eri, pi:*pi, la:*la});
              // reinsert into kernel as it would spawn new items!!!
              // but must redo entire closure!
              //closure.clear();
              //for item in self.lrkernel.iter() {closure.push(*item);} //copy
              //closed = 0;
             } // can extend
            } // conflict extension
        }//for each conflict item
        for nc in newconflicts {self.conflicts.insert(nc);}
     }// while !closed

     answer
  }//close_all

}// impl MLState


impl PartialEq for MLState
{
   fn eq(&self, other:&MLState) -> bool {
      if self.lrkernel.len() != other.lrkernel.len() { return false; }
      for item in self.lrkernel.iter() {
        if !other.lrkernel.contains(item) {return false;}
      }
      true
   }//eq

   fn ne(&self, other:&MLState) ->bool
   {!self.eq(other)}
}
impl Eq for MLState {}

// known-conflict structure:
// (type rr/sr ,rule1, rule2/shift_la)  -> (clearly_resolved, rule# or 0-shift)

pub struct MLStatemachine  // Consumes Grammar
{
   pub Gmr: Grammar,
   pub States: Vec<MLState>, 
   pub Statelookup: HashMap<usize,LookupSet<usize>>,
   pub FSM: Vec<HashMap<usize,Stateaction>>,
   pub Open: Vec<usize>, // for LALR only, vector of unclosed states
   pub known_conflicts:HashMap<(bool,usize,usize),(bool,usize)>,
   pub prev_states : HashMap<usize,HashSet<(usize,usize)>>,
   pub combing: Bimap<usize,Vec<usize>>,
}
impl MLStatemachine
{
  pub fn new(gram:Grammar) -> Self
  { 
       MLStatemachine {
          Gmr: gram,
          States: Vec::with_capacity(8*1024), // reserve 8K states
          Statelookup: HashMap::with_capacity(1024),
          FSM: Vec::with_capacity(8*1024),
          Open: Vec::new(), // not used for lr1, set externally if lalr
          known_conflicts:HashMap::new(),
          prev_states:HashMap::new(), //state --> symbol,state parent
          combing: Bimap::new(), //stores what each nt really represents
       }
  }//new

// calle on fully closed state, produces new state kernels:
  fn simplemakegotos(&mut self,si:usize,agenda:&mut Vec<usize>) //LR1
  {
     // key to following hashmap is the next symbol's index after pi (the dot)
     // the values of the map are the "kernels" of the next state to generate
     let mut newstates:HashMap<usize,MLState> = HashMap::with_capacity(128);
     let mut keyvec:Vec<usize> = Vec::with_capacity(128); //keys of newstates
     let mut newsiconflicts = HashSet::new();
     let mut reagenda = false;
     for item in &self.States[si].items
     {
       if self.States[si].deprecated.contains(item) {continue;}
       let isaccept = (item.ri == self.Gmr.startrulei && item.la==self.Gmr.eoftermi && item.pi>0);
       let rule = self.Gmr.Rules.get(item.ri).unwrap();
       if item.pi<rule.rhs.len() && !isaccept {  // can goto? (dot before end of rule)
          let nextsymi = rule.rhs[item.pi].index;
          if let None = newstates.get(&nextsymi) {
             newstates.insert(nextsymi,MLState::new());
             keyvec.push(nextsymi); // push only if unqiue
          }
          let symstate = newstates.get_mut(&nextsymi).unwrap();
          let newitem = LRitem { // this will be a kernel item in new state
             ri : item.ri,
             pi : item.pi+1,
             la : item.la, 
          };
          //let lhssym = &self.Gmr.Rules[item.ri].lhs.sym;
          let lhssymi = self.Gmr.Rules[item.ri].lhs.index; //*self.Gmr.Symhash.get(lhssym).unwrap();
          symstate.insert_kernel(newitem,lhssymi);
          // SHIFT/GOTONEXT actions to be added by addstate function
       }//can goto
       else // . at end of production, or isaccept - or reduce
       {
          let isconflict;
          if isaccept {
            isconflict =tryadd_action(&mut self.FSM,&self.Gmr,Accept,si,item.la,&mut self.known_conflicts,false);
          }
          else {
            isconflict=tryadd_action(&mut self.FSM, &self.Gmr,Reduce(item.ri),si,item.la,&mut self.known_conflicts,false);
          }
          // if tryadd action returned conflict, insert conflict into
          // state's conflict set
          // but this only allows us to insert a reduce-type conflict no
          // info concerning shift conflict.  but that's OK!
          match &isconflict {
            Some((false,r1,la1)) => { //sr conflict
              newsiconflicts.insert(LRitem{ri:*r1,pi:self.Gmr.Rules[*r1].rhs.len(),la:*la1});
              if self.Gmr.Rules[*r1].rhs.len()==0 {reagenda=true;}
            },
            Some((true,r1,r2)) => { //rr conflict
              newsiconflicts.insert(LRitem{ri:*r1,pi:self.Gmr.Rules[*r1].rhs.len(),la:item.la});
              newsiconflicts.insert(LRitem{ri:*r2,pi:self.Gmr.Rules[*r2].rhs.len(),la:item.la});
              if self.Gmr.Rules[*r1].rhs.len()==0  || self.Gmr.Rules[*r2].rhs.len()==0 {reagenda=true;}              
            },
            _ => {},
          }//match isconflict
          
          // only place addreduce is called
       } // set reduce action
     }// for each item
//println!("AGENDA1: {:?}",&agenda);
// these conflicts need to be backwards propagated
//     if newsiconflicts.len()>0 {agenda.push(si);} // re-agenda
     if reagenda {agenda.push(si);} // re-agenda
//println!("AGENDA: {:?}",&agenda);     
     for ncf in newsiconflicts {
       if self.States[si].deprecated.contains(&ncf) {continue;}
       self.States[si].conflicts.insert(ncf);
     } // insert new conflicts into CURRENT state

     // form closures for all new states and add to self.States list
     for key in keyvec // keyvec must be separate to avoid borrow error
     {
        let mut kernel = newstates.remove(&key).unwrap();
        self.mladdstate(kernel,si,key,agenda); //only place addstate called
        // don't know if this will be new state or previous state
     }
  }//simplemakegotos

// psi is previous state index, nextsym is next symbol
  fn mladdstate(&mut self, mut state:MLState, psi:usize, nextsymi:usize, agenda:&mut Vec<usize>)
  {  let nextsym = &self.Gmr.Symbols[nextsymi].sym;
     let newstateindex = self.States.len(); // index of new state
     state.index = newstateindex;
     let lookupkey =state.hashval();
     if let None=self.Statelookup.get(&lookupkey) {
        self.Statelookup.insert(lookupkey,LookupSet::new());
     }
     let indices = self.Statelookup.get_mut(&lookupkey).unwrap();
     let mut toadd = newstateindex; // defaut is add new state (will push)
     for i in indices.iter()
     {
       if &state==&self.States[*i] {toadd=*i; break; } // state i exists
     }

//     if self.Gmr.tracelev>3 {println!("Transition to state {} from state {}, symbol {}..",toadd,psi,nextsym);}

     // toadd is either a new stateindex or an existing one

     if toadd==newstateindex {  // add new state
       indices.insert(newstateindex); // add to StateLookup index hashset
       self.States.push(state);
       self.FSM.push(HashMap::with_capacity(128)); // always add row to fsm at same time
       let mut prev_set = HashSet::new();
       prev_set.insert((nextsymi,psi));
       self.prev_states.insert(newstateindex,prev_set);
       //if self.lalr {self.Open.push(newstateindex)}  //lalr
       agenda.push(newstateindex);
     }// add new state
     else { // add to prev_states
       self.prev_states.get_mut(&toadd).unwrap().insert((nextsymi,psi));
       // propagate conflicts backwards
       let mut backconfs = HashSet::new();
       for item@LRitem{ri,pi,la} in self.States[toadd].conflicts.iter() {
         if self.States[toadd].deprecated.contains(item) {continue;}
         if *pi>0 && self.Gmr.Rules[*ri].rhs[pi-1].index==nextsymi {
            backconfs.insert(LRitem{ri:*ri,pi:pi-1,la:*la});
         }
       }//for
       let mut bchanged = false;
       for bc in backconfs {bchanged=self.States[psi].conflicts.insert(bc)||bchanged;}
       if bchanged {agenda.push(psi);}   
     }// existing state

     // add to- or change FSM TABLE ...  only Shift or Gotnext added here.
//     let nextsymi = *self.Gmr.Symhash.get(nextsym).expect("GRAMMAR CORRUPTION, UNKOWN SYMBOL");
     let gsymbol = &self.Gmr.Symbols[nextsymi]; //self.Gmr.getsym(nextsym).
     let newaction = if gsymbol.terminal {Stateaction::Shift(toadd)}
        else {Stateaction::Gotonext(toadd)};

     // toadd is index of next state, new or old
     // insert action into FSM

     let isconflict = tryadd_action(&mut self.FSM, &self.Gmr, newaction,psi,nextsymi,&mut self.known_conflicts,false);
     match &isconflict {
            Some((false,r1,la1)) => {
              if self.States[psi].conflicts.insert(LRitem{ri:*r1,pi:self.Gmr.Rules[*r1].rhs.len(),la:*la1}) { agenda.push(psi); }
            },
            /*
            Some((true,r1,r2)) => { // should not be possible
              let res1 = self.States[psi].conflicts.insert(LRitem{ri:*r1,pi:self.Gmr.Rules[*r1].rhs.len(),la:nextsymi});
              let res2 = self.States[psi].conflicts.insert(LRitem{ri:*r2,pi:self.Gmr.Rules[*r2].rhs.len(),la:nextsymi});
              if res1 || res2 {agenda.push(psi);} //need re-closure
            },
            */
            _ => {},
          }//match isconflict
  }  //mladdstate

// may not need anymore
// set reduce/accept actions at the end, starting with startrule
  fn mlset_reduce(&mut self)
  {
     let mut interior:HashSet<usize> = HashSet::new();
     let mut frontier = vec![0]; //start with state (not rule) 0
     while frontier.len()>0
     {
       let si = frontier.pop().unwrap();
       interior.insert(si);
       // expand frontier
       for (_,action) in self.FSM[si].iter() {
         match action {
           Shift(nsi) | Gotonext(nsi) => {
             if !interior.contains(nsi) {frontier.push(*nsi);} 
           },
           _ => {},
         }//match
       } // expand frontier
       // process this item - insert actions
       for item in &self.States[si].items
       {
         let (ri,pi,la) = (item.ri,item.pi,item.la);
         let isaccept = (ri== self.Gmr.startrulei && la==self.Gmr.eoftermi && pi>0);
         if isaccept {
               tryadd_action(&mut self.FSM,&self.Gmr,Accept,si,la,&mut self.known_conflicts,true);  // don't check conflicts here
         }         
         else if pi==self.Gmr.Rules[ri].rhs.len() { //dot at end of rhs
               tryadd_action(&mut self.FSM,&self.Gmr,Reduce(ri),si,la,&mut self.known_conflicts,true);  // check conflicts here
         }//if reduce situation
       } // for each item
     }// while frontier exists
     // eliminate extraneous states
     for si in 0..self.FSM.len() {
       if !interior.contains(&si) { self.FSM[si] = HashMap::new(); }
     }
     println!("LRSD: total reachable states: {}",interior.len());
  }//mlset_reduce



// replaces genfsm procedure
  pub fn selml(&mut self) // algorithm according to paper (k=max delay)
  {
     // modify startrule
     let sri = self.Gmr.startrulei;
     for i in 1..MAXK {
      self.Gmr.Rules[sri].rhs.push(self.Gmr.Symbols[self.Gmr.eoftermi].clone());
     }
     // agenda is a state index, possibly indicating just a kernel

     // construct initial state for START --> . topsym EOF^k, EOF
     // place in agenda
     let mut startstate = MLState::new();
     startstate.insert_kernel( LRitem {
         ri : self.Gmr.startrulei, //self.Gmr.Rules.len()-1, 
         pi : 0,
         la : self.Gmr.eoftermi,
       },self.Gmr.startnti);
     startstate.index = 0;
     self.States.push(startstate); //index always 0
     self.FSM.push(HashMap::with_capacity(128)); // row for state
     self.prev_states.insert(0,HashSet::new());
     let mut agenda = vec![0]; // start with start state
     while agenda.len()>0
     {
        let si:usize = agenda.pop().unwrap();
        // redundant for now:        
        let res=self.States[si].close_all(&mut self.Gmr,&mut self.combing);

        if !res {
          eprintln!("SELECTIVE DELAY TRANSFORMATION FAILED FOR MAX DELAY LENGTH={}, DEFAULTS APPLIED",MAXK);
        }

/*
if si==0 {
     println!("STATE {} ITEMS:",si);
     for item in self.States[si].items.iter() {
       if self.States[si].deprecated.contains(item) {continue;}
       printitem(item,&self.Gmr);
     }
     println!("STATE {} CONFLICTS:",si);
     for item in self.States[si].conflicts.iter() {
       if self.States[si].deprecated.contains(item) {continue;}
       printitem(item,&self.Gmr);
     } 
println!("AGENDA: {:?}",&agenda);
}
*/

        // states closed as they are created? won't reclose!
        let mut removeprevs = HashSet::new();
        for ((symi,psi)) in self.prev_states.get(&si).unwrap().iter() {
          // must remove entry from previous_states!
          let mut newconfs = HashSet::new(); //backwards propagation
          for item@LRitem{ri,pi,la} in self.States[si].conflicts.iter() {
            if self.States[si].deprecated.contains(item) {continue;}
            if *pi>0 && *symi == self.Gmr.Rules[*ri].rhs[pi-1].index {
                 newconfs.insert(LRitem{ri:*ri,pi:pi-1,la:*la});
//println!("backward prop conflict {:?}  sym is {}",&LRitem{ri:*ri,pi:pi-1,la:*la}, &self.Gmr.Symbols[*symi].sym);
            }// symi matches
          } // for each conflict in si
          let mut added = false;
          for nc in newconfs {
            if self.States[*psi].deprecated.contains(&nc) {continue;}
            if self.States[*psi].conflicts.insert(nc) {added=true;}
          }
          if added {agenda.push(*psi); removeprevs.insert((*symi,*psi));}
//if added {println!("State {} added back to agenda",psi);}          
        } // for each conflict and previous state
        // remove oldprevs
        let prevssi = self.prev_states.get_mut(&si).unwrap();
        for rp in removeprevs.iter() {prevssi.remove(rp);}
        // for loop will not run at first for start state

        /* should be redundant. - deprecated being ignored.
        // compute qmax of si - take out deprecated items
        let depitems = self.States[si].deprecated.clone();
        for ditem in depitems.iter() {
           self.States[si].items.remove(ditem);
           self.States[si].lrkernel.remove(ditem);
        }
        */
        // now call makegotos, create kernels of new states, call addstate...
        self.simplemakegotos(si,&mut agenda); //won't detect conflicts
        // create version that does not detect conflicts.  But then
        // when should reduce actions be added?  at the end.

     }//while agenda exists
     self.mlset_reduce();

    if self.Gmr.tracelev>4 {
     // print all rules
     println!("ALL RULES OF TRANSFORMED GRAMMAR");
     for ri in 0..self.Gmr.Rules.len() {
       printrule(&self.Gmr.Rules[ri],ri);
     }
     // print all states
     for state in self.States.iter() {printmlstate(state,&self.Gmr);}
    }

  }//selml

  pub fn to_statemachine(self) -> Statemachine // transfer before writeparser
  {
     Statemachine {
       Gmr: self.Gmr,
       States: Vec::new(),
       Statelookup:HashMap::new(),
       FSM: self.FSM,
       lalr: false,
       Open:Vec::new(),
       sr_conflicts:HashMap::new(),
     }
  }//to_statemachine

}//impl MLStatemachine


//whenever a state's item set has been expanded, we need to put it back
//on the Open list - LR or LALR - to call closure and makegotos again.
// As soon as step "extension" is applied, adding new NT, new rules and
// new item to a state, we should recompute the state's closure  -- bad idea,
// Better to completely compute the conflicts set and deprecate set first before
// calling it a state!   but later it could change.

// try-add returns option<not-clearly resolved conflict>
pub  fn tryadd_action(FSM: &mut Vec<HashMap<usize,Stateaction>>, Gmr:&Grammar, newaction:Stateaction, si:usize, la:usize, known_conflicts:&mut HashMap<(bool,usize,usize),(bool,usize)>, printout:bool) -> Option<(bool,usize,usize)>
  {  let mut answer = None;
     let currentaction = FSM[si].get(&la);
     let mut changefsm = true; // add or keep current
     match (currentaction, &newaction) {
       (None,_) => {},  // most likely: just add
       (Some(Reduce(rsi)), Shift(_)) => {
         if Gmr.tracelev>4 {
           println!("Shift-Reduce Conflict between rule {} and lookahead {} in state {}",rsi,Gmr.symref(la),si);
         }
         let (clr,res) =mlsr_resolve(Gmr,rsi,la,si,known_conflicts,printout);
         if res {changefsm = false; }
         if !clr {answer = Some((false,*rsi,la));}
       },
       (Some(Reduce(cri)),Reduce(nri)) if cri==nri => { changefsm=false; },
       (Some(Reduce(cri)),Reduce(nri)) if cri!=nri => { // RR conflict
         let winner = if (cri<nri) {cri} else {nri};
//         println!("Reduce-Reduce conflict between rules {} and {} resolved in favor of {} ",cri,nri,winner);
//         printrule(&Gmr.Rules[*cri]);
//         printrule(&Gmr.Rules[*nri]);
//         printrulela(*cri,Gmr,la);
//         printrulela(*nri,Gmr,la);
         if winner==cri {changefsm=false;}
         answer = Some((true,*cri,*nri));
       },
       (Some(Accept),_) => { changefsm = false; },
       (Some(Shift(_)), Reduce(rsi)) => {
         if Gmr.tracelev>4 {
           println!("Shift-Reduce Conflict between rule {} and lookahead {} in state {}",rsi,Gmr.symref(la),si);
         }
         // look in state to see which item caused the conflict...
         let (clr,res) = mlsr_resolve(Gmr,rsi,la,si,known_conflicts,printout);
         if !res {changefsm = false; }
         if !clr {answer = Some((false,*rsi,la));}
       },
       _ => {}, // default add newstate
     }// match currentaction
     if changefsm { FSM[si].insert(la,newaction); }
     answer
  }//tryadd_action

  // reslove shift-reduce conflict, returns true if reduce, but defaults
  // to false (shift) so parsing will always continue and terminate.
  // returns (clear,reduce/shift)
fn mlsr_resolve(Gmr:&Grammar, ri:&usize, la:usize, si:usize,known_conflicts:&mut HashMap<(bool,usize,usize),(bool,usize)>,printout:bool) -> (bool,bool)
  {
     if let Some((true,r)) = known_conflicts.get(&(false,*ri,la)) {
        return (true,*r!=0);
     }
     let mut clearly_resolved = true;
     let mut resolution = false; // shift
     let lasym = &Gmr.Symbols[la];
     let lapred = lasym.precedence;
     let rulepred = Gmr.Rules[*ri].precedence;
     if (lapred==rulepred) && lapred<0 {  //<0 means right-associative
       /* default */
     } // right-associative lookahead, return shift
     else
     if (lapred==rulepred) && lapred>0 { // left associative
        resolution = true;
     } // right-associative lookahead, return shift     
     else if (lapred.abs()>rulepred.abs() && rulepred!=0) {/*default*/}
     else if (lapred.abs()<rulepred.abs() /* && lapred!=0 */ ) {
       resolution = true;
       if lapred==0 {
          clearly_resolved = false;
          if printout {
            println!("Shift-Reduce conflict between lookahead {} and rule {} in state {} resolved in favor of Reduce. The lookahead has undeclared precedence",&Gmr.Symbols[la].sym,ri,si);
            printrulela(*ri,Gmr,la);
          }
       }
     } // reduce
     else {
       clearly_resolved=false;
       // report unclear case
       if printout {
         println!("Shift-Reduce conflict between lookahead {} and rule {} in state {} not clearly resolved by precedence and associativity declarations, defaulting to Shift",&Gmr.Symbols[la].sym,ri,si);
         printrulela(*ri,Gmr,la);
       }
     }
     known_conflicts.insert((false,*ri,la),(clearly_resolved,if resolution {1} else {0}));
     (clearly_resolved,resolution)
  }//mlsr_resolve
// no printing until end




//////////////////////////////////////
//////////////////////////////////////
// implemented marked delaying transformations.
impl Grammar
{
  // version that does not change existing rule
  pub fn delay_extend(&mut self,ri:usize,dbegin:usize,dend:usize,combing:&mut Bimap<usize,Vec<usize>>) -> usize
  {
    let mut ntcx = self.ntcxmax+1;
//forget about delay markers.  called with rule number, dbegin and dend
       // check if first symbol at marker is a nonterminal
       let NT1 = &self.Rules[ri].rhs[dbegin];
       //let mut rulei = self.Rules[ri].clone();
       // construct suffix delta to be added to each rule
       let mut delta = Vec::new();
       for i in dbegin+1..dend {
         delta.push(self.Rules[ri].rhs[i].clone());
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

         let mut nttype = String::from("(");
         for i in dbegin .. dend {
           let rsymi = self.Rules[ri].rhs[i].index;
           nttype.push_str(&format!("{},",&self.Symbols[rsymi].rusttype));
         }
         nttype.push(')');
         self.enumhash.insert(nttype.clone(),ntcx); ntcx+=1;
         newnt.rusttype = nttype;
         newnt.index = self.Symbols.len();
         self.Symbols.push(newnt.clone());
         self.Symhash.insert(newntname.clone(),self.Symbols.len()-1);
         // set First, which is no longer called.
         let mut ntfirsts = self.Firstseq(&self.Rules[ri].rhs[dbegin..dend],self.Symbols.len());
         ntfirsts.remove(&self.Symbols.len());
         self.First.insert(newnt.index,ntfirsts);

         // register new combed NT with combing map
         let defvec = vec![self.Rules[ri].rhs[dbegin].index];
         let mut oldvec = combing.get(&self.Rules[ri].rhs[dbegin].index).unwrap_or(&defvec).clone();
         oldvec.push(self.Rules[ri].rhs[dend-1].index);
         combing.insert(newnt.index,oldvec);

         let NTrules:Vec<_> = self.Rulesfor.get(&NT1.index).unwrap().iter().collect();
         let mut rset = HashSet::new(); // rules set for newnt (delayed nt)
         for ntri in NTrules {
           // create new rule
           let mut newrule = Grule::from_lhs(&newnt);
           newrule.rhs = self.Rules[*ntri].rhs.clone();
           for d in &delta { newrule.rhs.push(d.clone()); } //extend

           //////// set semantic action for new rule.
           // need to call/refer action for original rule for NT1
           // need to form a tuple.
           // internal variable symbol.
           let newvar = format!("_delvar_{}_{}_",&newnt.index,dbegin);
// check for return at end of last action.

           let mut actionri = format!(" let {} = {{ {}; ",&newvar,self.Rules[*ntri].action); // retrieves value from original action.
           // need to assign values to new items added to delta
           // they will be popped off of the stack by parser_writer as
           // item2, item1 item0...  because parser writer will write an action
           // for the extended rule. [Mc] --> abc
           
           let mut dtuple = format!("({},",&newvar);
           let mut labi = self.Rules[*ntri].rhs.len(); // original rule rhs len
           for sym in &delta {
             let defaultlabel =format!("_item_del{}_{}_",&labi,ntri);
             let slabel = if sym.label.len()>0 {checkboxlabel(&sym.label)}
               else {
               // set label!
               newrule.rhs[labi].label = defaultlabel.clone();
                &defaultlabel
             };
             dtuple.push_str(&format!("{},",slabel));
             labi+=1;
           }
           actionri.push_str(&format!("{}) }}",&dtuple));  //rparen added here.
           newrule.action = actionri;

           if self.tracelev>1 {
             print!("COMBINED DELAY RULE: ");
             printrule(&newrule,self.Rules.len());
           }

           self.Rules.push(newrule);
           rset.insert(self.Rules.len()-1);
         }// for each rule for this NT1 to be delayed, add suffix
         self.Rulesfor.insert(newnt.index,rset);
       } // newnt is actually a new symbol, else it and its rules exist

       ////// do not change original rule, form a new rule.
       let mut newrulei = Grule::from_lhs(&self.Rules[ri].lhs); //copy
       let mut newrhs = Vec::with_capacity(self.Rules[ri].rhs.len()-1);
       if dbegin>0 {
         for i in 0..dbegin {newrhs.push(self.Rules[ri].rhs[i].clone());}
       }
       let mut clonenewnt = newnt.clone();
       let ntlabel = format!("_delayeditem{}_",dbegin);
       clonenewnt.label = ntlabel.clone();
       newrhs.push(clonenewnt); // newnt added to rule!
       for i in dend .. self.Rules[ri].rhs.len() {
         newrhs.push(self.Rules[ri].rhs[i].clone());
       }

       /////// change semantic action of original rule.
       let mut newaction = String::from(" ");
       // break up tuple
       //let mut labi = 0;
       for i in dbegin..dend {
          let defaultlab = format!("_item{}_",i);
          let symi = &self.Rules[ri].rhs[i]; // original rule
          let labeli = if symi.label.len()>0 {checkboxlabel(&symi.label)}
            else {&defaultlab};
          newaction.push_str(&format!("let mut {} = {}.{}; ",labeli,&ntlabel,i-dbegin));
          //labi+=1;
       }// break up tuple
       // anything to do with the other values?  they have labels, but indexes
       // may be off - but original actions will refer to them as-is.
       newaction.push_str(&self.Rules[ri].action);
       newrulei.rhs = newrhs; // change rhs of rule
       newrulei.action = newaction;

       // special case: newrule becomes startrule if startrule changed.
       if newrulei.lhs.index == self.startnti {
         self.Rules[self.startrulei] = newrulei;
         return self.startrulei;
       } else {      //register new rule
         self.Rulesfor.get_mut(&newrulei.lhs.index).unwrap().insert(self.Rules.len());
         if self.tracelev>1 {
           print!("TRANSFORMED RULE FOR DELAY: ");
           printrule(&newrulei,ri);
         }
         self.Rules.push(newrulei);
         return self.Rules.len()-1;
       }// new rule added (not start rule, which is replaced).
  }// delay_extend



////////////////// don't touch - in use!
  // this must be called before start symbol, eof and startrule added to grammar!
  pub fn delay_transform(&mut self)
  {
    let mut ntcx = self.ntcxmax+1;
    for (ri, delaymarks) in self.delaymarkers.iter() {
     for (dbegin,dend) in delaymarks.iter() {
       // check if first symbol at marker is a nonterminal
       let NT1 = &self.Rules[*ri].rhs[*dbegin];
       if NT1.terminal {
         eprintln!("WARNING: STARTING DELAY MARKER MUST PRECEED NONTERMINAL SYMBOL, RULE {} IN GRAMMAR.  MARKERS IGNORED",ri); continue;
       }// NT1 is non-terminal
       // construct suffix delta to be added to each rule
       let mut delta = Vec::new();
//println!("!!!!dbegin:{}, dend:{}, ri:{}",dbegin,dend,ri);
//printrule(&self.Rules[*ri],*ri);
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

         if self.sametype || !self.genabsyn {newnt.rusttype = self.Absyntype.clone();}  else {
            let mut nttype = String::from("(");
            for i in *dbegin .. *dend {
             let rsymi = self.Rules[*ri].rhs[i].index;
             nttype.push_str(&format!("{},",&self.Symbols[rsymi].rusttype));
            }
            nttype.push(')');
            self.enumhash.insert(nttype.clone(),ntcx); ntcx+=1;
            newnt.rusttype = nttype;
        }// form type of newnt
//println!("newnttype for {} is {}",&newnt.sym, &newnt.rusttype);         
         newnt.index = self.Symbols.len();
         self.Symbols.push(newnt.clone());
         self.Symhash.insert(newntname.clone(),self.Symbols.len()-1);
         // First set computed after delay_transform called.
         // change rules by adding suffix
         let NTrules:Vec<_> = self.Rulesfor.get(&NT1.index).unwrap().iter().collect();
         let mut rset = HashSet::new(); // rules set for newnt (delayed nt)
         for ntri in NTrules {
           // create new rule
           let mut newrule = Grule::from_lhs(&newnt);
           newrule.rhs = self.Rules[*ntri].rhs.clone();
           for d in &delta { newrule.rhs.push(d.clone()); } //extend

           //////// set semantic action for new rule.
           // need to call/refer action for original rule for NT1
           // need to form a tuple.
           // internal variable symbol.
           let newvar = format!("_delvar_{}_{}_",&newnt.index,dbegin);
// check for return at end of last action.

           let mut actionri = format!(" let {} = {{ {}; ",&newvar,self.Rules[*ntri].action); // retrieves value from original action.
           // need to assign values to new items added to delta
           // they will be popped off of the stack by parser_writer as
           // item2, item1 item0...  because parser writer will write an action
           // for the extended rule. [Mc] --> abc
           
           let mut dtuple = format!("({},",&newvar);
           let mut labi = self.Rules[*ntri].rhs.len(); // original rule rhs len
           for sym in &delta {
             let defaultlabel =format!("_item_del{}_{}_",&labi,ntri);
             let slabel = if sym.label.len()>0 {checkboxlabel(&sym.label)}
               else {
               // set label!
               newrule.rhs[labi].label = defaultlabel.clone();
                &defaultlabel
             };
             dtuple.push_str(&format!("{},",slabel));
             labi+=1;
           }
           actionri.push_str(&format!("{}) }}",&dtuple));  //rparen added here.
           newrule.action = actionri;

           if self.tracelev>1 {
             print!("COMBINED DELAY RULE: ");
             printrule(&newrule,self.Rules.len());
           }

           self.Rules.push(newrule);
           rset.insert(self.Rules.len()-1);
         }// for each rule for this NT1 to be delayed, add suffix
         self.Rulesfor.insert(newnt.index,rset);
       } // newnt is actually a new symbol, else it and its rules exist
       // change original rule ri to refer to newnt
       let mut newrhs = Vec::with_capacity(self.Rules[*ri].rhs.len()-1);
       if *dbegin>0 {
         for i in 0..*dbegin {newrhs.push(self.Rules[*ri].rhs[i].clone());}
       }
       let mut clonenewnt = newnt.clone();
       let ntlabel = format!("_delayeditem{}_",dbegin);
       clonenewnt.label = ntlabel.clone();
       newrhs.push(clonenewnt); // newnt added to rule!
       for i in *dend .. self.Rules[*ri].rhs.len() {
         newrhs.push(self.Rules[*ri].rhs[i].clone());
       }

       /////// change semantic action of original rule.
       let mut newaction = String::from(" ");
       // break up tuple
       //let mut labi = 0;
       for i in *dbegin..*dend {
          let defaultlab = format!("_item{}_",i);
          let symi = &self.Rules[*ri].rhs[i]; // original rule
          let labeli = if symi.label.len()>0 {checkboxlabel(&symi.label)}
            else {&defaultlab};
          newaction.push_str(&format!("let mut {} = {}.{}; ",labeli,&ntlabel,i-dbegin));
          //labi+=1;
       }// break up tuple
       // anything to do with the other values?  they have labels, but indexes
       // may be off - but original actions will refer to them as-is.
       newaction.push_str(&self.Rules[*ri].action);
       self.Rules[*ri].rhs = newrhs; // change rhs of rule
       self.Rules[*ri].action = newaction;
       
       if self.tracelev>1 {
         print!("TRANSFORMED RULE FOR DELAY: ");
         printrule(&self.Rules[*ri],*ri);
       }
       
     } // for each pair of delay marks assume dend>dbegin+1
    }//for each rule

  }// delay_transform
} // transformation



//// generic structure bijective hashmap
#[derive(Default,Debug)]
pub struct Bimap<TA:Hash+Default+Eq+Clone, TB:Hash+Default+Eq+Clone>
{
  pub forward: HashMap<TA,TB>,
  pub backward: HashMap<TB,TA>,
}
impl<TA:Hash+Default+Eq+Clone, TB:Hash+Default+Eq+Clone> Bimap<TA,TB>
{
  pub fn new() -> Self {
    Bimap { forward:HashMap::new(), backward:HashMap::new() }
  }
  pub fn with_capacity(cap:usize) -> Self {
    Bimap { forward:HashMap::with_capacity(cap), backward:HashMap::with_capacity(cap) }  
  }
  pub fn insert(&mut self, x:TA,y:TB) -> bool {
    let (x2,y2) = (x.clone(),y.clone());
    let fopt = &self.forward.remove(&x);
    if let Some(y0) = fopt {
      self.backward.remove(y0);
    }
    let bopt = &self.backward.remove(&y);
    if let Some(x0) = bopt {
      self.forward.remove(x0);
    }
    self.forward.insert(x,y);
    self.backward.insert(y2,x2);
    fopt.is_none() && bopt.is_none()
  }//insert
  pub fn get(&self,x:&TA) -> Option<&TB> { self.forward.get(x) }
  pub fn get_mut(&mut self,x:&TA) -> Option<&mut TB> { self.forward.get_mut(x) }
  pub fn rget(&self,x:&TB) -> Option<&TA> { self.backward.get(x) }
  pub fn rget_mut(&mut self,x:&TB) -> Option<&mut TA> { self.backward.get_mut(x) }
  pub fn len(&self)->usize {self.forward.len()}
  pub fn delete(&mut self, x:&TA) -> Option<TB> {
    if let Some(y) = self.forward.remove(x) {
      self.backward.remove(&y);
      Some(y)
    } else {None}
  }
  pub fn rdelete(&mut self, x:&TB) -> Option<TA> {
    if let Some(y) = self.backward.remove(x) {
      self.forward.remove(&y);
      Some(y)
    } else {None}
  }
  pub fn keys(&self) -> std::collections::hash_map::Keys<'_,TA,TB> {
    self.forward.keys()
  }
  pub fn rkeys(&self) -> std::collections::hash_map::Keys<'_,TB,TA> {
    self.backward.keys()
  }
  pub fn iter(&self) -> std::collections::hash_map::Iter<'_,TA,TB> {
    self.forward.iter()
  }
}//impl Bimap
// will be used to map nonterminal symbols to vectors of symbols


fn printitem(item:&LRitem, Gmr:&Grammar)
{
   let lhs = &Gmr.Rules[item.ri].lhs;
   print!("({}) {} --> ",item.ri,&lhs.sym);
   for i in 0..Gmr.Rules[item.ri].rhs.len() {
      if i==item.pi {print!(" . ");}
      print!("{} ",&Gmr.Rules[item.ri].rhs[i].sym);
   }
   if item.pi==Gmr.Rules[item.ri].rhs.len() {print!(" . ");}
   println!(" LA: {}",&Gmr.Symbols[item.la].sym);
}//printitem

// independent function for tracing
pub fn printmlstate(state:&MLState,Gmr:&Grammar) 
{
  println!("-----------\nState {}:",state.index);
  for item@LRitem{ri,pi,la} in state.items.iter() {
    if !state.deprecated.contains(item) {printitem(item,Gmr);}
  }//for item
}//printlalrstate
