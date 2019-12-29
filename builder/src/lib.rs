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
                    let error_message = format!("Field `{}` cant't be none.", ident.as_ref().unwrap().to_string());
                    let builder_if_expanded = quote! {
                        if self.#ident.is_none() {
                            std::result::Result::Err(#error_message)?;
                        }
                    };
                    let builder_construct_expanded = quote! {
                        #ident: self.#ident.clone().unwrap(),
                    };
                    (ident, expanded, builder_expanded, builder_if_expanded, builder_construct_expanded)
                })
            }
            _ => todo!(),
        },
        _ => todo!(),
    };

    let field_names = metadata.clone().map(|item| item.0);
    let fields = metadata.clone().map(|item| item.1);
    let builder_fns = metadata.clone().map(|item| item.2);
    let builder_if_stmts = metadata.clone().map(|item| item.3);
    let builder_constructs = metadata.map(|item| item.4) ;

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

             pub fn build(&mut self) -> std::result::Result<#name, std::boxed::Box<dyn std::error::Error>> {
                #(#builder_if_stmts)*

                Ok(#name {
                    #(#builder_constructs)*
                })
             }
        }
    };
    expanded.into()
}
