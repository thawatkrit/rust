// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(rustc_attrs)]

use std::fmt;

// CoerceUnsized is not implemented for tuples. You can still create
// an unsized tuple by transmuting a trait object.
fn any<T>() -> T { unreachable!() }

#[rustc_error]
fn main() { //~ ERROR compilation successful
    let t: &(u8, fmt::Debug) = any();
    println!("{:?}", &t.1);
}
