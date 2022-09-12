#[cfg(test)]
mod integration_tests {
    use busca_minas::tablero::Tablero;

    fn verificar_representacion_con_tablero_resuelto(
        representacion: String,
        resultado_esperado: String,
    ) {
        let tablero = Tablero::crear_de_string(&representacion);

        assert_eq!(tablero.imprimir_tablero_resuelto(), resultado_esperado)
    }

    #[test]
    fn test_tablero_vacio_devuelve_el_mismo_tablero() {
        let representacion = String::from(".....\n.....\n");
        let esperado = String::from(".....\n.....\n");

        verificar_representacion_con_tablero_resuelto(representacion, esperado)
    }

    #[test]
    fn test_tablero_lleno_devuelve_el_mismo_tablero() {
        let representacion = String::from("*****\n*****\n");
        let esperado = String::from("*****\n*****\n");

        verificar_representacion_con_tablero_resuelto(representacion, esperado)
    }

    #[test]
    fn test_tablero_general_devuelve_el_correcto() {
        let representacion = String::from(".*.*.\n..*..\n..*..\n.....\n");
        let esperado = String::from("1*3*1\n13*31\n.2*2.\n.111.\n");

        verificar_representacion_con_tablero_resuelto(representacion, esperado)
    }
}
