// This is a comment, and is ignored by the compiler.
// You can test this code by clicking the "Run" button over there ->
// or if you prefer to use your keyboard, you can use the "Ctrl + Enter"
// shortcut.

mod array_utils;
mod file_utils;
mod fock_space;

use crate::fock_space::Hubbard;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let hubbard_model = Hubbard {
        n_sites: 3,
        t: 1,
        u: 2,
    };
    hubbard_model.get_eigenvalues();
    println!("Time elapsed: {:.2?}", now.elapsed());
}
