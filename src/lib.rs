#![allow(dead_code, unused_imports, unused_variables)]

extern crate byteorder;
extern crate regex_syntax;
extern crate utf8_ranges;

pub use builder::{DFABuilder, MatcherBuilder};
pub use dfa::{DFA, DFAKind};
pub use dfa_ref::DFARef;
pub use error::{Error, ErrorKind};
pub use state_id::StateID;

mod builder;
mod determinize;
mod dfa;
mod dfa_ref;
mod error;
mod matcher;
mod minimize;
mod nfa;
mod sparse;
mod state_id;
