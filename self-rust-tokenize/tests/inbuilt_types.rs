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

#[test]
fn vec() {
    let vec1 = vec!["hello", "test"];
    let tokens = SelfRustTokenize::to_tokens(&vec1);
    assert_eq!(
        tokens.to_string(),
        quote!(vec!["hello", "test"]).to_string()
    );
}

#[test]
fn arrays() {
    let array1 = ["hello", "test"];
    let tokens = SelfRustTokenize::to_tokens(&array1);
    assert_eq!(tokens.to_string(), quote!(["hello", "test"]).to_string());
}

#[test]
fn tuples() {
    assert_eq!(
        SelfRustTokenize::to_tokens(&()).to_string(),
        quote!(()).to_string()
    );

    let tup1 = ("hello", Box::new(2));
    let tokens = SelfRustTokenize::to_tokens(&tup1);
    assert_eq!(
        tokens.to_string(),
        quote!(("hello", Box::new(2i32))).to_string()
    );
}

#[cfg(feature = "smallvec")]
#[test]
fn smallvec() {
    use smallvec::{smallvec, SmallVec};
    let v: SmallVec<[_; 128]> = smallvec![1, 2, 3];
    let tokens = SelfRustTokenize::to_tokens(&v);
    assert_eq!(
        tokens.to_string(),
        quote!(::smallvec::smallvec![1i32, 2i32, 3i32]).to_string()
    );
}
