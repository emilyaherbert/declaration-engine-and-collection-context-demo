#[allow(clippy::module_inception)]
mod declaration_engine;
mod declaration_id;
mod declaration_ref;

pub(crate) use declaration_engine::*;
pub(crate) use declaration_id::*;
pub(crate) use declaration_ref::*;
