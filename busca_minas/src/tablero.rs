//! Modulo de tablero de buscaminas

use crate::BuscaminasError::{self, ErrorDeParseo};

const BOMBA_UTF: u8 = 42;
const ESPACIO_UTF: u8 = 46;

const AVANZA: i32 = 1;
const NO_MUEVE: i32 = 0;
const RETROCEDE: i32 = -1;

/// Posibles valores de una casilla
pub enum Casilla {
    /// Representacion en string: "*"
    Bomba,
    /// Representacion en string: "."
    Espacio,
}

/// Representacion del tablero del buscaminas
pub struct Tablero {
    /// Matriz de casillas
    pub casillas: Vec<Vec<Casilla>>,
}

impl TryFrom<&String> for Tablero {
    /// Tipo de error devuelto en caso de que falle la conversion
    type Error = BuscaminasError;

    /// Devuelve un tablero dado una referencia a un str que lo representa.
    ///
    /// # Argumentos
    /// * `tablero` - String que representa un tablero, solo puede contener * (Bombas) o . (Casillas)
    ///
    /// # Errores
    /// Lanza un `ErrorDeParseo` si:
    /// * Hay un caracter invalido dentro del string
    /// * Hay filas de distintos tamaños
    ///
    /// # Ejemplos
    /// ```
    /// use busca_minas::tablero::Tablero;
    /// use busca_minas::BuscaminasError;
    ///
    /// fn main() -> Result<(), BuscaminasError> {
    ///     let representacion: String = String::from(".....\n.....\n");
    ///     let tablero: Tablero = Tablero::try_from(&representacion)?;
    ///
    ///     assert_eq!(tablero.filas(), 2);
    ///     Ok(())
    /// }
    /// ```
    fn try_from(tablero: &String) -> Result<Self, Self::Error> {
        let mut filas: Vec<Vec<Casilla>> = Vec::new();
        let mut casillas: Vec<Casilla> = Vec::new();

        for linea in tablero.split(NUEVA_LINEA) {
            if linea.is_empty() {
                continue;
            }
            for casilla in linea.as_bytes() {
                match *casilla {
                    BOMBA_UTF => casillas.push(Casilla::Bomba),
                    ESPACIO_UTF => casillas.push(Casilla::Espacio),
                    _ => {
                        return Err(ErrorDeParseo(String::from(
                            "Caracteres invalidos dentro del archivo",
                        )))
                    }
                }
            }
            filas.push(casillas);
            casillas = Vec::new()
        }

        if !filas.is_empty() && !Tablero::filas_de_mismo_tam(&filas) {
            return Err(ErrorDeParseo(String::from(
                "No pueden haber filas de distinto tamaño",
            )));
        }

        Ok(Tablero::new(filas))
    }
}

const BOMBA: &str = "*";
const CASILLA: &str = ".";
const NUEVA_LINEA: &str = "\n";

impl Tablero {
    /// Devuelve un tablero a partir de una matriz de Casillas
    fn new(casillas: Vec<Vec<Casilla>>) -> Tablero {
        Tablero { casillas }
    }

    /// Devuelve la cantidad de filas del tablero
    ///
    /// # Ejemplos
    /// ```
    /// use busca_minas::tablero::Tablero;
    /// use busca_minas::BuscaminasError;
    ///
    /// fn main() -> Result<(), BuscaminasError> {
    ///     let representacion: String = String::from(".....\n.....\n.....\n");
    ///     let tablero: Tablero = Tablero::try_from(&representacion)?;
    ///
    ///     assert_eq!(tablero.filas(), 3);
    ///     Ok(())
    /// }
    /// ```
    pub fn filas(&self) -> usize {
        self.casillas.len()
    }

    /// Devuelve la cantidad de colunas del tablero
    /// Precondicion: La matriz debe contener al menos una fila
    ///
    /// # Ejemplos
    /// ```
    /// use busca_minas::tablero::Tablero;
    /// use busca_minas::BuscaminasError;
    ///
    /// fn main() -> Result<(), BuscaminasError> {
    ///     let representacion: String = String::from(".....\n.....\n.....\n");
    ///     let tablero: Tablero = Tablero::try_from(&representacion)?;
    ///
    ///     assert_eq!(tablero.columnas(), 5);
    ///     Ok(())
    /// }
    /// ```
    pub fn columnas(&self) -> usize {
        self.casillas[0].len()
    }

    /// Devuelve un String que representa al tablero resuelto
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use busca_minas::tablero::Tablero;
    /// use busca_minas::BuscaminasError;
    ///
    /// fn main() -> Result<(), BuscaminasError> {
    ///     let representacion: String = String::from(".*.*.\n..*..\n..*..\n.....\n");
    ///     let tablero = Tablero::try_from(&representacion)?;
    ///
    ///     assert_eq!(tablero.resolver(), "1*3*1\n13*31\n.2*2.\n.111.\n");
    ///     Ok(())
    /// }
    ///
    /// ```
    pub fn resolver(&self) -> String {
        let mut resultado: String = String::new();
        for (i, fila) in self.casillas.iter().enumerate() {
            for (j, casilla) in fila.iter().enumerate() {
                match casilla {
                    Casilla::Bomba => resultado += BOMBA,
                    Casilla::Espacio => {
                        let cant = self.cantidad_de_bombas(i, j);
                        if cant == 0 {
                            resultado += CASILLA
                        } else {
                            resultado += format!("{cant}").as_str();
                        }
                    }
                }
            }
            if i < self.filas() {
                resultado += NUEVA_LINEA;
            }
        }
        resultado
    }

    /// Devuelve las posiciones adyacentes de una posicion del tablero
    ///
    /// # Argumentos
    /// `pos_fila`: Numero de fila dentro del tablero
    ///             Precondicion: La posicion tiene que ser mayor o igual que 0 y menor que la cantidad de filas del tablero
    /// `pos_col`: Numero de columna dentro del tablero
    ///             Precondicion: La posicion tiene que ser mayor o igual que 0 y menor que la cantidad de columnas del tablero
    /// Importante:
    /// Las primera posicion es la posicion 0
    fn construir_posiciones_adyacentes(
        &self,
        pos_fila: usize,
        pos_col: usize,
    ) -> Vec<(usize, usize)> {
        let mut posiciones: Vec<(usize, usize)> = Vec::new();

        let limite_inf = if pos_fila > 0 { RETROCEDE } else { NO_MUEVE };
        let limite_sup = if pos_fila < self.filas() - 1 {
            AVANZA
        } else {
            NO_MUEVE
        };
        let limite_inf_col = if pos_col > 0 { RETROCEDE } else { 0 };
        let limite_sup_col = if pos_col < self.columnas() - 1 {
            AVANZA
        } else {
            NO_MUEVE
        };

        for dx in limite_inf..(limite_sup + 1) {
            for dy in limite_inf_col..(limite_sup_col + 1) {
                if dx == 0 && dy == 0 {
                    continue;
                }
                posiciones.push((
                    (pos_fila as i32 + dx) as usize,
                    (pos_col as i32 + dy) as usize,
                ))
            }
        }
        posiciones
    }

    /// Devuelve la cantidad de bombas cercanas de una posicion del tablero
    ///
    /// # Argumentos
    /// `pos_fila`: Numero de fila dentro del tablero
    ///             Precondicion: La posicion tiene que ser mayor o igual que 0 y menor que la cantidad de filas del tablero
    /// `pos_col`: Numero de columna dentro del tablero
    ///             Precondicion: La posicion tiene que ser mayor o igual que 0 y menor que la cantidad de columnas del tablero
    /// Importante:
    /// Las primera posicion es la posicion 0
    fn cantidad_de_bombas(&self, pos_fila: usize, pos_col: usize) -> usize {
        let mut cantidad: usize = 0;

        let posiciones_ady = self.construir_posiciones_adyacentes(pos_fila, pos_col);

        for (i, j) in posiciones_ady {
            match self.casillas[i][j] {
                Casilla::Bomba => cantidad += 1,
                _ => continue,
            }
        }
        cantidad
    }

    /// Verifica que una matriz tenga la misma cantidad de elementos en una fila
    ///
    /// # Argumentos
    /// `tablero` - Una matriz de casillas
    ///             Precondicion: Tiene que ser una matriz de al menos 1 fila
    ///
    fn filas_de_mismo_tam(tablero: &[Vec<Casilla>]) -> bool {
        let fila_len = (*tablero[0]).len();
        tablero.iter().all(|fila| fila.len() == fila_len)
    }
}

#[cfg(test)]
mod unit_tests {
    use crate::{tablero::Tablero, BuscaminasError};

    fn construir_tablero(str: &str) -> Result<Tablero, BuscaminasError> {
        Tablero::try_from(&String::from(str))
    }

    fn verificar_cant_bombas(
        fila: usize,
        col: usize,
        cant_bombas: usize,
        tablero: &Tablero,
    ) -> Result<(), BuscaminasError> {
        assert_eq!(cant_bombas, tablero.cantidad_de_bombas(fila, col));
        Ok(())
    }

    #[test]
    fn cantidad_de_bombas_valida() -> Result<(), BuscaminasError> {
        let tablero = construir_tablero(".*.*.\n..*..\n..*..\n.....")?;

        verificar_cant_bombas(1, 1, 3, &tablero)
    }

    #[test]
    fn cantidad_de_bombas_valida_en_posicion_extrema_izquierda() -> Result<(), BuscaminasError> {
        let tablero = construir_tablero(".*.*.\n..*..\n..*..\n.....\n")?;

        verificar_cant_bombas(0, 0, 1, &tablero)
    }

    #[test]
    fn cantidad_de_bombas_valida_en_posicion_extrema_derecha() -> Result<(), BuscaminasError> {
        let tablero = construir_tablero(".*.*.\n..*..\n..*..\n.....\n")?;

        verificar_cant_bombas(3, 4, 0, &tablero)
    }

    #[test]
    fn cantidad_de_bombas_valida_en_tablero_vacio() -> Result<(), BuscaminasError> {
        let tablero = construir_tablero(".....\n.....\n.....\n.....\n")?;

        verificar_cant_bombas(2, 1, 0, &tablero)
    }

    #[test]
    fn cantidad_bombas_valida_en_tablero_lleno() -> Result<(), BuscaminasError> {
        let tablero = construir_tablero("*****\n**.**\n*****\n")?;

        verificar_cant_bombas(1, 2, 8, &tablero)
    }
}
