// Copyright 2015 Brendan Zabarauskas and the gl-rs developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(unused_parens, non_camel_case_types)]
#![allow(bad_style)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::let_unit_value)]
#![allow(clippy::let_and_return)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate stdweb_derive;

mod webgl {
    include!(concat!(env!("OUT_DIR"), "/test_webgl_stdweb.rs"));
}

mod webgl2 {
    include!(concat!(env!("OUT_DIR"), "/test_webgl2_stdweb.rs"));
}
