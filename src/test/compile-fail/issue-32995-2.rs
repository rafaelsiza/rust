// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![deny(parenthesized_params_in_types_and_modules)]
//~^ NOTE lint level defined here
//~| NOTE lint level defined here
//~| NOTE lint level defined here
#![allow(dead_code, unused_variables)]
#![feature(conservative_impl_trait)]

fn main() {
    { fn f<X: ::std::marker()::Send>() {} }
    //~^ ERROR parenthesized parameters may only be used with a trait
    //~| WARN previously accepted
    //~| NOTE issue #42238

    { fn f() -> impl ::std::marker()::Send { } }
    //~^ ERROR parenthesized parameters may only be used with a trait
    //~| WARN previously accepted
    //~| NOTE issue #42238
}

#[derive(Clone)]
struct X;

impl ::std::marker()::Copy for X {}
//~^ ERROR parenthesized parameters may only be used with a trait
//~| WARN previously accepted
//~| NOTE issue #42238
