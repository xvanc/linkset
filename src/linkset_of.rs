/*
 * Copyright (c) 2023 xvanc and contributors
 * SPDX-License-Identifier: BSD-3-Clause
 */

use crate::Linkset;

/// Types which wrap a [`Linkset<T>`]
///
/// # Safety
pub unsafe trait LinksetOf<T: 'static> {}

unsafe impl<T: 'static> LinksetOf<T> for Linkset<T> {}

#[cfg(feature = "std")]
mod std_impls {
    use crate::{Linkset, LinksetOf};

    unsafe impl<T> LinksetOf<T> for std::sync::Mutex<Linkset<T>> {}
    unsafe impl<T> LinksetOf<T> for std::sync::RwLock<Linkset<T>> {}
}

#[cfg(feature = "spin")]
mod std_impls {
    use crate::{Linkset, LinksetOf};

    unsafe impl<T> LinksetOf<T> for spin::Mutex<Linkset<T>> {}
    unsafe impl<T> LinksetOf<T> for spin::RwLock<Linkset<T>> {}
}
