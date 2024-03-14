use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, parse_quote, Item};

#[proc_macro_attribute]
pub fn serde_clap_default(_: TokenStream, stream: TokenStream) -> TokenStream {
    // ...
    let item = parse_macro_input!(stream as Item);

    match item {
        Item::Struct(mut s) => {
            let mut default_fns = vec![];
            for field in s.fields.iter_mut() {
                for (i, attr) in field.attrs.iter_mut().enumerate() {
                    if !attr.path().is_ident("serde_clap_default") {
                        continue;
                    }

                    let value = attr.parse_args::<proc_macro2::TokenStream>().unwrap();

                    // inject the serde default
                    let fn_name = format!(
                        "__serde_clap_default__{}_{}",
                        s.ident,
                        field.ident.as_ref().expect("tuple structs are unsupported")
                    );
                    let fn_name_ident = Ident::new(&fn_name, Span::call_site());

                    let return_type = &field.ty;
                    let default_fn = quote! {
                        #[doc(hidden)]
                        fn #fn_name_ident () -> #return_type {
                            #value
                        }
                    };

                    default_fns.push(default_fn);
                    field
                        .attrs
                        .push(parse_quote!( #[serde(default = #fn_name)] ));

                    // add the clap default
                    let value_stringified = value.to_string();
                    field
                        .attrs
                        .push(parse_quote!( #[clap(default_value = #value_stringified)] ));

                    field.attrs.remove(i);

                    break;
                }
            }

            let expanded = quote! {
                #( #default_fns )*

                #s
            };

            expanded.into()
        }

        _ => panic!("can only be used on structs"),
    }
}
