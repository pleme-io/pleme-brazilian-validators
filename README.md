# pleme-brazilian-validators

Brazilian document and format validators for Pleme platform (CPF, CNPJ, CEP, phone, PIX)

## Installation

```toml
[dependencies]
pleme-brazilian-validators = "0.1"
```

## Usage

```rust
use pleme_brazilian_validators::{Cpf, Cnpj, Cep};

let cpf = Cpf::parse("123.456.789-09")?;
assert!(cpf.is_valid());

let cnpj = Cnpj::parse("11.222.333/0001-81")?;
let cep = Cep::parse("01001-000")?;
```

## Feature Flags

| Feature | Description |
|---------|-------------|
| `serialization` | Serde serialize/deserialize support |
| `graphql` | async-graphql scalar types |
| `full` | All features enabled |

Enable features in your `Cargo.toml`:

```toml
pleme-brazilian-validators = { version = "0.1", features = ["full"] }
```

## Development

This project uses [Nix](https://nixos.org/) for reproducible builds:

```bash
nix develop            # Dev shell with Rust toolchain
nix run .#check-all    # cargo fmt + clippy + test
nix run .#publish      # Publish to crates.io (--dry-run supported)
nix run .#regenerate   # Regenerate Cargo.nix
```

## License

MIT - see [LICENSE](LICENSE) for details.
