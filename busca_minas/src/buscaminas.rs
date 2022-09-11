use std::fs;

const NEW_LINE_UTF: u8 = 10;
const BOMBA_UTF: u8 = 42;
const ESPACIO_UTF: u8 = 46;

#[derive(Debug)]
enum Casilla {
    Bomba,
    Espacio,
}

struct Tablero {
    casillas: Vec<Vec<Casilla>>,
    filas: usize,
    columnas: usize,
}

impl Tablero {
    fn new(casillas: Vec<Vec<Casilla>>, filas: usize, columnas: usize) -> Tablero {
        Tablero {
            casillas,
            filas,
            columnas,
        }
    }
}

fn leer_archivo(path: &str) -> String {
    fs::read_to_string(path).expect("Error al leer archivo")
}

fn construir_tablero(archivo: &String) -> Tablero {
    let mut vec_tablero: Vec<Vec<Casilla>> = Vec::new();
    let mut fila: Vec<Casilla> = Vec::new();

    let mut cant_filas: usize = 0;
    let mut cant_col: usize = 0;
    for casilla in archivo.as_bytes() {
        match *casilla {
            NEW_LINE_UTF => {
                vec_tablero.push(fila);
                fila = Vec::new();
                cant_filas += 1;
                cant_col = 0;
                continue;
            }
            BOMBA_UTF => fila.push(Casilla::Bomba),

            ESPACIO_UTF => fila.push(Casilla::Espacio),

            _ => continue,
        }
        cant_col += 1
    }
    cant_filas += 1;
    vec_tablero.push(fila);

    Tablero::new(vec_tablero, cant_filas, cant_col)
}

fn imprimir_tablero_resuelto(tablero: &Tablero) {
    for (i, fila) in tablero.casillas.iter().enumerate() {
        for (j, casilla) in fila.iter().enumerate() {
            match casilla {
                Casilla::Bomba => print!("*"),
                Casilla::Espacio => {
                    let cantidad_de_bombas = cantidad_de_bombas(i, j, tablero);

                    if cantidad_de_bombas == 0 {
                        print!(".")
                    } else {
                        print!("{cantidad_de_bombas}")
                    }
                }
            }
        }
        print!("{}", '\n');
    }
}

fn construir_posiciones_adyacentes(
    pos_fila: usize,
    pos_col: usize,
    tablero: &Tablero,
) -> Vec<(usize, usize)> {
    let mut posiciones: Vec<(usize, usize)> = Vec::new();


    if pos_fila > 0 {
        posiciones.push((pos_fila - 1, pos_col))
    }

    if pos_fila < tablero.filas - 1 {
        posiciones.push((pos_fila + 1, pos_col))
    }

    if pos_col > 0 {
        posiciones.push((pos_fila, pos_col - 1))
    }

    if pos_col < tablero.columnas - 1 {
        posiciones.push((pos_fila, pos_col + 1))
    }

    if pos_fila > 0 && pos_col > 0 {
        posiciones.push((pos_fila - 1, pos_col - 1))
    }

    if pos_fila < tablero.filas - 1 && pos_col < tablero.columnas - 1 {
        posiciones.push((pos_fila + 1, pos_col + 1))
    }

    if pos_fila > 0 && pos_col < tablero.columnas - 1 {
        posiciones.push((pos_fila - 1, pos_col + 1))
    }

    if pos_col > 0 && pos_fila < tablero.filas - 1 {
        posiciones.push((pos_fila + 1, pos_col - 1))
    }

    posiciones
}

fn cantidad_de_bombas(pos_fila: usize, pos_col: usize, tablero: &Tablero) -> usize {
    let mut cantidad: usize = 0;

    let posiciones_ady = construir_posiciones_adyacentes(pos_fila, pos_col, tablero);

    for (i, j) in posiciones_ady {
        match tablero.casillas[i][j] {
            Casilla::Bomba => cantidad += 1,
            _ => continue,
        }
    }
    return cantidad;
}

pub fn run(path: &str) {
    let archivo = leer_archivo(path);

    let tablero = construir_tablero(&archivo);

    imprimir_tablero_resuelto(&tablero);
}
