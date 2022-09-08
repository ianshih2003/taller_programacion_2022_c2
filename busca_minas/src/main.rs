use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Parameter {}", &args[1]);
    let tablero_dir = &args[1];
    let contents = fs::read_to_string(tablero_dir).expect("Error al leer archivo");
    println!("{}", contents);
}
