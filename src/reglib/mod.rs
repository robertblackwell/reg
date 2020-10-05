/// When you load `module_abc` in main "mod module_abc;" cargo looks for a file module_abc.rs
/// in the same folder than main.rs, if doesn't find one looks for a file module_abc/mod.rs
/// (this one), so here we call the others

pub mod server;
pub mod worker;
pub mod queue;
