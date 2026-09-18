# meter-core

The `meter-core` crate provides some core traits and data structures about data write/read computation and collection.

`MeterRecord` carries database identity, calculated `value`, insertion `rows`, and
`source`. Read records set `rows` to zero. `ItemCalculator<T>::calc` still computes
only the metering value.

`Collect::on_write` returns a boxed `Send` future with `Result<(), WriteRejected>`;
`WriteRejected` exposes the rejection reason and implements `std::error::Error`.
Collectors must reject before recording accepted usage. `Collect::on_read` remains
synchronous.

Await `Registry::record_write(record)` to submit calculated or precomputed usage.
It clones the collector under the lock, releases the lock before collection, and
propagates rejection. With no collector it succeeds. It has no quota or rollback
policy: collection precedes dispatch and does not guarantee persistence.
