# sabun — Semantic Binary Diff

Method-level DEX diff, resource diff, permission diff. Consumes DexParser, ArchiveReader, AxmlParser traits.

## Build & Test

```bash
cargo build
cargo test
cargo run -- diff <base> <target>
```

## Conventions

- Edition 2024, Rust 1.91.0+, MIT, clippy pedantic
- Release: codegen-units=1, lto=true, opt-level="z", strip=true
