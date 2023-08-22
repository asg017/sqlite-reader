# `sqlite-reader`

A small collection of shared Rust traits + names for SQLite extensions created with [`sqlite-loadable-rs`](https://github.com/asg017/sqlite-loadable-rs/).

These types allow for extensions to take "readers" (or streams) as inputs, to get around the 1GB SQLite value limit, and to stream large files/URLs without taking up much memory.

Might deprecate this repo in the future, and directly include with `sqlite-loadable-rs` or something in the future.
