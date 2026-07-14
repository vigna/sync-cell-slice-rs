# Change Log

## [0.9.13] - 2026-07-14

### New

* The crate is now `no_std`.

* `SyncCell` implements now an opaque `Debug`.

### Changed

* The safety documentation of `get`, `set`, `swap`, `replace`, and `take`
  now specifies that moving, copying, or dropping values from a thread
  that does not own them additionally requires `T: Send`.

### Improved

* Compile-time assertions pin down the `Send`/`Sync` surface.

* CI now runs clippy on all targets, tests under Miri, and checks the
  minimum supported Rust version.

## [0.9.12] - 2026-02-15

### Changed

* 2024 edition, Rust 1.85

* `SyncCell::as_ptr` is no longer unsafe, as you need
  in any case unsafe code to dereference the pointer.

## [0.9.11] - 2024-11-27

### Improved

* More docs.

## [0.9.10] - 2024-11-27

### Improved

* More docs.

## [0.9.9] - 2024-11-26

### New

* First release.
