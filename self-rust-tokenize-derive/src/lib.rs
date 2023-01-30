use proc_macro::TokenStream;
use syn_helpers::{
    derive_trait,
    proc_macro2::{Ident, Literal, Span},
    quote,
    syn::{parse_macro_input, parse_quote, DeriveInput},
    Constructable, FieldMut, Fields, Item, Trait, TraitItem, TypeOfSelf,
};

#[proc_macro_derive(SelfRustTokenize, attributes(panic_on_self_tokenize))]
pub fn self_rust_tokenize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let append_to_token_stream = TraitItem::new_method(
        Ident::new("append_to_token_stream", Span::call_site()),
        None,
        TypeOfSelf::Reference,
        vec![parse_quote!(
            token_stream: &mut ::self_rust_tokenize::proc_macro2::TokenStream
        )],
        None,
        |mut item: Item| {
            item.map_constructable(|mut constructable| {
                if constructable
                    .get_fields()
                    .get_field_attributes()
                    .iter()
                    .any(|attr| attr.path.is_ident("panic_on_self_tokenize"))
                {
                    return Ok(vec![parse_quote!(panic!();)]);
                }

                let segments =
                    constructable.get_constructor_path().segments.into_iter().map(|seg| Literal::string(&seg.ident.to_string()));

                let call = match constructable.get_fields_mut() {
                    Fields::Named(named, _) => {
                        let values = named.iter_mut().map(|named_field| {
                            let reference = named_field.get_reference();
                            let name = Literal::string(&named_field.name.to_string());
                            quote!((#name, ::self_rust_tokenize::SelfRustTokenize::to_tokens(#reference)))
                        });
                        parse_quote! {
                            ::self_rust_tokenize::_private::add_named_constructor_body(
                                token_stream,
                                &[#(#segments),*],
                                vec![#(#values),*],
                            );
                        }
                    }
                    Fields::Unnamed(unnamed, _) => {
                        let values = unnamed.iter_mut().map(|unnamed_field| {
                            let reference = unnamed_field.get_reference();
                            quote!(::self_rust_tokenize::SelfRustTokenize::to_tokens(#reference))
                        });
                        parse_quote! {
                            ::self_rust_tokenize::_private::add_unnamed_constructor_body(
                                token_stream,
                                &[#(#segments),*],
                                vec![#(#values),*],
                            );
                        }
                    }
                    Fields::Unit(_) => {
                        parse_quote! {
                            ::self_rust_tokenize::_private::add_unit_constructor_body(
                                token_stream,
                                &[#(#segments),*],
                            );
                        }
                    }
                };
                Ok(vec![call])
            })
        },
    );

    let self_rust_tokenize = Trait {
        name: parse_quote!(::self_rust_tokenize::SelfRustTokenize),
        generic_parameters: None,
        items: vec![append_to_token_stream],
    };

    let output = derive_trait(input, self_rust_tokenize);

    output.into()
}
