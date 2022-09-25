//! Modulo lectura de archivos

use std::fs;

use crate::BuscaminasError::{self, IoError};

/// Lee el archivo que corresponde al directorio y lo devuelve como String. Si ocurre algun error de lectura del archivo, devuelve error
///
/// # Argumentos
/// `directorio` - Ruta al archivo que se quiere leer
///
/// # Errores
/// Devuelve `IoError` si la ruta no existe o si hubo un error en la lectura
///
/// # Ejemplos
/// ```
/// use busca_minas::lectura_de_archivo::leer_archivo;
/// use busca_minas::BuscaminasError;
///
/// fn main() -> Result<(), BuscaminasError> {
///     let tablero = leer_archivo("file_tests/test_tablero_1.txt")?;
///     Ok(())
/// }
/// ```
pub fn leer_archivo(directorio: &str) -> Result<String, BuscaminasError> {
    match fs::read_to_string(directorio) {
        Ok(tablero) => Ok(tablero),
        Err(_) => Err(IoError),
    }
}

#[cfg(test)]
mod unit_tests {
    use crate::BuscaminasError::IoError;
    use crate::{leer_archivo, BuscaminasError};

    #[test]
    fn lectura_de_archivo() -> Result<(), BuscaminasError> {
        let resultado = leer_archivo("file_tests/test_tablero_1.txt")?;
        let esperado = ".*.*.\n..*..\n..*..\n.....\n";

        assert_eq!(resultado, esperado);
        Ok(())
    }

    #[test]
    fn lectura_de_archivo_invalido() -> Result<(), BuscaminasError> {
        let resultado = leer_archivo("file_tests/invalido.txt");
        let esperado = "Error en la lectura del archivo";
        match resultado {
            Ok(_) => assert!(false),
            Err(e) => {
                if let IoError = e {
                    assert_eq!("Error en la lectura del archivo", esperado)
                }
            }
        }

        Ok(())
    }

    #[test]
    fn lectura_de_archivo_vacio() -> Result<(), BuscaminasError> {
        let resultado = leer_archivo("file_tests/vacio.txt")?;
        let esperado = "";

        assert_eq!(resultado, esperado);
        Ok(())
    }
}
