use quote::quote;
use self_rust_tokenize::SelfRustTokenize;

#[derive(SelfRustTokenize)]
enum X {
    A,
    B(String),
    C { m: bool, n: i32 },
}

#[test]
fn variant() {
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
