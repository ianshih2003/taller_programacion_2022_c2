use busca_minas;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Error: Agregue directorio")
    } else {
        let tablero_dir: &str = &args[1];

        busca_minas::run(tablero_dir)
    }
}
