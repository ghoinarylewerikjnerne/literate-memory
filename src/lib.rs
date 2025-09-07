use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input, Fields, FnArg, Ident, ImplItem, ImplItemFn, ItemImpl, ItemStruct,
};

struct StructAndImpl {
    struct_def: ItemStruct,
    impl_def: ItemImpl,
}

impl Parse for StructAndImpl {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(StructAndImpl {
            struct_def: input.parse()?,
            impl_def: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn bind_fields(input: TokenStream) -> TokenStream {
    let both = parse_macro_input!(input as StructAndImpl);
    let struct_def = both.struct_def;
    let mut impl_def = both.impl_def;

    let struct_ident = &struct_def.ident;

    let fields = match &struct_def.fields {
        Fields::Named(named) => &named.named,
        _ => {
            return syn::Error::new_spanned(
                struct_ident,
                "bind_fields only supports structs with named fields",
            )
            .to_compile_error()
            .into();
        }
    };

    let idents: Vec<&Ident> = fields
        .iter()
        .map(|f| f.ident.as_ref().expect("named fields should have idents"))
        .collect();

    for item in &mut impl_def.items {
        if let ImplItem::Fn(ImplItemFn { sig, block, .. }) = item {
            if sig.receiver().is_some() {
                let is_ref = sig.inputs.iter().any(|arg| match arg {
                    FnArg::Receiver(rec) => rec.reference.is_some(),
                    _ => false,
                });

                let binding = if is_ref {
                    quote! { let #struct_ident { #(#idents),* } = *self; }
                } else {
                    quote! { let #struct_ident { #(#idents),* } = self; }
                };
                let let_stmt: syn::Stmt = syn::parse_quote! { #binding };
                block.stmts.insert(0, let_stmt);
            }
        }
    }

    TokenStream::from(quote! {
        #struct_def
        #impl_def
    })
}
