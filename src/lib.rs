// test時はno_stdを無効に設定
#![cfg_attr(not(test), no_std)]
pub mod types;
pub mod spc_file_parser;
mod spc_assembler;
