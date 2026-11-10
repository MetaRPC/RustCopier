# MetaRPC Rust Trade Copier SDK

Authenticate, configure a master–follower relationship, inspect its state, pause and remove it.

**[Documentation](https://metarpc.github.io/RustCopier/) · [Quickstart](https://metarpc.github.io/RustCopier/quickstart/) · [Authentication](https://metarpc.github.io/RustCopier/authentication/) · [API reference](https://metarpc.github.io/RustCopier/reference/)**

## Install from source

Requires Stable Rust, Cargo and protoc on PATH.

```bash
git clone https://github.com/MetaRPC/RustCopier.git
cd RustCopier
cargo test --all-targets
```

## First request

Set `MRPC_API_KEY` to your real owner API key in your environment, then run:

```bash
cargo run --example list
```

This List example is read-only: no account creation or trades. 

## Authentication

Every SDK call sends `ApiKey` (`apikey` in gRPC metadata). The gRPC body also carries `user_key`; the header does not replace it. `TRIAL` is not an owner key. Read the [verified transport contract](https://metarpc.github.io/RustCopier/authentication/).

## Scope and verification

cTrader Spotware demo ↔ MT5 and TradeLocker HEROFX demo ↔ MT5 acceptance passed in both directions on November 10, 2026: five sizing modes, pause/resume, live/persisted logs and API-key errors. See the [provider guide](https://metarpc.github.io/RustCopier/providers/) and [dated results](https://metarpc.github.io/RustCopier/testing/).

MT4 FreshForex-Demo and MT5 MetaQuotes-Demo acceptance passed on November 10, 2026: opens/closes, five sizing modes, pause/resume, live/persisted logs and API-key failures. See the [separate broker test evidence](https://metarpc.github.io/RustCopier/testing/).

Core operations: connect · start · list · pause · remove (async). Advanced server features are distinguished from the supported wrapper surface. Review legacy replication examples before using them with disposable demos. Standard tests need no broker credentials.

See [testing and contributing](https://metarpc.github.io/RustCopier/testing/) for the regression suite, demo checklist and Pages build. Edit `docs/`; generated `site/` output is not source.
