# bastion-rs

**Rust port of the [Bastion](https://github.com/adrydevel/bastion) agent core.**

Single small binary — the risk kernel and council logic in Rust for a faster, dependency-light runtime.

```bash
cargo install --git https://github.com/adrydevel/bastion-rs
bastion run --ticker NVDA
```

## Modules

- `risk` — fractional-Kelly sizing
- `council` — quorum vote over agent opinions
- `chain` — Robinhood Chain adapter
- `cli` — the `bastion` command

## License

MIT
