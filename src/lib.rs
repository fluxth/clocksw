use std::alloc::System;

#[global_allocator]
static A: System = System;

pub mod drivers;
pub mod display;
pub mod views;
pub mod utils;
pub mod fonts;
pub mod helpers;
