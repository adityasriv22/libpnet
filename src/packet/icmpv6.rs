// Copyright (c) 2015 Robert Clipsham <robert@octarineparrot.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! ICMPv6 packet abstraction

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/icmpv6.rs"));

#[cfg(not(feature = "with-syntex"))]
include!("icmpv6.rs.in");
