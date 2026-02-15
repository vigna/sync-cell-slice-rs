/*
 * SPDX-FileCopyrightText: 2024 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

#![doc = include_str!("../README.md")]

use std::cell::Cell;

/// A mutable memory location that is [`Sync`].
///
/// # Memory layout
///
/// `SyncCell<T>` has the same memory layout and caveats as [`Cell<T>`], but it
/// is [`Sync`] if `T` is. In particular, since [`Cell<T>`] has the same
/// in-memory representation as its inner type `T`, `SyncCell<T>`, too, has the
/// same in-memory representation as its inner type `T`. `SyncCell<T>` is also
/// [`Send`] if [`Cell<T>`] is [`Send`].
///
/// `SyncCell<T>` is useful when you need to share a mutable memory location
/// across threads, and you rely on the fact that the intended behavior will not
/// cause data races. For example, the content will be written once and then
/// read many times, in this order.
///
/// The main goal of `SyncCell<T>` is to make it possible to write to
/// different locations of a slice in parallel, leaving the control of data
/// races to the user, without the access cost of an atomic variable. For this
/// purpose, `SyncCell` implements the
/// [`as_slice_of_cells`](SyncCell::as_slice_of_cells) method, which turns a
/// `&SyncCell<[T]>` into a `&[SyncCell<T>]`, similar to the [analogous method
/// of `Cell`](Cell::as_slice_of_cells).
///
/// Since this is the most common usage, the extension trait [`SyncSlice`] adds
/// to slices a method [`as_sync_slice`](SyncSlice::as_sync_slice) that turns a
/// `&mut [T]` into a `&[SyncCell<T>]`.
///
/// # Methods
///
/// `SyncCell` painstakingly reimplements the methods of [`Cell`] as unsafe,
/// since they rely on external synchronization mechanisms to avoid undefined
/// behavior.
///
/// `SyncCell` implements also a few traits implemented by [`Cell`] by
/// delegation for convenience, but some, such as [`Clone`] or [`PartialOrd`],
/// cannot be implemented because they would use unsafe methods.
///
/// # Safety
///
/// Multiple threads can read from and write to the same `SyncCell` at the same
/// time. It is the responsibility of the user to ensure that there are no data
/// races, which would cause undefined behavior.
///
/// # Examples
///
/// In this example, you can see that `SyncCell` enables mutation across
/// threads:
///
/// ```
/// use sync_cell_slice::SyncCell;
/// use sync_cell_slice::SyncSlice;
///
/// let x = 0;
/// let c = SyncCell::new(x);
///
/// let mut v = vec![1, 2, 3, 4];
/// let s = v.as_sync_slice();
///
/// std::thread::scope(|scope| {
///     scope.spawn(|| {
///         // You can use interior mutability in another thread
///         unsafe { c.set(5) };
///     });
///
///     scope.spawn(|| {
///         // You can use interior mutability in another thread
///         unsafe { s[0].set(5) };
///     });
///     scope.spawn(|| {
///         // You can use interior mutability in another thread
///         // on the same slice
///         unsafe { s[1].set(10) };
///     });
/// });
/// ```
///
/// In this example, we invert a permutation in parallel:
///
/// ```
/// use sync_cell_slice::SyncCell;
/// use sync_cell_slice::SyncSlice;
///
/// let mut perm = vec![0, 2, 3, 1];
/// let mut inv = vec![0; perm.len()];
/// let inv_sync = inv.as_sync_slice();
///
/// std::thread::scope(|scope| {
///     scope.spawn(|| { // Invert first half
///         for i in 0..2 {
///             unsafe { inv_sync[perm[i]].set(i) };
///         }
///     });
///
///     scope.spawn(|| { // Invert second half
///         for i in 2..perm.len() {
///             unsafe { inv_sync[perm[i]].set(i) };
///         }
///     });
/// });
///
/// assert_eq!(inv, vec![0, 3, 1, 2]);
/// ```
#[repr(transparent)]
pub struct SyncCell<T: ?Sized>(Cell<T>);

// This is where we depart from Cell.
unsafe impl<T: ?Sized> Send for SyncCell<T> where Cell<T>: Send {}
unsafe impl<T: ?Sized + Sync> Sync for SyncCell<T> {}

impl<T> SyncCell<T> {
    /// Creates a new `SyncCell` containing the given value.
    #[inline]
    pub const fn new(value: T) -> Self {
        Self(Cell::new(value))
    }

    /// Sets the contained value by delegation to [`Cell::set`].
    ///
    /// # Safety
    ///
    /// Multiple threads can read from and write to the same `SyncCell` at the
    /// same time. It is the responsibility of the user to ensure that there are no
    /// data races, which would cause undefined behavior.
    #[inline]
    pub unsafe fn set(&self, val: T) {
        self.0.set(val);
    }

    /// Swaps the values of two `SyncCell`s by delegation to [`Cell::swap`].
    ///
    /// # Safety
    ///
    /// Multiple threads can read from and write to the same `SyncCell` at the
    /// same time. It is the responsibility of the user to ensure that there are no
    /// data races, which would cause undefined behavior.
    #[inline]
    pub unsafe fn swap(&self, other: &SyncCell<T>) {
        self.0.swap(&other.0);
    }

    /// Replaces the contained value with `val`, and returns the old contained
    /// value by delegation to [`Cell::replace`].
    ///
    /// # Safety
    ///
    /// Multiple threads can read from and write to the same `SyncCell` at the
    /// same time. It is the responsibility of the user to ensure that there are no
    /// data races, which would cause undefined behavior.
    #[inline]
    pub unsafe fn replace(&self, val: T) -> T {
        self.0.replace(val)
    }

    /// Unwraps the value, consuming the cell.
    #[inline]
    pub fn into_inner(self) -> T {
        self.0.into_inner()
    }
}

impl<T: Copy> SyncCell<T> {
    /// Returns a copy of the contained value by delegation to [`Cell::get`].
    ///
    /// # Safety
    ///
    /// Multiple threads can read from and write to the same `SyncCell` at the
    /// same time. It is the responsibility of the user to ensure that there are no
    /// data races, which would cause undefined behavior.
    #[inline]
    pub unsafe fn get(&self) -> T {
        self.0.get()
    }
}

impl<T: ?Sized> SyncCell<T> {
    /// Returns a raw pointer to the underlying data in this cell
    /// by delegation to [`Cell::as_ptr`].
    ///
    /// Multiple threads can read from and write to the same [`SyncCell`] at the
    /// same time. It is the responsibility of the user to ensure that there are no
    /// data races, which might lead to undefined behavior.
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut T {
        self.0.as_ptr()
    }

    /// Returns a mutable reference to the underlying data by delegation to
    /// [`Cell::get_mut`].
    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        self.0.get_mut()
    }

    /// Returns a `&SyncCell<T>` from a `&mut T`.
    #[allow(trivial_casts)]
    #[inline]
    pub fn from_mut(value: &mut T) -> &Self {
        // SAFETY: `Cell::from_mut` converts `&mut T` to `&Cell<T>`, and
        // `SyncCell<T>` has the same memory layout as `Cell<T>` due to
        // `#[repr(transparent)]`.
        unsafe { &*(Cell::from_mut(value) as *const Cell<T> as *const Self) }
    }
}

impl<T: Default> SyncCell<T> {
    /// Takes the value of the cell, leaving [`Default::default`] in its place.
    ///
    /// # Safety
    ///
    /// Multiple threads can read from and write to the same `SyncCell` at the
    /// same time. It is the responsibility of the user to ensure that there are no
    /// data races, which would cause undefined behavior.
    #[inline]
    pub unsafe fn take(&self) -> T {
        self.0.take()
    }
}

#[allow(trivial_casts)]
impl<T> SyncCell<[T]> {
    /// Returns a `&[SyncCell<T>]` from a `&SyncCell<[T]>`.
    #[inline]
    pub fn as_slice_of_cells(&self) -> &[SyncCell<T>] {
        let slice_of_cells = self.0.as_slice_of_cells();
        // SAFETY: `SyncCell<T>` has the same memory layout as `Cell<T>`
        // due to `#[repr(transparent)]`.
        unsafe { &*(slice_of_cells as *const [Cell<T>] as *const [SyncCell<T>]) }
    }
}

impl<T: Default> Default for SyncCell<T> {
    /// Creates a `SyncCell<T>`, with the `Default` value for `T`.
    #[inline]
    fn default() -> SyncCell<T> {
        SyncCell::new(Default::default())
    }
}

impl<T> From<T> for SyncCell<T> {
    /// Creates a new `SyncCell` containing the given value.
    fn from(value: T) -> SyncCell<T> {
        SyncCell::new(value)
    }
}

/// Extension trait turning a `&mut [T]` into a `&[SyncCell<T>]`.
///
/// The result is [`Sync`] if `T` is [`Sync`].
pub trait SyncSlice<T> {
    /// Returns a `&[SyncCell<T>]` from a `&mut [T]`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sync_cell_slice::SyncSlice;
    ///
    /// let mut v = vec![1, 2, 3, 4];
    /// // s can be used to write to v from multiple threads
    /// let s = v.as_sync_slice();
    ///
    /// std::thread::scope(|scope| {
    ///     scope.spawn(|| {
    ///         unsafe { s[0].set(5) };
    ///     });
    ///     scope.spawn(|| {
    ///         unsafe { s[1].set(10) };
    ///     });
    /// });
    /// ```
    fn as_sync_slice(&mut self) -> &[SyncCell<T>];
}

impl<T> SyncSlice<T> for [T] {
    fn as_sync_slice(&mut self) -> &[SyncCell<T>] {
        SyncCell::from_mut(self).as_slice_of_cells()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_into_inner() {
        let c = SyncCell::new(42);
        assert_eq!(c.into_inner(), 42);
    }

    #[test]
    fn test_set_and_get() {
        let c = SyncCell::new(0);
        unsafe { c.set(10) };
        assert_eq!(unsafe { c.get() }, 10);
    }

    #[test]
    fn test_swap() {
        let a = SyncCell::new(1);
        let b = SyncCell::new(2);
        unsafe { a.swap(&b) };
        assert_eq!(unsafe { a.get() }, 2);
        assert_eq!(unsafe { b.get() }, 1);
    }

    #[test]
    fn test_replace() {
        let c = SyncCell::new(5);
        let old = unsafe { c.replace(10) };
        assert_eq!(old, 5);
        assert_eq!(unsafe { c.get() }, 10);
    }

    #[test]
    fn test_take() {
        let c = SyncCell::new(42);
        let val = unsafe { c.take() };
        assert_eq!(val, 42);
        assert_eq!(unsafe { c.get() }, 0);
    }

    #[test]
    fn test_get_mut() {
        let mut c = SyncCell::new(3);
        *c.get_mut() = 7;
        assert_eq!(unsafe { c.get() }, 7);
    }

    #[test]
    fn test_as_ptr() {
        let c = SyncCell::new(99);
        let ptr = c.as_ptr();
        assert_eq!(unsafe { *ptr }, 99);
    }

    #[test]
    fn test_from_mut() {
        let mut val = 10;
        let c = SyncCell::from_mut(&mut val);
        unsafe { c.set(20) };
        assert_eq!(val, 20);
    }

    #[test]
    fn test_default() {
        let c: SyncCell<i32> = SyncCell::default();
        assert_eq!(unsafe { c.get() }, 0);
    }

    #[test]
    fn test_from() {
        let c: SyncCell<i32> = SyncCell::from(42);
        assert_eq!(unsafe { c.get() }, 42);
    }

    #[test]
    fn test_as_slice_of_cells() {
        let mut v = [1, 2, 3];
        let sync_slice = v.as_sync_slice();
        assert_eq!(sync_slice.len(), 3);
        assert_eq!(unsafe { sync_slice[0].get() }, 1);
        assert_eq!(unsafe { sync_slice[1].get() }, 2);
        assert_eq!(unsafe { sync_slice[2].get() }, 3);
    }

    #[test]
    fn test_sync_slice_mutation() {
        let mut v = vec![0; 4];
        let sync_slice = v.as_sync_slice();

        std::thread::scope(|scope| {
            for (i, cell) in sync_slice.iter().enumerate() {
                scope.spawn(move || {
                    unsafe { cell.set(i * 10) };
                });
            }
        });

        assert_eq!(v, vec![0, 10, 20, 30]);
    }
}
