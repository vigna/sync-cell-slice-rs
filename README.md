# Sync cells and slices

[![downloads](https://img.shields.io/crates/d/sync-cell-slice)](https://crates.io/crates/sync-cell-slice)
[![dependents](https://img.shields.io/librariesio/dependents/cargo/sync-cell-slice)](https://crates.io/crates/sync-cell-slice/reverse_dependencies)
![license](https://img.shields.io/crates/l/sync-cell-slice)
[![](https://tokei.rs/b1/github/vigna/sync-cell-slice-rs?type=Rust)](https://github.com/vigna/sync-cell-slice-rs)
[![Latest version](https://img.shields.io/crates/v/sync-cell-slice.svg)](https://crates.io/crates/sync-cell-slice)
[![Documentation](https://docs.rs/sync-cell-slice/badge.svg)](https://docs.rs/sync-cell-slice)

Sometimes, multiple threads need to access a place or an element of a slice
without atomic operations because the absence of data races is guaranteed
externally (e.g., each thread writes to a different element of the slice).

This small crate implements a solution based on a [`SyncCell<T>`] newtype with
base type [`Cell<T>`]. Contrarily to [`Cell<T>`], [`SyncCell<T>`] can be shared
among threads, as long as its content can be shared, too. This result is
obtained by forcing [`Sync`] on [`SyncCell<T>`] if `T` is [`Sync`].

All access methods are unsafe, because lack of external synchronization might
lead to data races, and thus to undefined behavior. Note that this approach is
radically different from that of [`SyncUnsafeCell`], all of which methods are
safe.

An important advantage of using [`Cell`] instead of [`UnsafeCell`] as base type
is that we can use the [`Cell::as_slice_of_cells`] method to implement an
analogous method for [`SyncCell`] that makes it possible to make [`SyncCell`]
 and slices commute, that is, to obtain (safely) from a `&SyncCell<[T]>` a
`&[SyncCell<T>]`. Since [`SyncCell<T>`] is [`Sync`] if `T` is, `[SyncCell<T>]`
is [`Sync`] if `T` is, too. Thus, if `T` is [`Sync`] sharing a slice of `T`
among threads is just a matter of wrapping the slice in a [`SyncCell`] and
 calling [`SyncCell::as_slice_of_cells`]. This process is carried out by the
extension trait [`SyncSlice`], which adds to slices a method [`as_sync_slice`].

The design is based on suggestions in a [post by Alice
Ryhl](https://stackoverflow.com/questions/65178245/how-do-i-write-to-a-mutable-slice-from-multiple-threads-at-arbitrary-indexes-wit/65182786#65182786)
and in this [thread on the Rust Language
Forum](https://users.rust-lang.org/t/parallel-interior-mutability/121542).

## Acknowledgments

This software has been partially supported by project SERICS (PE00000014) under
the NRRP MUR program funded by the EU - NGEU. Views and opinions expressed are
however those of the authors only and do not necessarily reflect those of the
European Union or the Italian MUR. Neither the European Union nor the Italian
MUR can be held responsible for them.

[`SyncUnsafeCell`]: <https://doc.rust-lang.org/std/cell/struct.SyncUnsafeCell.html>
[`as_sync_slice`]: <https://docs.rs/sync-cell-slice/latest/sync_cell_slice/trait.SyncSlice.html#tymethod.as_sync_slice>
[`Cell<T>`]: <https://doc.rust-lang.org/std/cell/struct.Cell.html>
[`Cell`]: <https://doc.rust-lang.org/std/cell/struct.Cell.html>
[`UnsafeCell`]: <https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html>
[`Cell::as_slice_of_cells`]: <https://doc.rust-lang.org/std/cell/struct.Cell.html#method.as_slice_of_cells>
[`SyncCell`]: <https://docs.rs/sync-cell-slice/latest/sync_cell_slice/struct.SyncCell.html>
[`SyncCell::as_slice_of_cells`]: <https://docs.rs/sync-cell-slice/latest/sync_cell_slice/struct.SyncCell.html#method.as_slice_of_cells>
[`SyncSlice`]: <https://docs.rs/sync-cell-slice/latest/sync_cell_slice/trait.SyncSlice.html>
[`Sync`]: <https://doc.rust-lang.org/std/marker/trait.Sync.html>
[`SyncCell<T>`]: <https://docs.rs/sync-cell-slice/latest/sync_cell_slice/struct.SyncCell.html>
