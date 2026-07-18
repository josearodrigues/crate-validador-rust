use crate::utils::{calcular_digito_mod11, para_digitos, somente_digitos, todos_iguais};

/// Valida um número de CPF.
///
/// Aceita CPFs com ou sem formatação.
///
/// # Exemplos
///
/// ```
/// use crate_validador_rust::validar_cpf;
///
/// assert!(validar_cpf("529.982.247-25"));
/// assert!(validar_cpf("52998224725"));
///
/// assert!(!validar_cpf("12345678900"));
/// ```
pub fn validar_cpf(cpf: &str) -> bool {
    let cpf = somente_digitos(cpf);

    if cpf.len() != 11 {
        return false;
    }

    let numeros = para_digitos(&cpf);

    if todos_iguais(&numeros) {
        return false;
    }

    const PESOS_DV1: [u32; 9] = [10, 9, 8, 7, 6, 5, 4, 3, 2];
    const PESOS_DV2: [u32; 10] = [11, 10, 9, 8, 7, 6, 5, 4, 3, 2];

    let dv1 = calcular_digito_mod11(&numeros[..9], &PESOS_DV1);

    if numeros[9] != dv1 {
        return false;
    }

    let dv2 = calcular_digito_mod11(&numeros[..10], &PESOS_DV2);

    numeros[10] == dv2
}
