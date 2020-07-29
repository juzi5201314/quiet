use proc_macro::TokenStream;

use quote::quote;
use syn::{ItemFn, parse_macro_input};

pub fn test(_: TokenStream, item: TokenStream) -> TokenStream {
    let f = parse_macro_input!(item as ItemFn);
    let name = &f.sig.ident;
    let return_t = &f.sig.output;
    let body = &f.block;

    TokenStream::from(quote! {
        #[tokio::test]
        async fn #name() #return_t {
            dotenv::dotenv().ok();
            crate::database::init().await;
            #body
        }
    })
}