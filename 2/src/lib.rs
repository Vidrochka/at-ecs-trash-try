#![feature(closure_lifetime_binder)]
#![feature(map_many_mut)]
#![feature(box_into_inner)]
#![feature(map_try_insert)]
#![feature(trait_upcasting)]

#[cfg(test)]
pub mod tests;

pub mod chunk;
pub mod system;
pub mod entity_id;
pub mod type_info;
pub use chunk::*;