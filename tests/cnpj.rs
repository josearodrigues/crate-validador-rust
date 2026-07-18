use crate_validador_rust::validar_cnpj;

#[test]
fn cnpj_valido() {
    assert!(validar_cnpj("04252011000110"));
}

#[test]
fn cnpj_com_formatacao() {
    assert!(validar_cnpj("04.252.011/0001-10"));
}

#[test]
fn cnpj_invalido() {
    assert!(!validar_cnpj("04252011000100"));
}

#[test]
fn cnpj_com_digitos_iguais() {
    assert!(!validar_cnpj("11.111.111/1111-11"));
}

#[test]
fn cnpj_vazio() {
    assert!(!validar_cnpj(""));
}

#[test]
fn cnpj_curto() {
    assert!(!validar_cnpj("123"));
}

#[test]
fn cnpj_longo() {
    assert!(!validar_cnpj("123456789012345678"));
}

#[test]
fn cnpj_com_letras() {
    assert!(!validar_cnpj("ABCD"));
}

#[test]
fn cnpj_com_caracteres_invalidos() {
    assert!(!validar_cnpj("04.252.011/0001-XX"));
}
