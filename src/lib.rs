/*
 * Copyright (c) 2023 xvanc and contributors
 * SPDX-License-Identifier: BSD-3-Clause
 */

#![no_std]
#![feature(decl_macro, const_mut_refs, ptr_internals)]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

mod entry;
mod linkset_of;

pub use entry::Entry;
pub use linkset_of::LinksetOf;

use core::ops::{Deref, DerefMut};

pub macro declare_weak($name:ident) {
    ::core::arch::global_asm!(
        concat!(".weak __start___linkset_", stringify!($name)),
        concat!(".weak __stop___linkset_", stringify!($name)),
    );
}

pub macro declare($vis:vis $name:ident: $t:ty) {
    #[allow(non_upper_case_globals)]
    $vis static $name: Linkset<$t> = declare_in!($name: $t |set| set);
    declare_weak!($name);
}

/// Declare a linker set wrapped within another type
pub macro declare_in(
     $name:ident: $t:ty |$set:ident| $value:expr
) {{
    const _: () = {
        #[export_name = concat!("__linkset_set_", stringify!($name))]
        static GUARANTEE_SET_IS_UNIQUE: () = ();
    };

    #[allow(improper_ctypes)]
    extern "C" {
        #[link_name = concat!("__start___linkset_", stringify!($name))]
        static mut set_start: Entry<$t>;
        #[link_name = concat!("__stop___linkset_", stringify!($name))]
        static mut set_stop: Entry<$t>;
    }

    let $set = unsafe {
        Linkset::from_raw_parts(
            core::ptr::addr_of_mut!(set_start),
            core::ptr::addr_of_mut!(set_stop),
        )
    };

    const fn check_linkset<T: LinksetOf<$t>>(set: T) -> T {
        set
    }
    check_linkset($value)
}}

/// Declare an entry in a linker set
///
/// ```
/// entry!($set:ident: $t:ty, $value:expr);
/// ```
///
/// The set must be of the same type `$t` and must be declared with [`linkset::declare!()`].
/// `$set` must match the identifier used in `declare!()` and must be in scope (cannot be a path).
pub macro entry($set:ident, $t:ty, $value:expr) {
    const _: () = {
        #[used]
        #[link_section = concat!("__linkset_", stringify!($set))]
        static ENTRY: Entry<$t> = {
            const fn create_entry<T, S>(_: &S, value: &'static mut T) -> Entry<T>
            where
                S: LinksetOf<T>,
            {
                Entry::new(value)
            }

            create_entry(&$set, unsafe {
                #[used]
                static mut STORAGE: $t = $value;
                &mut STORAGE
            })
        };
    };
}

pub struct Linkset<T: 'static> {
    start: *mut Entry<T>,
    stop: *mut Entry<T>,
}

unsafe impl<T: Send + 'static> Send for Linkset<T> {}
unsafe impl<T: Sync + 'static> Sync for Linkset<T> {}

impl<T: 'static> Linkset<T> {
    /// Create a `Linkset` from its raw parts
    ///
    /// This function not generally intended to be used directly. Instead, linker sets should
    /// be create with the [`linkset::declare!()`] macro.
    ///
    /// # Safety
    ///
    #[inline]
    pub const unsafe fn from_raw_parts(start: *mut Entry<T>, stop: *mut Entry<T>) -> Linkset<T> {
        Self { start, stop }
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const Entry<T> {
        self.start
    }

    #[inline]
    pub const fn as_mut_ptr(&mut self) -> *mut Entry<T> {
        self.start
    }

    #[inline]
    pub const fn len(&self) -> usize {
        unsafe { self.stop.offset_from(self.start) as usize }
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn as_slice(&self) -> &'static [Entry<T>] {
        unsafe { core::slice::from_raw_parts(self.as_ptr(), self.len()) }
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &'static mut [Entry<T>] {
        unsafe { core::slice::from_raw_parts_mut(self.as_mut_ptr(), self.len()) }
    }
}

impl<T: 'static> Deref for Linkset<T> {
    type Target = [Entry<T>];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T: 'static> DerefMut for Linkset<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<T: 'static> IntoIterator for &Linkset<T> {
    type IntoIter = core::slice::Iter<'static, Entry<T>>;
    type Item = &'static Entry<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}
