# Redis with Rust Example

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Redis](https://img.shields.io/badge/redis-%23DC382D.svg?style=for-the-badge&logo=redis&logoColor=white)](https://redis.io/)
[![Tokio](https://img.shields.io/badge/tokio-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://tokio.rs/)

Um exemplo prático em Rust demonstrando como interagir com Redis de forma assíncrona, incluindo conexões TLS, gerenciamento de conexões e armazenamento/recuperação de structs personalizados como hashes Redis.

## ✨ Funcionalidades

- **Conexões assíncronas**: Utiliza [`tokio`](https://tokio.rs/) e a crate [`redis`](https://crates.io/crates/redis) para operações assíncronas.
- **Suporte a TLS**: Conexões seguras com Redis via TLS.
- **Padrão Builder**: Configuração flexível de conexões Redis através do [`RedisConfig`](src/config/mod.rs).
- **Serialização**: Uso de [`serde`](https://serde.rs/) para structs personalizados como [`Person`](src/lib.rs).
- **Testes de integração**: Validação de operações Redis.
- **Git hooks**: Verificações de qualidade de código com [`lefthook`](https://github.com/evilmartians/lefthook).

## 📋 Pré-requisitos

- Rust (edição 2021 ou superior)
- Servidor Redis em execução (padrão: localhost:6380 com TLS)
- Opcional: `cargo-audit` para auditorias de segurança (instalado via makefile)

## 🚀 Instalação

1. **Clone o repositório**:
   ```bash
   git clone https://github.com/diegohat/redis-with-rust-example.git
   cd redis-with-rust-example
   ```

2. **Instale as dependências**:
   ```bash
   cargo build
   ```

3. **Configure os git hooks** (opcional, para verificações pré-commit e pré-push):
   ```bash
   make setup-hooks
   ```

## 📖 Uso

### Executando o Exemplo

Execute o programa principal para ver operações de hash Redis com uma struct [`Person`](src/lib.rs):

```bash
cargo run
```

Isso conectará ao Redis, armazenará uma pessoa como hash, recuperará e imprimirá os resultados.

### Usando a Biblioteca

Importe e utilize o builder [`RedisConfig`](src/config/mod.rs) para conectar:

```rust
use redis_with_rust_example::RedisConfig;

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    let config = RedisConfig::builder()
        .host("localhost")
        .port(6380)
        .tls(true)
        .db(15)
        .build();

    let mut conn = config.connect().await?;
    // Realize operações Redis...
    Ok(())
}
```

### Structs Personalizados

Defina structs que implementem `TryFrom<HashMap<String, String>>` para conversão fácil de hashes Redis, como mostrado em [`Person`](src/lib.rs).

## 🧪 Testes

Execute os testes unitários:
```bash
cargo test
```

Execute os testes de integração (requer servidor Redis):
```bash
cargo test --test integration
```

## 🤝 Contribuição

- Formatação de código com `cargo fmt`
- Linting com `cargo clippy`
- Auditorias de segurança com `cargo audit`
- Git hooks (via lefthook) aplicam essas verificações em commits e pushes

## 📄 Licença

Este projeto está licenciado sob a Licença MIT.