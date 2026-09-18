# meter-example

The `meter-example` provides a simple implementation of `meter-core` and an example.

## How to run example?

```sh
cargo run --example simple
```

The example enables metering, awaits writes, and reports a rejection if collection
fails. Its simple collector accepts every write and records usage before dispatch;
recorded usage does not imply successful persistence.
