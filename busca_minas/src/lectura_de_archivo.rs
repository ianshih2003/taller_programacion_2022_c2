use std::fs;

/// Lee el archivo que corresponde al directorio y lo devuelve como String
pub fn leer_archivo(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(val) => val,
        Err(_) => String::from("Error en lectura de archivo"),
    }
}

#[cfg(test)]
mod unit_tests {
    use crate::leer_archivo;

    #[test]
    fn lectura_de_archivo() {
        let resultado = leer_archivo("file_tests/test_tablero_1.txt");
        let esperado = ".*.*.\n..*..\n..*..\n.....\n";

        assert_eq!(resultado, esperado);
    }

    #[test]
    fn lectura_de_archivo_invalido() {
        let resultado = leer_archivo("file_tests/invalido.txt");
        let esperado = "Error en lectura de archivo";

        assert_eq!(resultado, esperado);
    }

    #[test]
    fn lectura_de_archivo_vacio() {
        let resultado = leer_archivo("file_tests/vacio.txt");
        let esperado = "";

        assert_eq!(resultado, esperado);
    }
}
