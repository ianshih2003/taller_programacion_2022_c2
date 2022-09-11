pub mod buscaminas;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let tablero_dir: &str = &args[1];

    buscaminas::run(tablero_dir)
}
