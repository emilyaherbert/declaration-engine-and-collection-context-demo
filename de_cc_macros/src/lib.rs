mod debug;
mod partial_eq;
mod with_cc;

use proc_macro::TokenStream;

use debug::derive_debug_with_cc_inner;
use partial_eq::derive_partial_eq_with_cc_inner;
use with_cc::derive_with_cc_inner;

#[proc_macro_derive(DebugWithCC)]
pub fn derive_debug_with_cc(input: TokenStream) -> TokenStream {
    derive_debug_with_cc_inner(input)
}

#[proc_macro_derive(PartialEqWithCC)]
pub fn derive_partial_eq_with_cc(input: TokenStream) -> TokenStream {
    derive_partial_eq_with_cc_inner(input)
}

#[proc_macro_derive(WithCC)]
pub fn derive_with_cc(input: TokenStream) -> TokenStream {
    derive_with_cc_inner(input)
}
