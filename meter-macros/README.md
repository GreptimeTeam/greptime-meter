# meter-macros

`write_meter!` supports calculated and precomputed writes:

```rust,ignore
let cost = write_meter!(catalog, schema, requests, rows, source).await?;
// Dispatch requests only after admission succeeds.

let cost = write_meter!(MeterRecord::new(catalog, schema, 0, rows, source)).await?;
```

Both forms return `Result<u64, meter_core::collect::WriteRejected>` when awaited.
A missing calculator uses zero value but still invokes admission with the supplied
row count. Requests are borrowed for calculation; arguments are evaluated once.
Collection happens before dispatch, so success does not guarantee persistence.

The default `noop` feature returns `Ok(0)` without evaluating write arguments or
invoking collectors. Row-count expressions and precomputed records are not built.
Use `default-features = false` on this dependency to enable collection.

`read_meter!(catalog, schema, item, source)` remains synchronous, returning the
calculated value and recording `rows = 0` when enabled and a calculator exists.
