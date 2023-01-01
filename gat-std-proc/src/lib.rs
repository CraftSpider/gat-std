
use proc_macro::TokenStream;
use quote::quote;

mod desugar;

#[proc_macro_attribute]
pub fn gat_desugar(args: TokenStream, val: TokenStream) -> TokenStream {
    if !args.is_empty() {
        return quote!(compile_error!("gat_desugar takes no arguments");).into();
    }

    match desugar::_impl(val.into()) {
        Ok(ts) => ts.into(),
        Err(e) => {
            let err = e.to_string();
            quote!(compile_error!(#err);).into()
        }
    }
}
