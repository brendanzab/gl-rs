use super::{NamedType, Registry};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type {
    pub kind: TypeKind,
    /// Optional types are the default in WebIDL, so we give this a special
    /// place in the `Type` reference. It's also convenient to be able to
    /// "squash" optional flags to avoid `Option<Option<T>>`.
    pub optional: bool,
}

/// The different kinds of primitive types supported by WebIDL
/// These are named according to their equivalents in rust.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Primitive {
    Bool,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
}

/// The definition of a type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeKind {
    // Primitive types
    Primitive(Primitive),

    // "Known" types, that may deserve special handling
    String,
    ArrayBuffer,
    ArrayBufferView,
    BufferSource,
    CanvasElement,

    // Collection types
    TypedArray(Primitive),
    Sequence(Box<Type>),
    Union(Vec<Type>),

    // Named types
    Named(String),

    // Misc. types
    Any,
    Object,
}

impl Type {
    /// Return an optional version of this type reference.
    /// Returns an identical copy if the type is already optional.
    pub fn optional(&self) -> Self {
        Type {
            kind: self.kind.clone(),
            optional: true,
        }
    }
}

impl<'a> From<&'a str> for Type {
    /// Construct a type reference from a name
    fn from(s: &'a str) -> Type {
        Type {
            kind: TypeKind::Named(s.into()),
            optional: false,
        }
    }
}

impl Primitive {
    /// Get the rust name for a primitive type
    pub fn name(self) -> &'static str {
        use self::Primitive::*;
        match self {
            Bool => "bool",
            I8 => "i8",
            U8 => "u8",
            I16 => "i16",
            U16 => "u16",
            I32 => "i32",
            U32 => "u32",
            I64 => "i64",
            U64 => "u64",
            F32 => "f32",
            F64 => "f64",
        }
    }
}

impl TypeKind {
    /// Look through type aliases to find the "real" definition of a type.
    /// Also returns the "original name" if applicable.
    pub fn flatten<'a>(&'a self, registry: &'a Registry) -> (Option<&'a str>, &'a TypeKind) {
        match self {
            &TypeKind::Primitive(ref p) => (Some(p.name()), self),
            &TypeKind::Named(ref s) => {
                if let &NamedType::Typedef(ref t) = registry.resolve_type(s) {
                    if !t.optional {
                        return (Some(s.as_str()), t.kind.flatten(registry).1);
                    }
                }
                (Some(s.as_str()), self)
            }
            _ => (None, self),
        }
    }
}
