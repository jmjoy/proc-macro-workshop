extern crate proc_macro;

use proc_macro::{TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, Data, Fields};
use syn::export::Span;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let metadata = match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(fields) => {
                fields.named.iter().map(|field| {
                    let ident = &field.ident;
                    let ty = &field.ty;
                    let expanded = quote! {
                         #ident: Option<#ty>,
                    };
                    let builder_expanded = quote! {
                        fn #ident(&mut self, #ident: #ty) -> &mut Self {
                            self.#ident = Some(#ident);
                            self
                        }
                    };
                    (ident, expanded, builder_expanded)
                })
            }
            _ => todo!(),
        },
        _ => todo!(),
    };

    let field_names = metadata.clone().map(|item| item.0);
    let fields = metadata.clone().map(|item| item.1);
    let builder_fns = metadata.map(|item| item.2);

    let builder_name = Ident::new(&format!("{}Builder", name), Span::call_site());
    let expanded = quote! {
        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#field_names: None,)*
                }
            }
        }

        pub struct #builder_name {
            #(#fields)*
        }

        impl #builder_name {
            #(#builder_fns)*
        }
    };
    expanded.into()
}
