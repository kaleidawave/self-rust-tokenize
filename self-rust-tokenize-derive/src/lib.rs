use std::error;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput, Stmt};
use syn_helpers::{build_implementation_over_structure, Field, Fields, Trait, TraitMethod};

#[proc_macro_derive(SelfRustTokenize, attributes(panic_on_self_tokenize))]
pub fn self_rust_tokenize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let self_rust_tokenize = Trait {
        name: parse_quote!(::self_rust_tokenize::SelfRustTokenize),
        generic_parameters: vec![],
        methods: vec![TraitMethod {
            method_name: Ident::new("to_tokens", Span::call_site()),
            return_type: Some(parse_quote!(::proc_macro2::TokenStream)),
            method_parameters: vec![parse_quote!(&self)],
            build_pair: Default::default(),
            method_generics: vec![],
        }],
    };

    build_implementation_over_structure(
        &input,
        self_rust_tokenize,
        |_, _| Ok(Default::default()),
        |method_name, fields| {
            if method_name == "to_tokens" {
                implement_to_tokens_for_fields(fields)
            } else {
                unreachable!()
            }
        },
    )
    .into()
}

fn implement_to_tokens_for_fields(fields: &mut Fields) -> Result<Vec<Stmt>, Box<dyn error::Error>> {
    if fields
        .get_field_attributes()
        .any(|attr| attr.path.is_ident("panic_on_self_tokenize"))
    {
        return Ok(vec![parse_quote!(panic!();)]);
    }
    match fields {
        Fields::Named {
            on_structure,
            fields,
        } => {
            let constructor_path = on_structure.full_name();
            let values = fields.iter_mut().map(|named_field| {
                let reference = named_field.get_reference();
                let qualifier = format!("{}:", named_field.name);
                quote! {
                    ::std::str::FromStr::from_str(#qualifier).unwrap(),
                    ::self_rust_tokenize::SelfRustTokenize::to_tokens(#reference),
                    ::std::str::FromStr::from_str(",").unwrap(),
                }
            });
            Ok(vec![
                parse_quote!(let mut _ts = ::std::str::FromStr::from_str(#constructor_path).unwrap();),
                parse_quote! {
                    ::std::iter::Extend::extend(&mut _ts, [
                        ::proc_macro2::TokenTree::Group(::proc_macro2::Group::new(
                            ::proc_macro2::Delimiter::Brace,
                            IntoIterator::into_iter([#(#values)*]).collect(),
                        ))
                    ]);
                },
                parse_quote!(return _ts;),
            ])
        }
        Fields::Unnamed {
            on_structure,
            fields,
        } => {
            let constructor_path = on_structure.full_name();
            let values = fields.iter_mut().map(|unnamed_field| {
                let reference = unnamed_field.get_reference();
                quote! {
                    ::self_rust_tokenize::SelfRustTokenize::to_tokens(#reference),
                    ::std::str::FromStr::from_str(",").unwrap(),
                }
            });
            Ok(vec![
                parse_quote!(let mut _ts = ::std::str::FromStr::from_str(#constructor_path).unwrap();),
                parse_quote! {
                    ::std::iter::Extend::extend(&mut _ts, [
                        ::proc_macro2::TokenTree::Group(::proc_macro2::Group::new(
                            ::proc_macro2::Delimiter::Parenthesis,
                            IntoIterator::into_iter([#(#values)*]).collect(),
                        ))
                    ]);
                },
                parse_quote!(return _ts;),
            ])
        }
        Fields::Unit { on_structure } => {
            let constructor_path = on_structure.full_name();
            Ok(vec![
                parse_quote!(return ::std::str::FromStr::from_str(#constructor_path).unwrap();),
            ])
        }
    }
}
