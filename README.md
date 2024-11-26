# dsi-bitstream

[![downloads](https://img.shields.io/crates/d/sync-cell-slice)](https://crates.io/crates/sync-cell-slice)
[![dependents](https://img.shields.io/librariesio/dependents/cargo/sync-cell-slice)](https://crates.io/crates/sync-cell-slice/reverse_dependencies)
![GitHub CI](https://github.com/vigna/sync-cell-slice-rs/actions/workflows/rust.yml/badge.svg)
![license](https://img.shields.io/crates/l/sync-cell-slice)
[![](https://tokei.rs/b1/github/vigna/sync-cell-slice-rs?type=Rust,Python)](https://github.com/vigna/sync-cell-slice-rs)
[![Latest version](https://img.shields.io/crates/v/sync-cell-slice.svg)](https://crates.io/crates/sync-cell-slice)
[![Documentation](https://docs.rs/sync-cell-slice/badge.svg)](https://docs.rs/sync-cell-slice)
[![Coverage Status](https://coveralls.io/repos/github/vigna/sync-cell-slice-rs/badge.svg?branch=main)](https://coveralls.io/github/vigna/sync-cell-slice-rs?branch=main)

Cells and slices that are accessible from multiple threads.

Sometimes, multiple threads needs to access a place, or an element of a slice,
without the need of the guarantees of atomic operations, because the absence of
data races is guaranteed externally (e.g., each threads writes to a different
element of the slice).

This small crate implements a solution based on a [`SyncCell<T>`] newtype with
base type [`Cell<T>`]. Contrarily to [`Cell<T>`], [`SyncCell<T>`] can be shared
among threads, as long as its content can be shared, too. This result is
obtained by forcing [`Sync`] on [`SyncCell<T>`] if `T` is.

All access methods are unsafe, because lack of external synchronization might
lead to data races, and thus to undefined behavior. Note that this approach is
radically different from that of [`SyncUnsafeCell`], all of which methods are
safe.

An important advantage of using [`Cell`] instead of [`UnsafeCell`] as base type is
that we can use the [`Cell::as_slice_of_cells`] method to make [`SyncCell`] and
slices commute, that is, to obtain (safely) from a reference to `SyncCell<[T]>`
a reference to `[SyncCell<T>]`. Since [`SyncCell<T>`] is `Sync` if `T` is,
`[SyncCell<T>]` is `Sync` if `T` is, too. Then, if `T` is `Sync` sharing a slice
 of `T` is just a matter of wrapping the slice in a `SyncCell` and calling
[`SyncCell::as_slice_of_cells`]. This process is carried out by the extension
trait [`SyncSlice`], which add to slices a method `as_sync_slice`.

The design is based on suggestions contained in a [post by Alice
 Ryhl](https://stackoverflow.com/questions/65178245/how-do-i-write-to-a-mutable-slice-from-multiple-threads-at-arbitrary-indexes-wit/65182786#65182786)
and in this [thread on the Rust Language
Forum](https://users.rust-lang.org/t/parallel-interior-mutability/121542/7).

[`Cell<T>`]: <https://doc.rust-lang.org/std/cell/struct.Cell.html>
[`Cell`]: <https://doc.rust-lang.org/std/cell/struct.Cell.html>
[`UnsafeCell`]: <https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html>
[`Cell::as_slice_of_cells`]: <https://doc.rust-lang.org/std/cell/struct.Cell.html#method.as_slice_of_cells>
[`SyncCell<T>`] <https://docs.rs/sync-cell-slice/latest/sync_cell_slice/struct.SyncCell.html>
[`SyncCell`]:
    <https://docs.rs/sync-cell-slice/latest/sync_cell_slice/struct.SyncCell.html>
[`SyncCell::as_slice_of_cells`]: <https://docs.rs/sync-cell-slice/latest/sync_cell_slice/struct.SyncCell.html#method.as_slice_of_cells>
[`SyncSlice`]: <https://docs.rs/sync-cell-slice/latest/sync_cell_slice/trait.SyncSlice.html>
[`Sync`]: <https://doc.rust-lang.org/std/marker/trait.Sync.html>
[`SyncUnsafeCell`]: <<https://doc.rust-lang.org/std/cell/struct.SyncUnsafeCell.html>
