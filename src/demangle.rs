//
// written by everettjf
// email : everettjf@live.com
// created at 2022-01-02
//
use symbolic_common::Name;
use symbolic_demangle::{Demangle, DemangleOptions};

pub fn demangle_symbol(symbol: &str) -> String {
    let name = Name::from(symbol);
    let result = name.try_demangle(DemangleOptions::complete());
    result.to_string()
}