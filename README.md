# greptime-meter

## OverView

The `greptime-meter` provides an abstraction about data read/write computation and collection. It consists of the following crates:

- meter-core: provides some core traits and data structures.
- meter-macros: provides some macros for user convenience, include `write_meter!` etc.
- meter-example: provides a simple implementation of `meter-core` and an example.

## Documentation

```shell
cargo doc --open --no-deps
```

## Write admission migration

- `MeterRecord::new(catalog, schema, value, rows, source)` now takes an explicit
  insertion row count. Read records use `rows = 0`; `value` keeps its calculated
  metering meaning (for example, WCU/RCU).
- Implement `Collect::on_write` with a boxed `Send` future returning
  `Result<(), meter_core::collect::WriteRejected>`. Reject before recording
  accepted usage. `on_read` remains synchronous.
- Await `write_meter!(catalog, schema, requests, rows, source)` and handle its
  rejection before dispatch. It returns the calculated value on success. Missing
  calculators still submit the row count with `value = 0`.
- For bulk writes, await `write_meter!(record)` with a precomputed `MeterRecord`.
  This uses the same `Registry::record_write` operation without a calculator.
- `meter-macros` defaults to `noop`: both write forms return `Ok(0)` without
  evaluating arguments, including row-count scans. Set `default-features = false`
  to enable collection. Direct registry calls always invoke registered collectors.

Collection happens before write dispatch. Success means admission and recording,
not successful persistence. The registry neither implements quota policy nor
refunds usage after a downstream failure. No registered collector permits writes.

Downstream integration is a manual checkpoint: update **both `meter-core` and
`meter-macros`** to the same intended revision and regenerate the consumer's
lockfile. Publishing and downstream dependency changes are separate steps.

## Validation

Run enabled and noop configurations separately to avoid feature unification
hiding the enabled macro implementation:

```sh
cargo check --workspace --all-targets
cargo check -p meter-macros -p meter-example --all-targets --no-default-features
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo clippy -p meter-macros -p meter-example --all-targets --no-default-features -- -D warnings
cargo nextest run --workspace
cargo nextest run -p meter-macros --no-default-features
cargo nextest run -p meter-macros --features noop
cargo test --doc --workspace
cargo test --doc -p meter-macros --no-default-features
```
