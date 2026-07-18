use crate::utils::{calcular_digito_mod11, para_digitos, somente_digitos, todos_iguais};

/// Valida um número de CNPJ.
///
/// Aceita CNPJs com ou sem formatação.
///
/// # Exemplos
///
/// ```
/// use crate_validador_rust::validar_cnpj;
///
/// assert!(validar_cnpj("04.252.011/0001-10"));
/// assert!(validar_cnpj("04252011000110"));
///
/// assert!(!validar_cnpj("11.111.111/1111-11"));
/// ```
pub fn validar_cnpj(cnpj: &str) -> bool {
    let cnpj = somente_digitos(cnpj);

    if cnpj.len() != 14 {
        return false;
    }

    let numeros = para_digitos(&cnpj);

    if todos_iguais(&numeros) {
        return false;
    }

    const PESOS_DV1: [u32; 12] = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];

    const PESOS_DV2: [u32; 13] = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];

    let dv1 = calcular_digito_mod11(&numeros[..12], &PESOS_DV1);

    if numeros[12] != dv1 {
        return false;
    }

    let dv2 = calcular_digito_mod11(&numeros[..13], &PESOS_DV2);

    numeros[13] == dv2
}
