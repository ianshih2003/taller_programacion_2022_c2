pub mod lectura_de_archivo;
pub mod tablero;

use std::fmt::{Display, Formatter};

use crate::lectura_de_archivo::leer_archivo;
use crate::tablero::Tablero;

#[derive(Debug)]
/// Errores dentro del buscaminas
pub enum BuscaminasError {
    /// Error en la lectura del archivo
    IoError,

    /// Error en el parseo del tablero
    ErrorDeParseo(String),
}

impl Display for BuscaminasError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            BuscaminasError::IoError => write!(f, "Error en la lectura del archivo"),
            BuscaminasError::ErrorDeParseo(string) => {
                write!(f, "Error en parseo de Tablero: {}", string)
            }
        }
    }
}

/// Recibe directorio donde se encuentra el archivo que representa el tablero e imprime un tablero resuelto
///
/// # Argumentos
/// `directorio` - Directorio donde se encuentra la representacion del tableor
///
/// # Errores
/// Devuelve:
/// * `ErrorDeParseo` si hay caracteres invalidos dentro del archivo
/// * `ErrorDeParseo` si hay filas de distintos tamaños
/// * `IoError` si hubo un problema en la lectura de archivo (directorio no existente, etc...)
pub fn run(directorio: &str) -> Result<(), BuscaminasError> {
    let archivo = leer_archivo(directorio)?;

    let tablero = Tablero::try_from(&archivo)?;

    print!("{}", tablero.resolver());
    Ok(())
}
