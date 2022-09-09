use {
    proc_macro::TokenStream,
    quote::{format_ident, quote},
    syn::{
        parse_macro_input, Data, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Ident, Variant,
    },
};

pub(super) fn derive_debug_with_cc_inner(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = parse_macro_input!(input);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let type_name = ident.to_string();
    let body = match data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => {
                let (field_names, fmt_fields) = fmt_fields_named(&type_name, fields_named);
                quote! {
                    let #ident { #(#field_names,)* } = self;
                    #fmt_fields
                }
            }
            Fields::Unnamed(fields_unnamed) => {
                let (field_names, fmt_fields) = fmt_fields_unnamed(&type_name, fields_unnamed);
                quote! {
                    let #ident(#(#field_names,)*) = self;
                    #fmt_fields
                }
            }
            Fields::Unit => {
                quote! {
                    f.write_str(#type_name)
                }
            }
        },
        Data::Enum(data_enum) => {
            let branches = {
                data_enum.variants.iter().map(|variant| {
                    let Variant {
                        ident: variant_ident,
                        fields,
                        ..
                    } = variant;
                    let type_variant_name = format!("{}::{}", type_name, variant_ident);
                    match fields {
                        Fields::Named(fields_named) => {
                            let (field_names, fmt_fields) =
                                fmt_fields_named(&type_variant_name, fields_named);
                            quote! {
                                #ident::#variant_ident { #(#field_names,)* } => {
                                    #fmt_fields
                                },
                            }
                        }
                        Fields::Unnamed(fields_unnamed) => {
                            let (field_names, fmt_fields) =
                                fmt_fields_unnamed(&type_variant_name, fields_unnamed);
                            quote! {
                                #ident::#variant_ident(#(#field_names,)*) => {
                                    #fmt_fields
                                },
                            }
                        }
                        Fields::Unit => {
                            quote! {
                                #ident::#variant_ident => {
                                    f.write_str(#type_variant_name)
                                },
                            }
                        }
                    }
                })
            };
            quote! {
                match self {
                    #(#branches)*
                }
            }
        }
        Data::Union(_) => {
            panic!("#[derive(DebugWithCC)] cannot be used on unions");
        }
    };
    let output = quote! {
        impl #impl_generics DebugWithCC for #ident #ty_generics
        #where_clause
        {
            fn fmt_with_cc<'a, 'c>(
                &'a self,
                f: &mut std::fmt::Formatter,
                cc: &'c CollectionContext,
            ) -> std::fmt::Result {
                #body
            }
        }
    };
    output.into()
}

fn fmt_fields_named<'i>(
    name: &str,
    fields_named: &'i FieldsNamed,
) -> (Vec<&'i Ident>, proc_macro2::TokenStream) {
    let field_names = {
        fields_named
            .named
            .iter()
            .map(|field| field.ident.as_ref().unwrap())
            .collect::<Vec<_>>()
    };
    let fmt_fields = {
        field_names.iter().map(|name| {
            let name_str = name.to_string();
            let expr = quote! {
                #name.with_cc(cc)
            };
            quote! {
                debug_struct = debug_struct.field(#name_str, &#expr);
            }
        })
    };
    let token_tree = quote! {
        let mut debug_struct = &mut f.debug_struct(#name);
        #(#fmt_fields)*
        debug_struct.finish()
    };
    (field_names, token_tree)
}

fn fmt_fields_unnamed(
    name: &str,
    fields_unnamed: &FieldsUnnamed,
) -> (Vec<Ident>, proc_macro2::TokenStream) {
    let field_names = {
        (0..fields_unnamed.unnamed.len())
            .map(|i| format_ident!("field_{}", i))
            .collect::<Vec<_>>()
    };
    let fmt_fields = {
        field_names.iter().map(|name| {
            let expr = quote! {
                #name.with_cc(cc)
            };
            quote! {
                debug_tuple = debug_tuple.field(&#expr);
            }
        })
    };
    let token_tree = quote! {
        let mut debug_tuple = &mut f.debug_tuple(#name);
        #(#fmt_fields)*
        debug_tuple.finish()
    };
    (field_names, token_tree)
}
