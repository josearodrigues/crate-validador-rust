use crate_validador_rust::validar_cpf;

fn main() {
    let cpf = "529.982.247-25";

    if validar_cpf(cpf) {
        println!("CPF válido.");
    } else {
        println!("CPF inválido.");
    }
}
