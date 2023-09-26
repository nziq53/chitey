#[proc_macro_attribute]
pub fn routes(_: TokenStream, input: TokenStream) -> TokenStream {
    route::with_methods(input)
}
