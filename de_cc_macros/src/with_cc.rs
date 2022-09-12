use {
    proc_macro::TokenStream,
    quote::quote,
    syn::{parse_macro_input, DeriveInput},
};

pub(super) fn derive_with_cc_inner(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, generics, ..
    } = parse_macro_input!(input);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let output = quote! {
        impl #impl_generics WithCC for #ident #ty_generics #where_clause {
            fn with_cc<'a, 'c>(&'a self, cc: &'c CollectionContext) -> WrapperCC<'a, 'c, Self> {
                WrapperCC { thing: self, cc }
            }
        }
    };
    output.into()
}
