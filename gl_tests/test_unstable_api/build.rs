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

extern crate gl_generator;

use gl_generator::generators;

fn main() {
    let _ = generators::gen_struct_name;
    let _ = generators::gen_enum_item::<Vec<u8>>;
    let _ = generators::gen_types::<Vec<u8>>;
    let _ = generators::gen_parameters;
    let _ = generators::gen_symbol_name;
}
