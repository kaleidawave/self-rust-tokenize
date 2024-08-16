#![doc = include_str!("./README.md")]

pub use self_rust_tokenize_derive::SelfRustTokenize;

/// Re-exports `proc_macro2` and `quote` to help implement custom `SelfRustTokenize`
pub mod helpers {
    pub use proc_macro2::{self, TokenStream};
    pub use quote::{quote, ToTokens as QuoteToTokens, TokenStreamExt};
}

use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use quote::{ToTokens as QuoteToTokens, TokenStreamExt};
use std::ops::Deref;

/// An item which can be turned into tokens that *match* its original construction
///
/// *note*: this is implmented for `Box<T>` and `Vec<T>` which while staisfies PartialEq it loses pointer information
pub trait SelfRustTokenize {
    /// Returns the tokens used to construct self
    fn to_tokens(&self) -> TokenStream {
        let mut ts = TokenStream::new();
        Self::append_to_token_stream(self, &mut ts);
        ts
    }

    fn append_to_token_stream(&self, token_stream: &mut TokenStream);
}

macro_rules! implement_using_quote_to_tokens {
    ($($T:ty),*) => {
        $(
            impl SelfRustTokenize for $T {
                fn append_to_token_stream(&self, token_stream: &mut TokenStream) {
                    QuoteToTokens::to_tokens(self, token_stream)
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

fn append_path(segments: &[&'static str], token_stream: &mut TokenStream, leading_colons: bool) {
    for (idx, segment) in segments.iter().enumerate() {
        if leading_colons || idx != 0 {
            token_stream.append(Punct::new(':', Spacing::Joint));
            token_stream.append(Punct::new(':', Spacing::Alone));
        }
        token_stream.append(Ident::new(segment, Span::call_site()))
    }
}

// Note the loss of pointer information here
impl<T: SelfRustTokenize> SelfRustTokenize for Box<T> {
    fn append_to_token_stream(&self, token_stream: &mut TokenStream) {
        append_path(&["std", "boxed", "Box", "new"], token_stream, true);
        token_stream.append(Group::new(
            Delimiter::Parenthesis,
            Deref::deref(self).to_tokens(),
        ));
    }
}

impl<T> SelfRustTokenize for std::marker::PhantomData<T> {
    fn append_to_token_stream(&self, token_stream: &mut TokenStream) {
        append_path(
            &["std", "marker", "PhantomData", "default"],
            token_stream,
            true,
        );
        // TODO not sure
        token_stream.append(Group::new(Delimiter::Parenthesis, Default::default()));
    }
}

impl<T: SelfRustTokenize> SelfRustTokenize for Vec<T> {
    fn append_to_token_stream(&self, token_stream: &mut TokenStream) {
        append_path(&["std", "vec"], token_stream, true);
        token_stream.append(Punct::new('!', Spacing::Alone));
        let mut inner_token_stream = TokenStream::default();
        for (idx, inner) in self.iter().enumerate() {
            inner.append_to_token_stream(&mut inner_token_stream);
            if idx != self.len() - 1 {
                inner_token_stream.append(Punct::new(',', Spacing::Alone));
            }
        }
        token_stream.append(Group::new(Delimiter::Bracket, inner_token_stream))
    }
}

impl<T: SelfRustTokenize> SelfRustTokenize for Option<T> {
    fn append_to_token_stream(&self, token_stream: &mut TokenStream) {
        match self {
            Some(value) => {
                append_path(&["std", "option", "Option", "Some"], token_stream, true);
                token_stream.append(Group::new(
                    Delimiter::Parenthesis,
                    SelfRustTokenize::to_tokens(value),
                ))
            }
            None => {
                append_path(&["std", "option", "Option", "None"], token_stream, true);
            }
        }
    }
}

impl SelfRustTokenize for String {
    fn append_to_token_stream(&self, token_stream: &mut TokenStream) {
        append_path(&["std", "string", "String", "from"], token_stream, true);
        let stream = TokenStream::from(TokenTree::from(Literal::string(self.as_str())));
        token_stream.append(Group::new(Delimiter::Parenthesis, stream))
    }
}

impl<T: SelfRustTokenize, const N: usize> SelfRustTokenize for [T; N] {
    fn append_to_token_stream(&self, token_stream: &mut TokenStream) {
        let mut inner_token_stream = TokenStream::new();
        for (idx, inner) in self.iter().enumerate() {
            inner.append_to_token_stream(&mut inner_token_stream);
            if idx != self.len() - 1 {
                inner_token_stream.append(Punct::new(',', Spacing::Alone));
            }
        }
        token_stream.append(Group::new(Delimiter::Bracket, inner_token_stream));
    }
}

impl<T: SelfRustTokenize> SelfRustTokenize for [T] {
    fn append_to_token_stream(&self, token_stream: &mut TokenStream) {
        token_stream.append(Punct::new('&', Spacing::Alone));
        let mut inner_token_stream = TokenStream::new();
        for (idx, inner) in self.iter().enumerate() {
            inner.append_to_token_stream(&mut inner_token_stream);
            if idx != self.len() - 1 {
                inner_token_stream.append(Punct::new(',', Spacing::Alone));
            }
        }
        token_stream.append(Group::new(Delimiter::Bracket, inner_token_stream));
    }
}

impl SelfRustTokenize for () {
    fn append_to_token_stream(&self, token_stream: &mut TokenStream) {
        token_stream.append(Group::new(Delimiter::Parenthesis, Default::default()));
    }
}

// Thanks! https://stackoverflow.com/a/56700760/10048799
macro_rules! tuple_impls {
    ( $( $name:ident )+ ) => {
        impl<$($name: SelfRustTokenize),+> SelfRustTokenize for ($($name,)+)
        {
            fn append_to_token_stream(&self, token_stream: &mut TokenStream) {
                #[allow(non_snake_case)]
                let ($($name,)+) = self;
                let mut inner_token_stream = TokenStream::new();
                $(
                    SelfRustTokenize::append_to_token_stream($name, &mut inner_token_stream);
                    inner_token_stream.append(Punct::new(',', Spacing::Alone));
                )*
                token_stream.append(Group::new(Delimiter::Parenthesis, inner_token_stream));
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
    use proc_macro2::{Delimiter, Group, Ident, Punct, Spacing, Span};
    use quote::TokenStreamExt;

    impl<'a, T: SelfRustTokenize> SelfRustTokenize for &'a T {
        fn append_to_token_stream(&self, token_stream: &mut TokenStream) {
            token_stream.append(Punct::new('&', Spacing::Alone));
            (*self).append_to_token_stream(token_stream);
        }
    }

    impl<'a, T: SelfRustTokenize> SelfRustTokenize for &'a mut T {
        fn append_to_token_stream(&self, token_stream: &mut TokenStream) {
            token_stream.append(Punct::new('&', Spacing::Alone));
            token_stream.append(Ident::new("mut", Span::call_site()));
            (**self).append_to_token_stream(token_stream);
        }
    }

    impl<'a, T: SelfRustTokenize> SelfRustTokenize for &'a [T] {
        fn append_to_token_stream(&self, token_stream: &mut TokenStream) {
            token_stream.append(Punct::new('&', Spacing::Alone));
            let mut inner_token_stream = TokenStream::new();
            for (idx, inner) in self.iter().enumerate() {
                inner.append_to_token_stream(&mut inner_token_stream);
                if idx != self.len() - 1 {
                    inner_token_stream.append(Punct::new(',', Spacing::Alone));
                }
            }
            token_stream.append(Group::new(Delimiter::Bracket, inner_token_stream));
        }
    }

    impl<'a, T: SelfRustTokenize> SelfRustTokenize for &'a mut [T] {
        fn append_to_token_stream(&self, token_stream: &mut TokenStream) {
            token_stream.append(Punct::new('&', Spacing::Alone));
            token_stream.append(Ident::new("mut", Span::call_site()));
            let mut inner_token_stream = TokenStream::new();
            for (idx, inner) in self.iter().enumerate() {
                inner.append_to_token_stream(&mut inner_token_stream);
                if idx != self.len() - 1 {
                    inner_token_stream.append(Punct::new(',', Spacing::Alone));
                }
            }
            token_stream.append(Group::new(Delimiter::Bracket, inner_token_stream));
        }
    }
}

#[cfg(feature = "smallvec")]
impl<T: smallvec::Array> SelfRustTokenize for smallvec::SmallVec<T>
where
    T::Item: SelfRustTokenize,
{
    fn append_to_token_stream(&self, token_stream: &mut TokenStream) {
        append_path(&["smallvec", "smallvec"], token_stream, true);
        token_stream.append(Punct::new('!', Spacing::Alone));
        let mut inner_token_stream = TokenStream::new();
        for (idx, inner) in self.iter().enumerate() {
            inner.append_to_token_stream(&mut inner_token_stream);
            if idx != self.len() - 1 {
                inner_token_stream.append(Punct::new(',', Spacing::Alone));
            }
        }
        token_stream.append(Group::new(Delimiter::Bracket, inner_token_stream));
    }
}

#[doc(hidden)]
pub mod _private {
    use proc_macro2::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream};
    use quote::TokenStreamExt;

    use crate::append_path;

    pub fn add_named_constructor_body(
        ts: &mut proc_macro2::TokenStream,
        segments: &[&'static str],
        items: Vec<(&'static str, TokenStream)>,
    ) {
        append_path(segments, ts, false);

        let mut arguments = TokenStream::new();
        for (name, item) in items.into_iter() {
            arguments.append(Ident::new(name, Span::call_site()));
            arguments.append(Punct::new(':', Spacing::Alone));
            arguments.extend(item);
            arguments.append(Punct::new(',', Spacing::Alone));
        }
        ts.append(Group::new(Delimiter::Brace, arguments));
    }

    pub fn add_unnamed_constructor_body(
        ts: &mut proc_macro2::TokenStream,
        segments: &[&'static str],
        items: Vec<TokenStream>,
    ) {
        append_path(segments, ts, false);

        let mut arguments = TokenStream::new();
        for item in items.into_iter() {
            arguments.extend(item);
            arguments.append(Punct::new(',', Spacing::Alone));
        }
        ts.append(Group::new(Delimiter::Parenthesis, arguments));
    }

    pub fn add_unit_constructor_body(ts: &mut proc_macro2::TokenStream, segments: &[&'static str]) {
        append_path(segments, ts, false);
    }
}
