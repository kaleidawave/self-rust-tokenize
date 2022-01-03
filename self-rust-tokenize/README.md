# Self rust tokenize

[![](https://img.shields.io/crates/v/self-rust-tokenize)](https://crates.io/crates/self-rust-tokenize)

For taking a instance of a structure and generating a `proc_macro2::TokenStream` of tokens which generate the structure

```rust
self_rust_tokenize::SelfRustTokenize::to_tokens(String::new("Hello")) === quote!(String::new("Hello"));
```

Deriving on a custom type

```rust
#[derive(SelfRustTokenize)]
struct A(i32);

let a = A(12);
self_rust_tokenize::SelfRustTokenize::to_tokens(a) == quote!(A(12));
```

The use case may be: sharing a structure between a crate that deals with instances of it and a proc macro crate which generates tokens that build the instances in a exported proc macro

```rust
/// Base definition crate
pub enum SpecialStructure { 
    // ...
}

impl SpecialStructure {
    pub fn generate_from_input(&str) -> Self {
        // Some long implementation
    }
}

/// Proc macro crate
use base_crate::SpecialStructure; 

#[proc_macro]
pub fn make_special_structure(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as LitStr).value();
    let instance = SpecialStructure::generate_from_input(&input);
    let instance_constructor = instance.to_tokens();
    (quote! {
        {
            use ::base_crate::SpecialStructure;
            #instance_constructor
        }
    }).into()
}

/// Main crate
SpecialStructure::generate_from_input("hello") === make_special_structure!("hello")
```

*note that the derived token stream is not scoped, you have to import the structures themselves*

### Why `self_rust_tokenize::SelfRustTokenize` trait and not `quote::ToTokens`?

`quote::ToTokens` is defined on many types in std to return a more primitive representation of themselves and can lose their type structure. On the other hand `self_rust_tokenize::SelfRustTokenize` implementations on std types keeps the type constructor information. Thus a new trait (`self_rust_tokenize::SelfRustTokenize`) is needed to prevent implementation conflicts.

e.g.

```rust
self_rust_tokenize::SelfRustTokenize::to_tokens(String::new("Hello")) === quote!(String::new("Hello"));
quote::ToTokens::to_tokens(String::new("Hello")) === quote!("Hello");
```