/// Remove qualquer caractere que não seja dígito.
pub(crate) fn somente_digitos(valor: &str) -> String {
    valor.chars().filter(|c| c.is_ascii_digit()).collect()
}

/// Converte uma string numérica para vetor de dígitos.
pub(crate) fn para_digitos(valor: &str) -> Vec<u32> {
    valor.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

/// Retorna `true` caso todos os dígitos sejam iguais.
pub(crate) fn todos_iguais(numeros: &[u32]) -> bool {
    numeros.iter().all(|&n| n == numeros[0])
}

/// Calcula um dígito verificador utilizando os pesos informados.
pub(crate) fn calcular_digito_mod11(numeros: &[u32], pesos: &[u32]) -> u32 {
    let soma: u32 = numeros
        .iter()
        .zip(pesos.iter())
        .map(|(&numero, &peso)| numero * peso)
        .sum();

    let resto = soma % 11;

    if resto < 2 { 0 } else { 11 - resto }
}
