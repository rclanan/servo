/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#![feature(default_type_params,macro_rules,unsafe_destructor)]

#![deny(unused_imports, unused_variable)]

#![feature(phase)]
#[phase(plugin, link)]
extern crate log;

extern crate debug;
extern crate alloc;
extern crate collections;
extern crate geom;
extern crate getopts;
extern crate layers;
extern crate libc;
extern crate native;
extern crate rand;
extern crate rustrt;
extern crate serialize;
extern crate sync;
#[cfg(target_os="macos")]
extern crate task_info;
extern crate "time" as std_time;
extern crate string_cache;
extern crate url;

#[phase(plugin)]
extern crate string_cache_macros;

use std::sync::Arc;

pub mod bloom;
pub mod cache;
pub mod debug_utils;
pub mod dlist;
pub mod fnv;
pub mod geometry;
pub mod logical_geometry;
pub mod memory;
pub mod namespace;
pub mod opts;
pub mod range;
pub mod resource_files;
pub mod smallvec;
pub mod sort;
pub mod str;
pub mod task;
pub mod tid;
pub mod time;
pub mod taskpool;
pub mod vec;
pub mod workqueue;

pub fn breakpoint() {
    unsafe { ::std::intrinsics::breakpoint() };
}

// Workaround for lack of `ptr_eq` on Arcs...
#[inline]
pub fn arc_ptr_eq<T: 'static + Send + Sync>(a: &Arc<T>, b: &Arc<T>) -> bool {
    let a: &T = a.deref();
    let b: &T = b.deref();
    (a as *const T) == (b as *const T)
}
