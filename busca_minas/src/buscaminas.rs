#[derive(Debug)]
enum Casilla {
    Bomba,
    Espacio(u8),
}

struct Tablero {
    casillas: Vec<Vec<Casilla>>,
}

impl Tablero {
    fn new(casillas: Vec<Vec<Casilla>>) -> Tablero {
        Tablero {
            casillas
        }
    }
}

use std::fs;

fn leer_archivo(path: &str) -> String {
    fs::read_to_string(path).expect("Error al leer archivo")
}

fn construir_tablero(archivo: &String) -> Tablero {
    let mut vec_tablero: Vec<Vec<Casilla>> = Vec::new();
    let mut fila: Vec<Casilla> = Vec::new();
    for casilla in archivo.as_bytes() {
        match casilla {
            10 => {
                vec_tablero.push(fila);
                fila = Vec::new();
            }
            46 => {
                fila.push(Casilla::Espacio(0));
            }
            42 => {
                fila.push(Casilla::Bomba);
            }
            _ => continue
        }
    }
    vec_tablero.push(fila);

    Tablero::new(vec_tablero)
}

fn cantidad_de_bombas(_pos_fila: usize, _pos_col: usize, _tablero: &Tablero) -> usize {
    let cantidad: usize = 0;
    cantidad
}

fn convertir_a_tablero_resuelto(tablero: &Tablero) {
    for (i, fila) in tablero.casillas.iter().enumerate() {
        for (j, _casilla) in fila.iter().enumerate() {
            println!("fila: {}, columna {}, {:?}", i, j, cantidad_de_bombas(i, j, tablero));
        }
    }
}

pub fn run(path: &str) {
    let archivo = leer_archivo(path);

    let tablero = construir_tablero(&archivo);

    convertir_a_tablero_resuelto(&tablero);
}