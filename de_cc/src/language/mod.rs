//! This module is intended to match the old internal compiler AST's as cleanly as possible.
//!
//! - The [untyped] module has the untyped AST.
//! - The [typed] module has the typed AST. This typed AST resembles the "old typed AST" in the compiler,
//!     but they are not the same, as the AST in [typed] does not contain inlined declarations. For example,
//!     the old typed AST inlined function declarations inside of function applications, while the AST in
//!     [typed] does not.
//! - The [resolved] module has the typed AST after resolution. This AST most closely resembles the old
//!     typed AST in the compiler but it also uses a new [ResolvedType](crate::type_system::resolved_types::ResolvedType),
//!     which is a codegen-safe well-formed subset of [TypeInfo](crate::type_system::type_info::TypeInfo).

pub(crate) mod literal;
pub(crate) mod partial;
pub(crate) mod resolved;
pub(crate) mod typed;
pub(crate) mod typing_context;
pub mod untyped;
