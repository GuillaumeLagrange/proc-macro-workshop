use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, Fields};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_ident = &input.ident;
    let builder_ident = format_ident!("{}Builder", struct_ident);

    let data = input.data;

    let fields = match data {
        syn::Data::Struct(data) => {
            let fields = data.fields;

            match fields {
                Fields::Named(fields) => fields
                    .named
                    .into_iter()
                    .map(|f| (f.ident.unwrap(), f.ty))
                    .collect::<Vec<_>>(),
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    };

    let field_names = fields.iter().map(|(name, _)| name).collect::<Vec<_>>();
    let field_types = fields.iter().map(|(_, ty)| ty).collect::<Vec<_>>();

    let expanded = quote! {
        pub struct #builder_ident {
            #(#field_names: Option<#field_types>),*
        }

        impl #builder_ident {
            #(
                pub fn #field_names(&mut self, #field_names: #field_types) -> &mut Self {
                    self.#field_names = Some(#field_names);
                    self
                }
            )*

            pub fn build(&mut self) -> Result<#struct_ident, Box<dyn std::error::Error>> {
                Ok(#struct_ident {
                    #(#field_names: self.#field_names.clone().ok_or("missing field")?),*
                })
            }
        }

        impl #struct_ident {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                    #(#field_names: None),*
                }
            }
        }
    };

    expanded.into()
}
