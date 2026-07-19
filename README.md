# bastion-rs

> [!WARNING]
> **Early skeleton — not the runtime.** This port is roughly 140 lines. It does
> not query a model, read the oracle, or implement replay, and `run` currently
> decides from placeholder votes rather than market data — the `risk` and
> `chain` modules are not wired into it yet (`cargo` will warn about the dead
> code). It is kept public as a work-in-progress port, not as something to run.
>
> **Use the TypeScript implementation instead** — it has the council, the risk
> kernel, regime detection, reflexive memory and deterministic replay:
>
> ```bash
> curl -fsSL https://bastiontrade.xyz/install.sh | sh
> bastion replay --demo
> ```
>
> → [adrydevel/bastion](https://github.com/adrydevel/bastion)

**Rust port of the [Bastion](https://github.com/adrydevel/bastion) agent core.**

The goal is a single dependency-light binary carrying the risk kernel and
council logic. Tracking issue for reaching parity: see the parent repo.

## Modules

- `risk` — fractional-Kelly sizing *(implemented, not yet called by `run`)*
- `council` — quorum vote over agent opinions *(implemented)*
- `chain` — Robinhood Chain adapter *(stub)*
- `cli` — the `bastion` command

## License

MIT
