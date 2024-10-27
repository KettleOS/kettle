//! Basic synchronization primitives.
//!
//! Documentation and source code often copied from <https://doc.rust-lang.org/std/sync>.
#![allow(unused)]

#[cfg(target_arch = "aarch64")]
#[path = "_arch/aarch64/sync.rs"]
mod inner;

use core::cell::UnsafeCell;

/// Any synchronization primitive that implements a Mutex lock.
/// Implementors guarantee that locks granted give exclusive access for the duration of its existence.
pub trait MutexLock<T: ?Sized> {
	/// Gives the closure temporary, exclusive read-write access to the data.
	fn lock<'a, R>(&self, f: impl FnOnce(&'a mut T) -> R) -> R where T: 'a;
}

/// A read-write exclusive lock.
///
/// Implementors guarantee that either many reader exist or one writer exists.
pub trait RwExLock<T: ?Sized> {
	/// Gives the closure temporary, exclusive read-write access to the data.
	fn write<'a, R>(&self, f: impl FnOnce(&'a mut T) -> R) -> R where T: 'a;

	/// Gives the closure temporary, exclusive read access to the data.
	fn read<'a, R>(&self, f: impl FnOnce(&'a T) -> R) -> R where T: 'a;
}

/// A single-threaded [RwExLock] used during kernel initialization.
pub struct KernelInitLock<T: ?Sized> {
	data: UnsafeCell<T>,
}

impl<T> KernelInitLock<T> {
	/// Constructs a new [NullLock] that is unlocked.
	pub fn new(data: T) -> Self {
		Self {
			data: UnsafeCell::new(data),
		}
	}
}

impl<T: ?Sized> RwExLock<T> for KernelInitLock<T> {
	fn write<'a, R>(&self, f: impl FnOnce(&'a mut T) -> R) -> R where T: 'a {
		// SAFETY:
		// The data will only be accessed in an exclusive context.
		let data = unsafe { &mut *self.data.get() };

		f(data)
	}

	fn read<'a, R>(&self, f: impl FnOnce(&'a T) -> R) -> R where T: 'a {
		// SAFETY:
		// The data will only be accessed in an exclusive context.
		let data = unsafe { &*self.data.get() };

		f(data)
	}
}
