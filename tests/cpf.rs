use validador::validar_cpf;

#[test]
fn cpf_valido() {
    assert!(validar_cpf("52998224725"));
}

#[test]
fn cpf_invalido() {
    assert!(!validar_cpf("12345678900"));
}

#[test]
fn cpf_com_formatacao() {
    assert!(validar_cpf("529.982.247-25"));
}

#[test]
fn cpf_com_digitos_iguais() {
    assert!(!validar_cpf("11111111111"));
}
