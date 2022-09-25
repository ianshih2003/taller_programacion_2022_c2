#[cfg(test)]
mod integration_tests {
    use busca_minas::{
        tablero::Tablero,
        BuscaminasError::{self, ErrorDeParseo},
    };

    fn verificar_representacion_con_tablero_resuelto(
        representacion: String,
        resultado_esperado: String,
    ) -> Result<(), BuscaminasError> {
        let tablero = Tablero::try_from(&representacion)?;

        assert_eq!(tablero.resolver(), resultado_esperado);
        Ok(())
    }

    #[test]
    fn test_tablero_vacio_devuelve_el_mismo_tablero() -> Result<(), BuscaminasError> {
        let representacion = String::from(".....\n.....\n");
        let esperado = String::from(".....\n.....\n");

        verificar_representacion_con_tablero_resuelto(representacion, esperado)
    }

    #[test]
    fn test_tablero_lleno_devuelve_el_mismo_tablero() -> Result<(), BuscaminasError> {
        let representacion = String::from("*****\n*****\n");
        let esperado = String::from("*****\n*****\n");

        verificar_representacion_con_tablero_resuelto(representacion, esperado)
    }

    #[test]
    fn test_tablero_general_devuelve_el_correcto() -> Result<(), BuscaminasError> {
        let representacion = String::from(".*.*.\n..*..\n..*..\n.....\n");
        let esperado = String::from("1*3*1\n13*31\n.2*2.\n.111.\n");

        verificar_representacion_con_tablero_resuelto(representacion, esperado)
    }

    #[test]
    fn test_tablero_con_caracteres_invalidos_devuelve_error() -> Result<(), BuscaminasError> {
        let representacion = String::from(".a.*.\n..*..\n..*..\n.....\n");

        let tablero = Tablero::try_from(&representacion);

        let error_esperado: String = String::from("Caracteres invalidos dentro del archivo");

        match tablero {
            Ok(_) => assert!(false),
            Err(err) => {
                if let ErrorDeParseo(mensaje) = err {
                    assert_eq!(mensaje, error_esperado)
                }
            }
        }

        Ok(())
    }

    #[test]
    fn test_tablero_con_filas_de_distinto_tam_devuelve_error() -> Result<(), BuscaminasError> {
        let representacion = String::from("..\n..*..\n");

        let tablero = Tablero::try_from(&representacion);

        let error_esperado: String = String::from("No pueden haber filas de distinto tamaño");

        match tablero {
            Ok(_) => assert!(false),
            Err(err) => {
                if let ErrorDeParseo(mensaje) = err {
                    assert_eq!(mensaje, error_esperado)
                }
            }
        }

        Ok(())
    }
}
