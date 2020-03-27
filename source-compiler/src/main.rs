use asttoir;
use ir;
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use backend_wasm;

fn main() {
    let contents = asttoir::read_from_file(None);
    println!("{:#?}", contents);
    let func = asttoir::populate_func(contents);
    println!("{:#?}", func);
    /*let ir_program = ir::Program::new();
    ir_program.funcs.push(func);
    ir_program.entry_point
    backend_wasm::run_backend()*/
}
