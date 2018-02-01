# webgl_generator

[![Version](https://img.shields.io/crates/v/webgl_generator.svg)](https://crates.io/crates/webgl_generator)
[![License](https://img.shields.io/crates/l/webgl_generator.svg)](https://github.com/brendanzab/gl-rs/blob/master/LICENSE)
[![Downloads](https://img.shields.io/crates/d/webgl_generator.svg)](https://crates.io/crates/webgl_generator)

Code generators for creating bindings to the WebGL APIs.

## Usage

See `tests/test_webgl_stdweb` for an example of how to use these generators.

## Generator types

### Stdweb generator

The stdweb generator is currently the only supported webgl generator. This generator
uses `stdweb` to bind the relevant javascript APIs, and integrates with the `stdweb`
`RenderingContext` trait.
