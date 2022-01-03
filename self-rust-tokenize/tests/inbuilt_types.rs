use quote::quote;
use self_rust_tokenize::SelfRustTokenize;

#[test]
fn number() {
    let thirty_four: i32 = 34;
    let tokens = SelfRustTokenize::to_tokens(&thirty_four);
    assert_eq!(tokens.to_string(), quote!(34i32).to_string());
}

#[test]
fn strings() {
    let str1 = "hi";
    let tokens = SelfRustTokenize::to_tokens(&str1);
    assert_eq!(tokens.to_string(), quote!("hi").to_string());

    let string1 = String::from("Hello");
    let tokens = SelfRustTokenize::to_tokens(&string1);
    assert_eq!(
        tokens.to_string(),
        quote!(String::from("Hello")).to_string()
    );
}
