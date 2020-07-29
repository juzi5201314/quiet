use proc_macro::TokenStream;

mod test;

#[proc_macro_attribute]
pub fn test(attr: TokenStream, item: TokenStream) -> TokenStream {
    test::test(attr, item)
}