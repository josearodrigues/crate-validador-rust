/// Valida um número de CPF.
///
/// Aceita CPFs com ou sem formatação.
///
/// # Exemplos
///
/// ```
/// use validador::validar_cpf;
///
/// assert!(validar_cpf("529.982.247-25"));
/// assert!(validar_cpf("52998224725"));
///
/// assert!(!validar_cpf("12345678900"));
/// ```
pub fn validar_cpf(cpf: &str) -> bool {
    // Remove qualquer caractere que não seja número
    let cpf: String = cpf.chars().filter(|c| c.is_ascii_digit()).collect();

    // CPF deve possuir 11 dígitos
    if cpf.len() != 11 {
        return false;
    }

    // Converte para vetor de números
    let numeros: Vec<u32> = cpf.chars().map(|c| c.to_digit(10).unwrap()).collect();

    // Rejeita CPFs com todos os dígitos iguais
    if numeros.iter().all(|&n| n == numeros[0]) {
        return false;
    }

    // ==========================
    // Primeiro dígito verificador
    // ==========================
    let soma: u32 = numeros[..9]
        .iter()
        .zip((2..=10).rev())
        .map(|(&n, peso)| n * peso)
        .sum();

    let resto = soma % 11;
    let dv1 = if resto < 2 { 0 } else { 11 - resto };

    if numeros[9] != dv1 {
        return false;
    }

    // ==========================
    // Segundo dígito verificador
    // ==========================
    let soma: u32 = numeros[..10]
        .iter()
        .zip((2..=11).rev())
        .map(|(&n, peso)| n * peso)
        .sum();

    let resto = soma % 11;
    let dv2 = if resto < 2 { 0 } else { 11 - resto };

    numeros[10] == dv2
}
