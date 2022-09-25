use std::env;

const CANT_ARGS_VALIDA: usize = 2;
const CANT_ARGS_SIN_DIRECTORIO: usize = 1;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        CANT_ARGS_VALIDA => {
            let tablero_dir: &str = &args[1];

            match busca_minas::run(tablero_dir) {
                Ok(()) => (),
                Err(e) => println!("{}", e),
            }
        }
        CANT_ARGS_SIN_DIRECTORIO => println!("Error: Agregue directorio"),

        _ => println!("Error: Cantidad de argumentos invalido"),
    }
}
