# Self Rust Tokenize

[![](https://img.shields.io/crates/v/self-rust-tokenize)](https://crates.io/crates/self-rust-tokenize)
[![](https://img.shields.io/docsrs/self-rust-tokenize)](https://docs.rs/self-rust-tokenize/latest)

For taking a instance of a structure and generating a `proc_macro2::TokenStream` of tokens which generate the structure.

```rust
assert_eq!(
    self_rust_tokenize::SelfRustTokenize::to_tokens(&String::from("Hello")).to_string(),
    self_rust_tokenize::quote!(::std::string::String::from("Hello")).to_string()
);
```

Deriving on a custom type

```rust
#[derive(self_rust_tokenize::SelfRustTokenize)]
struct A(pub i32);

let a = A(12);
assert_eq!(
    self_rust_tokenize::SelfRustTokenize::to_tokens(&a).to_string(),
    self_rust_tokenize::quote!(A(12i32,)).to_string()
);
```

The use case may be: sharing a structure between a crate that deals with instances of it and a proc macro crate which generates tokens that build the instances in a exported proc macro.

It can be used for doing constant compilation of structures that allocate due to the nature of the structure. (this crate was built for the partial constant compilation of abstract syntax trees).

```ignore
/// Base definition crate
pub struct SpecialStructure { 
    // ...
}

impl SpecialStructure {
    pub fn generate_from_input(&str) -> Self {
        // some long implementation
    }
}

/// Proc macro crate
use base_crate::SpecialStructure; 

#[proc_macro]
pub fn make_special_structure(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as LitStr).value();
    let instance = SpecialStructure::generate_from_input(&input);
    let instance_constructor = instance.to_tokens();
    quote! {
        {
            use ::base_crate::SpecialStructure;
            #instance_constructor
        }
    }.into()
}

/// Main crate
SpecialStructure::generate_from_input("hello") == make_special_structure!("hello")
```

*note that the derived token stream is not scoped, you have to import the structures themselves*

## Cargo Features

- `smallvec`, adds implementation of `SelfRustTokenize` for `SmallVec` in the `smallvec` crate
- `references`, adds implementation of `SelfRustTokenize` for immutable and mutable references and immutable and mutable slices. Note that for references the tokenization does not preserve the structure as pointer information is different in the tokenization of the new structure. e.g `ptr::eq` will behavior differently. (thus the reasoning for this being a opt-in feature)

### Why `self_rust_tokenize::SelfRustTokenize` trait and not `quote::ToTokens`?

`quote::ToTokens` is defined on many types in std to return a more primitive representation of themselves and can lose their type structure. On the other hand `self_rust_tokenize::SelfRustTokenize` implementations on std types keeps the type constructor information. Thus a new trait (`self_rust_tokenize::SelfRustTokenize`) is needed to prevent implementation conflicts.

e.g.

```ignore
let hello_string = String::new("Hello");
self_rust_tokenize::SelfRustTokenize::to_tokens(hello_string) == quote!(::std::string::String::new("Hello"));
quote::ToTokens::to_tokens(hello_string) == quote!("Hello");
```