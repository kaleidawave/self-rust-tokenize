use quote::quote;
use self_rust_tokenize::SelfRustTokenize;

#[test]
fn variant() {
    #[derive(SelfRustTokenize)]
    enum X {
        A,
        B(String),
        C { m: bool, n: i32 },
    }

    assert_eq!(X::A.to_tokens().to_string(), quote!(X::A).to_string());
    assert_eq!(
        X::B("Hello World".into()).to_tokens().to_string(),
        quote!(X::B(::std::string::String::from("Hello World"),)).to_string()
    );
    // rustfmt removes the trailing comma in the quote! 🥶
    #[rustfmt::skip]
    assert_eq!(
        X::C { m: false, n: 45 }.to_tokens().to_string(),
        quote!(X::C { m: false, n: 45i32, }).to_string()
    );
}

#[test]
fn self_tokenize_field() {
    #![allow(unused)]

    #[derive(SelfRustTokenize)]
    #[self_tokenize_field(actual_field)]
    struct Wrapper1<T, U> {
        ignored_field: String,
        another_ignored_field: u32,
        actual_field: T,
        another: std::marker::PhantomData<U>,
    }

    struct DoesNotImplementSelfRustTokenize;

    let w1: Wrapper1<u32, DoesNotImplementSelfRustTokenize> = Wrapper1 {
        ignored_field: Default::default(),
        another_ignored_field: Default::default(),
        actual_field: 200,
        another: Default::default(),
    };

    assert_eq!(w1.to_tokens().to_string(), quote!(200u32).to_string());

    #[derive(SelfRustTokenize)]
    #[self_tokenize_field(actual_field)]
    enum Wrapper2<T> {
        VariantOne,
        VariantTwo(String),
        #[self_tokenize_field(0)]
        Variant(T),
    }

    let w2 = Wrapper2::Variant(100u32);
    assert_eq!(w2.to_tokens().to_string(), quote!(100u32).to_string());
}
