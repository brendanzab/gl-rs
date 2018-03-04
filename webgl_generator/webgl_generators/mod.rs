use std::io;

use webgl_registry::Registry;

pub mod stdweb_gen;

/// Trait for a webgl bindings generator.
pub trait Generator {
    /// Builds the WebGL bindings.
    fn write<W>(&self, registry: &Registry, dest: &mut W) -> io::Result<()>
    where
        W: io::Write;
}
