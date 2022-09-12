use std::env;
use busca_minas;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Error: Agregue directorio")
    } else {
        let tablero_dir: &str = &args[1];

        busca_minas::run(tablero_dir)
    }
}
