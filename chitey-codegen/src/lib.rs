#![recursion_limit = "512"]

use proc_macro::TokenStream;
use quote::quote;

mod route;

#[proc_macro_attribute]
pub fn route(args: TokenStream, input: TokenStream) -> TokenStream {
    route::with_method(None, args, input)
}

#[proc_macro_attribute]
pub fn routes(_: TokenStream, input: TokenStream) -> TokenStream {
    route::with_methods(input)
}

macro_rules! method_macro {
    ($variant:ident, $method:ident) => {
        #[proc_macro_attribute]
        pub fn $method(args: TokenStream, input: TokenStream) -> TokenStream {
            route::with_method(Some(route::MethodType::$variant), args, input)
        }
    };
}

method_macro!(Get, get);
method_macro!(Post, post);
// method_macro!(Put, put);
// method_macro!(Delete, delete);
// method_macro!(Head, head);
// method_macro!(Connect, connect);
// method_macro!(Options, options);
// method_macro!(Trace, trace);
// method_macro!(Patch, patch);
