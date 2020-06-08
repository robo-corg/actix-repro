# actix-repro

Reproduces large buffer allocations for actix request payloads.

Build both bin targets:
`cargo build`

`cargo run --bin actix-repro`

## Running with cargo instruments

`cargo instruments --bin actix-repro -t Leaks --open`

Release also works:

```
cargo build --release
cargo instruments --bin actix-repro -t Leaks --open
```

## Notes

Uploader is the "driver" for the test that runs as a subprocess to make it slightly easier to see what allocations are from the server.

Driver is a child process just to make interaction with memory profilers easier.