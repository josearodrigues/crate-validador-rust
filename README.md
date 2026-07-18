<p align="center">
    <img src="assets/banner.svg" alt="crate_validador_rust" width="900">
</p>

<h1 align="center">
crate_validador_rust
</h1>

<p align="center">
Biblioteca Rust para validação de documentos brasileiros.
</p>

<p align="center">
Validação rápida, segura e sem dependências externas.
</p>

<p align="center">
<img src="https://img.shields.io/crates/v/crate_validador_rust.svg" alt="Crates.io">
<img src="https://docs.rs/crate_validador_rust/badge.svg" alt="docs.rs">
<img src="https://github.com/josearodrigues/crate-validador-rust/actions/workflows/rust.yml/badge.svg" alt="CI">
<img src="https://img.shields.io/badge/license-MIT%20%7C%20Apache--2.0-blue.svg" alt="License">
<img src="https://img.shields.io/badge/Rust-2024-orange.svg" alt="Rust">
</p>

---

Ideal para aplicações CLI, APIs REST, microsserviços, aplicações Web, estudos da linguagem Rust e projetos que necessitem validar CPFs e CNPJs de forma segura e eficiente.

---

## Índice

- [Por que esta crate?](#por-que-esta-crate)
- [Recursos](#recursos)
- [Documentos suportados](#documentos-suportados)
- [Instalação](#instalação)
- [Uso rápido](#uso-rápido)
- [API](#api)
- [Como funciona](#como-funciona)
- [Objetivos de Design](#objetivos-de-design)
- [Performance](#performance)
- [Estrutura do projeto](#estrutura-do-projeto)
- [Exemplos](#exemplos)
- [Executando testes](#executando-testes)
- [Qualidade do código](#qualidade-do-código)
- [Roadmap](#roadmap)
- [Versionamento](#versionamento)
- [MSRV Policy](#msrv-policy)
- [Segurança](#segurança)
- [Changelog](#changelog)
- [Comunidade](#comunidade)
- [Contribuindo](#contribuindo)
- [Licença](#licença)
- [Objetivos](#objetivos)
- [Autor](#autor)
- [Agradecimentos](#agradecimentos)

---

## Por que esta crate?

Existem diversas implementações para validação de documentos brasileiros.
Esta crate foi desenvolvida com foco em simplicidade, desempenho e aderência às boas práticas da comunidade Rust.

Principais diferenciais:

- API pequena e intuitiva
- Zero dependências externas
- Compatível com Rust Edition 2024
- Algoritmos oficiais
- Aceita documentos com ou sem máscara
- Implementação totalmente segura (`safe Rust`)
- Cobertura abrangente por testes

---

## Recursos

- ✅ Validação de CPF
- ✅ Validação de CNPJ
- ✅ Aceita documentos com ou sem máscara
- ✅ Remove caracteres não numéricos automaticamente
- ✅ API simples e idiomática
- ✅ Sem dependências externas
- ✅ Cobertura por testes
- ✅ Documentação no docs.rs
- ✅ Compatível com Rust Edition 2024

---

## Documentos suportados

| Documento         | Situação |
| ----------------- | -------- |
| CPF               | ✅       |
| CNPJ              | ✅       |

---

## Instalação

Via crates.io

```toml
[dependencies]
crate_validador_rust = "0.2.0"
```

Ou diretamente pelo GitHub

```toml
[dependencies]
crate_validador_rust = { git = "https://github.com/josearodrigues/crate-validador-rust" }
```

---

## Uso rápido

```rust
use crate_validador_rust::{
    validar_cpf,
    validar_cnpj,
};

fn main() {

    assert!(validar_cpf("529.982.247-25"));

    assert!(validar_cnpj("04.252.011/0001-10"));

}
```

---

## API

A biblioteca atualmente disponibiliza:

```rust
pub fn validar_cpf(cpf: &str) -> bool;

pub fn validar_cnpj(cnpj: &str) -> bool;

```

| Retorno | Significado |
|----------|-------------|
| `true` | documento válido |
| `false` | documento inválido |

---

## Como funciona

A validação segue o algoritmo oficial utilizado pela Receita Federal.

As etapas são:

1. Remove caracteres não numéricos;
2. Rejeita sequências de dígitos iguais;
3. Calcula o primeiro dígito verificador;
4. Calcula o segundo dígito verificador;
5. Compara os dígitos calculados com os informados.

---

## Objetivos de Design

Os princípios deste projeto são:

- Simplicidade acima de tudo
- API estável
- Zero dependências
- Alto desempenho
- Código idiomático
- Fácil manutenção
- Fácil extensão para novos documentos

---

## Performance

A biblioteca foi desenvolvida com foco em desempenho.

Características:

- sem dependências externas;
- sem expressões regulares;
- apenas operações sobre dígitos;
- alocação mínima de memória;
- algoritmos lineares (O(n)).

---

## Estrutura do projeto

```
.
├── .github/
│   └── workflows/
│       └── rust.yml
│
├── assets/
│   └── banner.svg
│
├── examples/
│   ├── cnpj.rs
│   └── cpf.rs
│
├── src/
│   ├── cnpj.rs
│   ├── cpf.rs
│   ├── lib.rs
│   └── utils.rs
│
├── tests/
│   ├── cnpj.rs
│   └── cpf.rs
│
├── CHANGELOG.md
├── CODE_OF_CONDUCT.md
├── CONTRIBUTING.md
├── Cargo.toml
├── LICENSE-APACHE
├── LICENSE-MIT
├── README.md
├── SECURITY.md
└── SUPPORT.md
```

O projeto está organizado da seguinte forma:

| Caminho | Descrição |
|---------|-----------|
| `src/` | Código-fonte da biblioteca |
| `tests/` | Testes de integração |
| `examples/` | Exemplos de utilização |
| `assets/` | Recursos gráficos utilizados no README |
| `.github/workflows/` | Integração contínua (GitHub Actions) |
| `CHANGELOG.md` | Histórico das versões |
| `CONTRIBUTING.md` | Guia para contribuidores |
| `CODE_OF_CONDUCT.md` | Código de conduta da comunidade |
| `SECURITY.md` | Política de segurança |
| `SUPPORT.md` | Informações sobre suporte |
| `LICENSE-MIT` | Licença MIT |
| `LICENSE-APACHE` | Licença Apache 2.0 |
| `Cargo.toml` | Manifesto da crate |

---

## Exemplos

```
cargo run --example cnpj
cargo run --example cpf
```

---

## Executando testes

```
cargo test
```

---

## Qualidade do código

Antes de cada publicação execute:

```bash
cargo fmt

cargo clippy --workspace --all-targets --all-features

cargo test

cargo package

cargo publish --dry-run
```

---

## Roadmap

### Documentos

- [x] CPF
- [x] CNPJ
- [ ] PIS/PASEP
- [ ] CNH
- [ ] RENAVAM
- [ ] Título de Eleitor

### Melhorias

- [ ] Benchmark (Criterion)
- [ ] Fuzz Testing
- [ ] no_std
- [ ] Cobertura >95%

---

## Versionamento

Este projeto segue Semantic Versioning (SemVer).

- PATCH → correções de bugs
- MINOR → novas funcionalidades compatíveis
- MAJOR → mudanças incompatíveis

Enquanto estiver abaixo da versão 1.0.0, pequenas alterações na API ainda podem ocorrer.

---

## MSRV Policy

Até a versão 1.0.0, a crate acompanha a versão estável mais recente do Rust.

Após a versão 1.0.0 será definida uma MSRV oficial, que somente será alterada em versões major ou quando estritamente necessário.

---

## Segurança

Esta biblioteca verifica apenas a validade matemática dos documentos.

Ela não consulta bases oficiais.

Não verifica:

- Receita Federal
- situação cadastral
- cancelamentos
- óbitos
- autenticidade jurídica

---

## Changelog

Todas as alterações do projeto seguem o arquivo
CHANGELOG.md.

Cada versão documenta:

- funcionalidades adicionadas;
- melhorias;
- correções;
- alterações incompatíveis.

---

## Comunidade

Sugestões, dúvidas e contribuições são sempre bem-vindas.

Você pode participar através de:

- Issues
- Discussions
- Pull Requests

---

## Contribuindo

Contribuições são muito bem-vindas.

Caso encontre algum problema ou tenha sugestões:

1. Abra uma Issue;
2. Faça um Fork;
3. Crie uma branch;
4. Envie um Pull Request.

---

## Licença

Este projeto está licenciado sob os termos das licenças **MIT** ou **Apache License 2.0**, à sua escolha.

Consulte os arquivos:

- `LICENSE-MIT`
- `LICENSE-APACHE`

---

## Objetivos

Este projeto busca:

- oferecer uma API simples;
- seguir as boas práticas da comunidade Rust;
- evitar dependências externas;
- implementar os algoritmos oficiais;
- servir como material de estudo da linguagem Rust.

---

## Autor

José Américo Rodrigues

- GitHub: https://github.com/josearodrigues

---

## Agradecimentos

Este projeto foi desenvolvido como parte dos estudos da linguagem Rust e tem como objetivo servir como uma biblioteca simples, idiomática e alinhada às recomendações da comunidade Rust.