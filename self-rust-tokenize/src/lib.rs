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
    u8,
    u16,
    u32,
    u64,
    u128,
    i8,
    i16,
    i32,
    i64,
    i128,
    f32,
    f64,
    char,
    bool,
    &'static str
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
        quote!(vec![#(#inner_tokens),*])
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

impl<T: SelfRustTokenize, const N: usize> SelfRustTokenize for [T; N] {
    fn to_tokens(&self) -> TokenStream {
        let inner_tokens = self.iter().map(SelfRustTokenize::to_tokens);
        quote!([#(#inner_tokens),*])
    }
}

impl<T: SelfRustTokenize> SelfRustTokenize for [T] {
    fn to_tokens(&self) -> TokenStream {
        let inner_tokens = self.iter().map(SelfRustTokenize::to_tokens);
        quote!(&[#(#inner_tokens),*])
    }
}

impl SelfRustTokenize for () {
    fn to_tokens(&self) -> TokenStream {
        quote!(())
    }
}

// Thanks! https://stackoverflow.com/a/56700760/10048799
macro_rules! tuple_impls {
    ( $( $name:ident )+ ) => {
        impl<$($name: SelfRustTokenize),+> SelfRustTokenize for ($($name,)+)
        {
            fn to_tokens(&self) -> TokenStream {
                #[allow(non_snake_case)]
                let ($($name,)+) = self;
                let inner_tokens = &[$(SelfRustTokenize::to_tokens($name)),+];
                quote!((#(#inner_tokens,)*))
            }
        }
    };
}

tuple_impls! { A }
tuple_impls! { A B }
tuple_impls! { A B C }
tuple_impls! { A B C D }
tuple_impls! { A B C D E }
tuple_impls! { A B C D E F }
tuple_impls! { A B C D E F G }
tuple_impls! { A B C D E F G H }
tuple_impls! { A B C D E F G H I }
tuple_impls! { A B C D E F G H I J }
tuple_impls! { A B C D E F G H I J K }
tuple_impls! { A B C D E F G H I J K L }

#[cfg(feature = "references")]
mod references {
    use super::{SelfRustTokenize, TokenStream};
    use quote::quote;

    impl<'a, T: SelfRustTokenize> SelfRustTokenize for &'a T {
        fn to_tokens(&self) -> TokenStream {
            let inner_tokens = (*self).to_tokens();
            quote!(&#inner_tokens)
        }
    }

    impl<'a, T: SelfRustTokenize> SelfRustTokenize for &'a mut T {
        fn to_tokens(&self) -> TokenStream {
            let inner_tokens = (**self).to_tokens();
            quote!(&mut #inner_tokens)
        }
    }

    impl<'a, T: SelfRustTokenize> SelfRustTokenize for &'a [T] {
        fn to_tokens(&self) -> TokenStream {
            let inner_tokens = self.iter().map(SelfRustTokenize::to_tokens);
            quote!(&[#(#inner_tokens),*])
        }
    }

    impl<'a, T: SelfRustTokenize> SelfRustTokenize for &'a mut [T] {
        fn to_tokens(&self) -> TokenStream {
            let inner_tokens = self.iter().map(SelfRustTokenize::to_tokens);
            quote!(&mut [#(#inner_tokens),*])
        }
    }
}

#[cfg(feature = "smallvec")]
impl<T: smallvec::Array> SelfRustTokenize for smallvec::SmallVec<T>
where
    T::Item: SelfRustTokenize,
{
    fn to_tokens(&self) -> TokenStream {
        let inner_tokens = self.iter().map(SelfRustTokenize::to_tokens);
        quote!(::smallvec::smallvec![#(#inner_tokens),*])
    }
}
