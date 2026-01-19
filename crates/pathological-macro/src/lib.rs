use proc_macro::TokenStream;
use quote::quote;


#[proc_macro_attribute]
pub fn transform_to_invariant(_: TokenStream, _: TokenStream) -> TokenStream {
    quote! {
        struct MightLookCovariant<'a> {
            transformed_to_invariant: &'a Invariant<'a>,
        }
    }.into()
}
