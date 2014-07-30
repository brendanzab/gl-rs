// Copyright 2013-2014 The gl-rs developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
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

#![crate_name = "gen"]
#![comment = "OpenGL function loader generator."]
#![license = "ASL2"]

#![feature(globs)]
#![feature(macro_rules)]
#![feature(phase)]

extern crate getopts;

#[phase(plugin, link)]
extern crate log;

use getopts::{optopt, optmulti, optflag, getopts, usage};

use std::os;
use std::path::Path;
use std::io;
use std::io::{File, Reader};

use registry::*;

pub mod generator;
pub mod registry;
pub mod ty;

fn main() {
    let opts = &[
        optopt("", "namespace", "OpenGL namespace (gl by default)", "gl|glx|wgl"),
        optopt("", "api", "API to generate bindings for (gl by default)", "gl|gles1|gles2"),
        optopt("", "profile", "Profile to generate (core by default)", "core|compatability"),
        optopt("", "version", "Version to generate bindings for (4.3 by default)", ""),
        optmulti("", "extension", "Extension to include", ""),
        optflag("h", "help", "Print usage information"),
        optflag("", "full", "Generate API for all profiles, versions and extensions"),
        optopt("", "xml", "The xml spec file (<namespace>.xml by default)", ""),
    ];

    let os_args = os::args().iter().map(|x| x.to_string()).collect::<Vec<String>>();
    let args = match getopts(os_args.as_slice(), opts) {
        Ok(a) => a,
        Err(x) => fail!("Error: {}\n{}", x, usage("glrsgen", opts)),
    };

    if args.opt_present("help") {
        println!("{}", usage("glrsgen", opts));
        return;
    }

    let ns = match args.opt_str("namespace").unwrap_or("gl".to_string()).as_slice() {
        "gl"  => Gl,
        "glx" => fail!("glx generation unimplemented"),
        "wgl" => fail!("wgl generation unimplemented"),
        ns     => fail!("Unexpected opengl namespace '{}'", ns)
    };

    let path = Path::new(
        args.opt_str("xml").unwrap_or(format!("{}.xml", ns))
    );

    let filter = if args.opt_present("full") {
        None
    } else {
        Some(Filter {
            extensions: args.opt_strs("extension"),
            profile: args.opt_str("profile").unwrap_or("core".to_string()),
            version: args.opt_str("version").unwrap_or("4.3".to_string()),
            api: args.opt_str("api").unwrap_or("gl".to_string()),
        })
    };

    let reg = Registry::from_xml(
        File::open(&path).ok()
            .expect(format!("Could not read {}", path.display()).as_slice())
            .read_to_string().ok()
            .expect( "registry source not utf8!" ).as_slice(), ns, filter
    );

    generator::Generator::write(&mut io::stdout(), &reg, ns, true);
}
