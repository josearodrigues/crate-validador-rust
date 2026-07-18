use crate_validador_rust::validar_cnpj;

fn main() {
    let cnpj = "04.252.011/0001-10";

    if validar_cnpj(cnpj) {
        println!("CNPJ válido.");
    } else {
        println!("CNPJ inválido.");
    }
}
