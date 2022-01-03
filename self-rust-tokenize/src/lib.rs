use proc_macro2::TokenStream;
use quote::quote;
pub use self_rust_tokenize_derive::SelfRustTokenize;

pub trait SelfRustTokenize {
    /// Returns the tokens used to construct self
    fn to_tokens(&self) -> TokenStream;
}

macro_rules! implement_using_quote_to_tokens {
    ($($T:ty),*) => {
        $(
            impl SelfRustTokenize for $T {
                fn to_tokens(&self) -> TokenStream {
                    quote::ToTokens::to_token_stream(self)
                }
            }
        )*
    };
}

implement_using_quote_to_tokens!(
    u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64, char, bool, &str
);

impl<T: SelfRustTokenize> SelfRustTokenize for Box<T> {
    fn to_tokens(&self) -> TokenStream {
        let inner_tokens = (&**self).to_tokens();
        quote!(Box::new(#inner_tokens))
    }
}

impl<T: SelfRustTokenize> SelfRustTokenize for Vec<T> {
    fn to_tokens(&self) -> TokenStream {
        let inner_tokens = self.iter().map(SelfRustTokenize::to_tokens);
        quote!(vec!(#(#inner_tokens),*))
    }
}

impl<T: SelfRustTokenize> SelfRustTokenize for Option<T> {
    fn to_tokens(&self) -> TokenStream {
        match self {
            Some(value) => {
                let inner_tokens = value.to_tokens();
                quote!(Some(#inner_tokens))
            }
            None => quote!(None),
        }
    }
}

impl SelfRustTokenize for String {
    fn to_tokens(&self) -> TokenStream {
        let value = self.as_str();
        quote!(String::from(#value))
    }
}
