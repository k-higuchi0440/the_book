extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

// 2. procedural macros
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    let ast = syn::parse(input).unwrap();

    // build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    // you can define the Rsut code that we want to return with quote!
    // quote! provides template mechanism. #name is replaced by the varible value above
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                // stringify! takes Rust Expression and turns into string literal
                // there is a possibility that #name might be an expression. so you should use stringify!
                println!("Hello, Macro! Myname is {}!", stringify!(#name));
            }
        }
    };
    gen.into() // the intermidiate representation above to TokenStream
}
