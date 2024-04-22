#![feature(async_closure)]
#![feature(closure_lifetime_binder)]
#![feature(async_fn_in_trait)]

pub mod tests;
pub mod archetype;
pub mod component;
pub mod chunk;
pub mod world;
pub mod entity;
pub mod type_info;
pub mod unknown_component;
pub mod call;
pub mod scheduler;
pub mod system;
