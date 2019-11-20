use crate::registry::Registry;
use std::io;

/// Allows GL to be used globally once loaded.
#[derive(Debug)]
pub struct GlobalGenerator {
    /// If `true`, the bindings will import a `trace!` macro from the parent
    /// module and invoke it with the name of the function about to be called
    /// _before_ actually calling each GL function.
    pub trace: bool,
    /// If `true`, the bindings will import an `error!` macro from the parent
    /// module and, when `debug_assertions` are configured for the build, will
    /// check _after_ each GL call if there was an error. If an error is
    /// detected, the `error!` macro will be invoked with a format string and
    /// args to format.
    pub debug_assert_error_check: bool,
    /// If `true` the bindings are stored in `AtomicPtr` values instead of
    /// `static mut` values. This makes it so that you can load GL from any
    /// thread, but there is a relaxed load when calling a function (which on
    /// `x86`/`x86_64` is "basically free", but on other systems it might not
    /// be).
    pub use_atomics: bool,
}
impl Default for GlobalGenerator {
    fn default() -> Self {
        Self {
            trace: true,
            debug_assert_error_check: true,
            use_atomics: true,
        }
    }
}

impl super::Generator for GlobalGenerator {
    fn write(&self, registry: &Registry, dest: &mut dyn io::Write) -> io::Result<()>
    {
        write_header(registry, dest, self)?;
        write_meta_loader(dest)?;
        super::write_type_aliases(registry, dest)?;
        super::write_enums(registry, dest)?;
        write_fns(
            registry,
            dest,
            self.trace,
            self.debug_assert_error_check,
            self.use_atomics,
        )?;
        write_ptrs(registry, dest, self.use_atomics)?;
        write_fn_mods(registry, dest, self.use_atomics)?;
        write_load_fn(registry, dest)?;
        Ok(())
    }
}

/// Creates a `__gl_imports` module which contains all the external symbols that
/// we need for the bindings.
fn write_header(registry: &Registry, dest: &mut dyn io::Write, gen: &GlobalGenerator) -> io::Result<()>
{
    writeln!(dest, "#![allow(bad_style)]")?;
    writeln!(dest, "#![allow(clippy::unreadable_literal)]")?;
    writeln!(dest, "#![allow(clippy::missing_safety_doc)]")?;
    writeln!(dest, "#![allow(clippy::too_many_arguments)]")?;
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
    if gen.use_atomics {
        writeln!(dest, "use core::sync::atomic::AtomicPtr;")?;
        writeln!(dest, "use core::sync::atomic::Ordering;")?;
    }
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

/// Creates the meta_loader function for fallbacks
fn write_meta_loader(dest: &mut dyn io::Write) -> io::Result<()>
{
    writeln!(
        dest,
        r#"#[inline(never)]
    unsafe fn meta_loader(
      loader: &mut dyn FnMut(*const c_char) -> *const c_void,
      names: &[&[u8]]
    ) -> OptVoidPtr {{
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
    "#
    )
}

/// Creates the functions corresponding to the GL commands.
///
/// The function calls the corresponding function pointer stored in the
/// `storage` module created  by `write_ptrs`.
fn write_fns(
    registry: &Registry,
    dest: &mut dyn io::Write,
    trace: bool,
    debug_assert: bool,
    use_atomics: bool,
) -> io::Result<()>
{
    writeln!(
        dest,
        r#"pub use functions::*;
    pub mod functions {{
      use super::*;"#
    )?;

    for cmd in registry.cmds() {
        writeln!(dest, "{}", registry.get_docs_for_cmd(cmd))?;
        if let Some(v) = registry.aliases().get(&cmd.proto.ident) {
            writeln!(dest, "/// ")?;
            writeln!(dest, "/// Fallbacks: {}", v.join(", "))?;
        }

        writeln!(
            dest,
            r#"#[inline]
      pub unsafe fn {name}({params}) {return_suffix} {{
        {trace_msg}let p = {get_style};
        let out = transmute::<
          NonNull<c_void>,
          extern "system" fn({typed_params}) {return_suffix}
        >(p)({idents});
        {debug_assert_error_check}out
      }}"#,
            name = cmd.proto.ident,
            get_style = if use_atomics {
                format!(
                    r#"{{ let p = storage::{name}.load(Ordering::Relaxed); if p.is_null() {{ panic!("{name}") }} p }}"#,
                    name = cmd.proto.ident
                )
            } else {
                format!(
                    r#"storage::{name}.expect("{name}")"#,
                    name = cmd.proto.ident
                )
            },
            params = super::gen_parameters(cmd, true, true).join(", "),
            typed_params = super::gen_parameters(cmd, false, true).join(", "),
            return_suffix = if cmd.proto.ty != "()" {
                format!("-> {}", cmd.proto.ty)
            } else {
                String::new()
            },
            idents = super::gen_parameters(cmd, true, false).join(", "),
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
          let err = GetError();
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
        )?;
    }

    writeln!(dest, "}}")
}

/// Creates a `storage` module which contains a static `FnPtr` per GL command in
/// the registry.
fn write_ptrs(registry: &Registry, dest: &mut dyn io::Write, use_atomics: bool) -> io::Result<()>
{
    writeln!(
        dest,
        "mod storage {{
      use super::*;"
    )?;

    for c in registry.cmds() {
        if use_atomics {
            writeln!(
                dest,
                "pub static {name}: AtomicPtr<c_void> = AtomicPtr::new(null_mut());",
                name = c.proto.ident
            )?;
        } else {
            writeln!(
                dest,
                "pub static mut {name}: OptVoidPtr = None;",
                name = c.proto.ident
            )?;
        }
    }

    writeln!(dest, "}}")
}

/// Creates one module for each GL command.
///
/// Each module contains `is_loaded` and `load_with` which interact with the
/// `storage` module  created by `write_ptrs`.
fn write_fn_mods(registry: &Registry, dest: &mut dyn io::Write, use_atomics: bool) -> io::Result<()>
{
    for c in registry.cmds() {
        let fallbacks = match registry.aliases().get(&c.proto.ident) {
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
        let fn_name = &c.proto.ident[..];
        let symbol = super::gen_symbol_name(registry.api(), &c.proto.ident[..]);

        writeln!(
            dest,
            r#"
        pub mod {fn_name} {{
          use super::*;

          #[inline]
          pub fn is_loaded() -> bool {{
            {load_check_op}
          }}

          /// Load `{fn_name}` using the provided loader.
          /// 
          /// ## Safety
          /// 
          /// * This writes to a static global. If you're accessing GL from more than one thread you can cause a data race.
          /// * This mostly trusts whatever the loader function says, so the loader must provide the correct pointer or a null pointer.
          pub unsafe fn load_with<F>(mut load_fn: F)
            where
          F: FnMut(*const c_char) -> *const c_void {{
            {loader_op}
          }}
        }}
      "#,
            load_check_op = if use_atomics {
                format!(
                    "unsafe {{ !storage::{fn_name}.load(Order::Relaxed).is_null() }}",
                    fn_name = fn_name
                )
            } else {
                format!(
                    "unsafe {{ storage::{fn_name}.is_some() }}",
                    fn_name = fn_name
                )
            },
            loader_op = if use_atomics {
                format!(
                    r#"if let Some(p) = meta_loader(&mut load_fn, &[b"{symbol}\0" {fallbacks}]) {{
            storage::{fn_name}.store(p, Ordering::SeqCst);
          }};"#,
                    fn_name = fn_name,
                    symbol = symbol,
                    fallbacks = fallbacks,
                )
            } else {
                format!(
                    r#"storage::{fn_name} = meta_loader(&mut load_fn, &[b"{symbol}\0" {fallbacks}]);"#,
                    fn_name = fn_name,
                    symbol = symbol,
                    fallbacks = fallbacks,
                )
            },
            fn_name = fn_name,
        )?;
    }

    Ok(())
}

/// Creates the `load_with` function.
///
/// The function calls `load_with` in each module created by `write_fn_mods`.
fn write_load_fn(registry: &Registry, dest: &mut dyn io::Write) -> io::Result<()>
{
    writeln!(dest,
    "/// Load each OpenGL symbol using a provided loader function.
    /// 
    /// This allows for the use of functions like `glfwGetProcAddress` or `SDL_GL_GetProcAddress`.
    /// 
    /// ```ignore
    /// gl::load_with(|ptr| SDL_GL_GetProcAddress(ptr));
    /// ```
    /// 
    /// ## Safety
    /// 
    /// * This writes to the static global function pointers. If you're accessing GL from more than one thread you can cause a data race.
    /// * This mostly trusts whatever the loader function says, so the loader must provide the correct pointer or a null pointer for each request.
    pub unsafe fn load_with<F>(mut load_fn: F)
      where
      F: FnMut(*const c_char) -> *const c_void {{
        #[inline(never)]
        unsafe fn inner(load_fn: &mut dyn FnMut(*const c_char) -> *const c_void) {{"
  )?;

    for c in registry.cmds() {
        writeln!(
            dest,
            "        {cmd_name}::load_with(&mut *load_fn);",
            cmd_name = &c.proto.ident[..]
        )?;
    }

    writeln!(
        dest,
        "      }}
      inner(&mut load_fn)
    }}
    "
    )
}
