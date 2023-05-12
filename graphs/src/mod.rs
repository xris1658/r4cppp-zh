#![feature(rustc_private)]

extern crate arena;

mod rc_graph;
mod ref_graph;

fn main() {
    println!("\nRc<RefCell<Node>>:");
    rc_graph::main();
    println!("\n&Node and UnsafeCell:");
    ref_graph::main();
}
