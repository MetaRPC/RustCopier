# First request with Rust

List your own copiers to check connectivity and identity without creating a terminal or placing a trade.

## 1. Install

Requires **Stable Rust, Cargo and protoc on PATH**. These instructions build from source; they do not assume a registry release exists.

```bash
git clone https://github.com/MetaRPC/RustCopier.git
cd RustCopier
cargo test --all-targets
```

Java on Windows uses `;` for classpath separators. Run C++ builds in a Visual Studio developer shell. Activate your Python virtual environment before installing.

## 2. Set your owner key

=== "Bash"
    ```bash
    export MRPC_API_KEY='your-owner-key'
    ```
=== "PowerShell"
    ```powershell
    $env:MRPC_API_KEY = 'your-owner-key'
    ```

Use the real owner API key, not a broker password, terminal ID or `TRIAL`. Read [authentication](authentication.md) if access is refused.

## 3. Run

```bash
cargo run --example list
```

Source: [`examples/list.rs`](https://github.com/MetaRPC/RustCopier/blob/main/examples/list.rs).

```rust
use rustcopier::client::CopierService;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key = std::env::var("MRPC_API_KEY")?;
    let service = CopierService::connect("https://copy.mrpc.pro", &key).await?;
    let result = service.list().await?;
    if !result.ok { return Err(result.error.into()); }
    for copier in result.copiers { println!("{}: paused={}", copier.id, copier.paused); }
    Ok(())
}
```

A successful empty list means this owner has no copiers. Treat failed requests as errors, not empty accounts.

## 4. Prepare a demo workflow

Read [the lifecycle guide](lifecycle.md), then [choose sizing](sizing.md). Existing replication examples under `examples/` are advanced integration examples: they may provision accounts and trade. Inspect them and their cleanup before running with disposable demos. List is the default starting point.
