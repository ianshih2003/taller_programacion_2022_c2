use std::fs;

const BOMBA_UTF: u8 = 42;
const ESPACIO_UTF: u8 = 46;

/// Posibles valores de una casilla
pub enum Casilla {
    Bomba,
    Espacio,
}

// Representacion del tablero del buscaminas
pub struct Tablero {
    /// Matriz de casillas
    pub casillas: Vec<Vec<Casilla>>,

    /// Cantidad de filas
    pub filas: usize,

    /// Cantidad de columnas
    pub columnas: usize,
}

impl Tablero {
    /// Construye un tablero
    fn new(casillas: Vec<Vec<Casilla>>, filas: usize, columnas: usize) -> Tablero {
        Tablero {
            casillas,
            filas,
            columnas,
        }
    }

    /// Construye un tablero dado un string que lo representa, donde . = Casilla y * = Bomba.
    pub fn crear_de_string(archivo: &String) -> Tablero {
        let mut vec_tablero: Vec<Vec<Casilla>> = Vec::new();
        let mut casillas: Vec<Casilla> = Vec::new();


        let filas = archivo.split("\n");

        let mut cant_filas: usize = 0;
        let mut cant_col: usize = 0;

        for fila in filas {
            for casilla in fila.as_bytes() {
                match *casilla {
                    BOMBA_UTF => casillas.push(Casilla::Bomba),
                    ESPACIO_UTF => casillas.push(Casilla::Espacio),
                    _ => continue,
                }
            }
            cant_filas += 1;
            cant_col = fila.len();
            vec_tablero.push(casillas);
            casillas = Vec::new()
        }

        vec_tablero.push(casillas);

        Tablero::new(vec_tablero, cant_filas, cant_col)
    }

    /// Imprime el tablero, reemplazando las casillas por la cantidad de bombas cercanas
    /// y devuelve la representacion en String
    pub fn imprimir_tablero_resuelto(&self) -> String {
        let mut resultado: String = String::new();
        for (i, fila) in self.casillas.iter().enumerate() {
            for (j, casilla) in fila.iter().enumerate() {
                match casilla {
                    Casilla::Bomba => resultado += "*",
                    Casilla::Espacio => {
                        let cant = self.cantidad_de_bombas(i, j);

                        if cant == 0 {
                            resultado += "."
                        } else {
                            resultado += format!("{cant}").as_str();
                        }
                    }
                }
            }
            if i < self.filas {
                resultado += "\n";
            }
        }
        print!("{}", resultado);
        resultado
    }

    /// Devuelve las posiciones adyacentes de una posicion del tablero
    fn construir_posiciones_adyacentes(
        &self,
        pos_fila: usize,
        pos_col: usize,
    ) -> Vec<(usize, usize)> {
        let mut posiciones: Vec<(usize, usize)> = Vec::new();

        if pos_fila > 0 {
            posiciones.push((pos_fila - 1, pos_col))
        }

        if pos_fila < self.filas - 1 {
            posiciones.push((pos_fila + 1, pos_col))
        }

        if pos_col > 0 {
            posiciones.push((pos_fila, pos_col - 1))
        }

        if pos_col < self.columnas - 1 {
            posiciones.push((pos_fila, pos_col + 1))
        }

        if pos_fila > 0 && pos_col > 0 {
            posiciones.push((pos_fila - 1, pos_col - 1))
        }

        if pos_fila < self.filas - 1 && pos_col < self.columnas - 1 {
            posiciones.push((pos_fila + 1, pos_col + 1))
        }

        if pos_fila > 0 && pos_col < self.columnas - 1 {
            posiciones.push((pos_fila - 1, pos_col + 1))
        }

        if pos_col > 0 && pos_fila < self.filas - 1 {
            posiciones.push((pos_fila + 1, pos_col - 1))
        }

        posiciones
    }

    /// Devuelve la cantidad de bombas cercanas de una posicion del tablero
    pub fn cantidad_de_bombas(&self, pos_fila: usize, pos_col: usize) -> usize {
        let mut cantidad: usize = 0;

        let posiciones_ady = self.construir_posiciones_adyacentes(pos_fila, pos_col);

        for (i, j) in posiciones_ady {
            match self.casillas[i][j] {
                Casilla::Bomba => cantidad += 1,
                _ => continue,
            }
        }
        return cantidad;
    }
}


fn leer_archivo(path: &str) -> String {
    fs::read_to_string(path).expect("Error al leer archivo")
}

/// Recibe directorio donde se encuentra el archivo que representa el tablero
/// Imprime un tablero resuelto
pub fn run(path: &str) {
    let archivo = leer_archivo(path);

    let tablero = Tablero::crear_de_string(&archivo);

    tablero.imprimir_tablero_resuelto();
}

#[cfg(test)]
mod tests {
    use crate::Tablero;

    fn construir_tablero(str: &str) -> Tablero {
        Tablero::crear_de_string(&String::from(str))
    }

    fn verificar_cant_bombas(fila: usize, col: usize, cant_bombas: usize, tablero: &Tablero) {
        assert_eq!(cant_bombas, tablero.cantidad_de_bombas(fila, col))
    }

    #[test]
    fn cantidad_de_bombas_valida() {
        let tablero = construir_tablero(".*.*.\n..*..\n..*..\n.....");

        verificar_cant_bombas(1, 1, 3, &tablero)
    }

    #[test]
    fn cantidad_de_bombas_valida_en_tablero_vacio() {
        let tablero = construir_tablero(".....\n.....\n.....\n.....");

        verificar_cant_bombas(2, 1, 0, &tablero)
    }

    #[test]
    fn cantidad_bombas_valida_en_tablero_lleno() {
        let tablero = construir_tablero("*****\n**.**\n*****");

        verificar_cant_bombas(1, 2, 8, &tablero)
    }
}