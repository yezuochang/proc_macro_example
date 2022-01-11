// Until extern crate self does not get into stable (1.34) this crate won't be able to run
// its tests on stable (given that it cannot reference itself as a crate).
//extern crate self as proc_dump_sample;

extern crate macro_crate;
pub use macro_crate::*;
#[cfg(test)]
mod tests;
