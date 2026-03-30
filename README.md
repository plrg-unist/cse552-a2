# CSE552 Assignment 2: Dataflow Analysis

Implement four dataflow analyses for a subset of Rust:

* Live variable analysis
* Available expression analysis
* Very busy expression analysis
* Reaching definition analysis

## Requirements

- [rustup](https://rustup.rs/)

## Usage

```
cargo test  # run tests
```

## Files

- `src/analysis.rs` — **the only file you need to change and submit**
- `src/expr.rs` — definitions used by the analysis
- `src/tests.rs` — tests
