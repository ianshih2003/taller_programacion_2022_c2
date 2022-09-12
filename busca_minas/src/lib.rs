pub mod lectura_de_archivo;
pub mod tablero;

use crate::lectura_de_archivo::leer_archivo;
use crate::tablero::Tablero;

/// Recibe directorio donde se encuentra el archivo que representa el tablero
/// Imprime un tablero resuelto
pub fn run(path: &str) {
    let archivo = leer_archivo(path);

    let tablero = Tablero::crear_de_string(&archivo);

    tablero.imprimir_tablero_resuelto();
}
