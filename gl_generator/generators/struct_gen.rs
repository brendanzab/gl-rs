use crate::registry::Registry;
use std::io;

#[derive(Debug)]
pub struct StructGenerator {
    /// If `true`, the bindings will import a `trace!` macro from the parent
    /// module and invoke it with the name of the function about to be called
    /// before actually calling each GL function.
    pub trace: bool,
    /// If `true`, the bindings will import an `error!` macro from the parent
    /// module and, when `debug_assertions` are configured for the build, will
    /// check after each GL call if there was an error. If an error is detected,
    /// the `error!` macro will be invoked with a format string and args to
    /// format.
    pub debug_assert_error_check: bool,
}
impl Default for StructGenerator {
    fn default() -> Self {
        Self {
            trace: true,
            debug_assert_error_check: true,
        }
    }
}

impl super::Generator for StructGenerator {
    fn write(&self, registry: &Registry, dest: &mut dyn io::Write) -> io::Result<()>
    {
        write_header(registry, dest, self)?;
        super::write_type_aliases(registry, dest)?;
        super::write_enums(registry, dest)?;
        write_struct(registry, dest)?;
        write_impl(registry, dest, self.trace, self.debug_assert_error_check)?;
        Ok(())
    }
}

fn write_header(registry: &Registry, dest: &mut dyn io::Write, gen: &StructGenerator) -> io::Result<()>
{
    writeln!(dest, "#![allow(bad_style)]")?;
    writeln!(dest, "#![allow(clippy::unreadable_literal)]")?;
    writeln!(dest, "#![allow(clippy::missing_safety_doc)]")?;
    writeln!(dest, "#![allow(clippy::too_many_arguments)]")?;
    writeln!(dest, "#![allow(clippy::many_single_char_names)]")?;
    writeln!(dest, "#![allow(clippy::let_unit_value)]")?;
    writeln!(dest, "#![allow(clippy::let_and_return)]")?;
    writeln!(dest)?;
    writeln!(dest, "//! Auto-generated GL bindings file.")?;
    writeln!(dest, "//! * API: {}", registry.api())?;
    writeln!(dest, "//! * Version: {:?}", registry.version())?;
    writeln!(dest, "//! * Fallbacks: {:?}", registry.fallbacks())?;
    writeln!(dest, "//! * Extensions: {:?}", registry.extensions())?;
    writeln!(dest, "//! * Generator Style: {:?}", gen)?;
    writeln!(dest)?;
    if gen.trace {
        writeln!(dest, "use super::trace;")?;
    }
    if gen.debug_assert_error_check {
        writeln!(dest, "use super::error;")?;
    }
    writeln!(dest, "use core::ffi::c_void;")?;
    writeln!(dest, "use core::ptr::NonNull;")?;
    writeln!(dest, "use core::mem::transmute;")?;
    writeln!(dest, "#[cfg(not(windows))] pub use libc::{{
    c_char, c_int, c_long, c_longlong, c_short, c_uint, c_ulong, c_ulonglong, c_ushort, c_float, c_double,
  }};")?;
    writeln!(
        dest,
        "#[cfg(windows)] pub type c_char = i8;
    #[cfg(windows)] pub type c_schar = i8;
    #[cfg(windows)] pub type c_uchar = u8;
    #[cfg(windows)] pub type c_short = i16;
    #[cfg(windows)] pub type c_ushort = u16;
    #[cfg(windows)] pub type c_int = i32;
    #[cfg(windows)] pub type c_uint = u32;
    #[cfg(windows)] pub type c_long = i32;
    #[cfg(windows)] pub type c_ulong = u32;
    #[cfg(windows)] pub type c_longlong = i64;
    #[cfg(windows)] pub type c_ulonglong = u64;
    #[cfg(windows)] pub type c_float = f32;
    #[cfg(windows)] pub type c_double = f64;"
    )?;
    writeln!(dest, "type OptVoidPtr = Option<NonNull<c_void>>;")?;
    Ok(())
}

/// Creates a structure which stores all the `FnPtr` of the bindings.
///
/// The name of the struct corresponds to the namespace.
fn write_struct(registry: &Registry, dest: &mut dyn io::Write) -> io::Result<()>
{
    writeln!(
        dest,
        "
        #[derive(Clone)]
        pub struct {api} {{",
        api = super::gen_struct_name(registry.api())
    )?;

    for cmd in registry.cmds() {
        writeln!(dest, "{}", registry.get_docs_for_cmd(cmd))?;
        if let Some(v) = registry.aliases().get(&cmd.proto.ident) {
            writeln!(dest, "/// ")?;
            writeln!(dest, "/// Fallbacks: {}", v.join(", "))?;
        }
        writeln!(dest, "pub {name}: OptVoidPtr,", name = cmd.proto.ident)?;
    }
    writeln!(dest, "_priv: ()")?;

    writeln!(dest, "}}")
}

/// Creates the `impl` of the structure created by `write_struct`.
fn write_impl(
    registry: &Registry,
    dest: &mut dyn io::Write,
    trace: bool,
    debug_assert: bool,
) -> io::Result<()>
{
    writeln!(dest,"impl {api} {{
    /// Load each OpenGL symbol using a provided loader function.
    /// 
    /// This allows for the use of functions like `glfwGetProcAddress` or `SDL_GL_GetProcAddress`.
    /// 
    /// ```ignore
    /// {api}::load_with(|ptr| SDL_GL_GetProcAddress(ptr));
    /// ```
    /// 
    /// ## Safety
    /// * This mostly trusts whatever the loader function says, so the loader must provide the correct pointer or a null pointer for each request.
    pub fn load_with<F>(mut load_fn: F) -> {api} where F: FnMut(*const c_char) -> *const c_void {{
      #[inline(never)]
      fn do_meta_load_fn(loader: &mut dyn FnMut(*const c_char) -> *const c_void, names: &[&[u8]]) -> OptVoidPtr {{
        fn ptr_ok(p: *const c_void) -> bool {{
          !(p.is_null() || p as usize % core::mem::align_of::<usize>() == 0)
        }}
        for name in names.iter() {{
          let p = loader(name.as_ptr() as *const c_char);
          if ptr_ok(p) {{
            return NonNull::new(p as *mut c_void);
          }}
        }}
        None
      }}
      let mut meta_load_fn = |names: &[&[u8]]| {{
        do_meta_load_fn(&mut load_fn, names)
      }};
      {api} {{",
          api = super::gen_struct_name(registry.api()))?;

    for cmd in registry.cmds() {
        let fallbacks = match registry.aliases().get(&cmd.proto.ident) {
            Some(v) => {
                let names = v
                    .iter()
                    .map(|name| {
                        format!(
                            r#",b"{}\0""#,
                            super::gen_symbol_name(registry.api(), &name[..])
                        )
                    })
                    .collect::<Vec<_>>();
                names.join(" ")
            },
            None => "".to_string(),
        };
        let symbol = super::gen_symbol_name(registry.api(), &cmd.proto.ident[..]);
        writeln!(
            dest,
            r#"  {name}: meta_load_fn(&[b"{symbol}\0" {fallbacks}]),"#,
            name = cmd.proto.ident,
            symbol = symbol,
            fallbacks = fallbacks,
        )?
    }

    writeln!(dest, "_priv: ()")?;

    writeln!(dest, "}} }}")?;

    for cmd in registry.cmds() {
        writeln!(
            dest,
            r#"#[inline] pub unsafe fn {name}(&self, {params}) {return_suffix} {{
        {trace_msg}let p = self.{name}.expect("{name}");
        let out = transmute::<
          NonNull<c_void>,
          extern "system" fn({typed_params}) {return_suffix}
        >(p)({idents});
        {debug_assert_error_check}out
      }}"#,
            trace_msg = if trace {
                format!(
                    r#"trace!("{}");
          "#,
                    cmd.proto.ident
                )
            } else {
                String::new()
            },
            debug_assert_error_check = if debug_assert && cmd.proto.ident != "GetError" {
                let mut args = super::gen_parameters(cmd, true, false);
                let params_fmt = core::iter::repeat("{:?}")
                    .take(args.len())
                    .collect::<Vec<_>>()
                    .join(", ");
                args.push("err".to_string());
                let params_args = args.join(", ");
                format!(
                    r#"if cfg!(debug_assertions) {{
          let err = self.GetError();
          if err != NO_ERROR {{
            error!("{name}({params_fmt}): {{}}", {params_args});
          }}
        }}
        "#,
                    name = cmd.proto.ident,
                    params_fmt = params_fmt,
                    params_args = params_args,
                )
            } else {
                String::new()
            },
            name = cmd.proto.ident,
            params = super::gen_parameters(cmd, true, true).join(", "),
            typed_params = super::gen_parameters(cmd, false, true).join(", "),
            return_suffix = if cmd.proto.ty != "()" {
                format!("-> {}", cmd.proto.ty)
            } else {
                String::new()
            },
            idents = super::gen_parameters(cmd, true, false).join(", "),
        )?
    }

    writeln!(
        dest,
        "}}

    unsafe impl Send for {api} {{}}",
        api = super::gen_struct_name(registry.api())
    )
}
