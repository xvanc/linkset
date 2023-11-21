/*
 * Copyright (c) 2023 xvanc and contributors
 * SPDX-License-Identifier: BSD-3-Clause
 */

use core::{
    borrow::{Borrow, BorrowMut},
    cmp::Ordering,
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Deref, DerefMut},
    ptr::Unique,
};

#[repr(transparent)]
pub struct Entry<T> {
    pub(crate) value: Unique<T>,
}

impl<T> Entry<T> {
    pub const fn new(value: &'static mut T) -> Entry<T> {
        Self {
            value: unsafe { Unique::new_unchecked(value) },
        }
    }

    pub const fn as_ref(&self) -> &'static T {
        unsafe { &*self.value.as_ptr() }
    }

    pub const fn as_mut(&mut self) -> &'static mut T {
        unsafe { &mut *self.value.as_ptr() }
    }
}

unsafe impl<T: Send> Send for Entry<T> {}
unsafe impl<T: Sync> Sync for Entry<T> {}

impl<T> Deref for Entry<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.value.as_ref() }
    }
}

impl<T> DerefMut for Entry<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.value.as_mut() }
    }
}

impl<T> AsMut<T> for Entry<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        &mut *self
    }
}

impl<T> AsRef<T> for Entry<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T> Borrow<T> for Entry<T> {
    #[inline]
    fn borrow(&self) -> &T {
        self
    }
}

impl<T> BorrowMut<T> for Entry<T> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut T {
        self
    }
}

impl<T: Debug> Debug for Entry<T> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        T::fmt(self, f)
    }
}

impl<T: Display> Display for Entry<T> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        T::fmt(self, f)
    }
}

impl<T: Hash> Hash for Entry<T> {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        T::hash(self, state)
    }
}

impl<T: PartialEq> PartialEq for Entry<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&**self, &**other)
    }
}

impl<T: Eq> Eq for Entry<T> {}

impl<T: PartialOrd> PartialOrd for Entry<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&**self, &**other)
    }
    #[inline]
    fn lt(&self, other: &Self) -> bool {
        PartialOrd::lt(&**self, &**other)
    }
    #[inline]
    fn le(&self, other: &Self) -> bool {
        PartialOrd::le(&**self, &**other)
    }
    #[inline]
    fn ge(&self, other: &Self) -> bool {
        PartialOrd::ge(&**self, &**other)
    }
    #[inline]
    fn gt(&self, other: &Self) -> bool {
        PartialOrd::gt(&**self, &**other)
    }
}

impl<T: Ord> Ord for Entry<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&**self, &**other)
    }
}
