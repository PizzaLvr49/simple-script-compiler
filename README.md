# simple-script-compiler

A tiny simple script language compiler/interpreter for learning and testing.

## Building

Requires Rust and Cargo. On Windows with PowerShell:

```powershell
cargo build --release
```

## Running

There is a CLI binary in `src/main.rs`. After building:

```powershell
cargo run -- path\to\script.ss
```

Replace `path\to\script.ss` with your script file.

## Testing

Run the test suite:

```powershell
cargo test
```

## License

Licensed under MIT. See `LICENSE`.
