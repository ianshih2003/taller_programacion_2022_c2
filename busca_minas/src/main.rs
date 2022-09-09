use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let tablero_dir: &str = &args[1];

    busca_minas::run(tablero_dir)
}
