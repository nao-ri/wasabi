mod ast;
// Export AST types directly under crate, without ast prefix.
pub use crate::ast::*;
mod function_type;

mod error;
// Export error types directly under the crate.
pub use crate::error::*;

pub mod types;

mod extensions;
mod parse;
mod encode;

#[cfg(test)]
mod tests;

// See long comment on Windows 10 allocator performance with parallel parsing in `parse.rs`.
#[cfg(target_os = "windows")]
use mimalloc::MiMalloc;
#[cfg(target_os = "windows")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

/*
 TODO WHEN CONTINUING
 - support multi-value, multi-table, multi-memory because they are anyway pretty much supported (and make for less special cases)
 - make AST blocks nested, remove end/else opcodes
 - remove blocktype, replace with function type (this should make our AST multi-value capable)
 - rename wasm crate to wasabi-wasm
*/
