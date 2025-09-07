use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, DeriveInput, Fields, ItemImpl, ImplItem, ImplItemMethod, FnArg, Type, Ident,
    spanned::Spanned,
};

/// Derive that emits a helper macro which expands to `let StructName { field1, field2, .. } = ...;`
#[proc_macro_derive(BindFields)]
pub fn derive_bind_fields(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_ident = input.ident;

    // Only support named-field structs for the example
    let fields = match input.data {
        syn::Data::Struct(syn::DataStruct { fields: Fields::Named(named), .. }) => named.named,
        _ => {
            return syn::Error::new_spanned(
                struct_ident,
                "BindFields only supports structs with named fields",
            )
            .to_compile_error()
            .into();
        }
    };

    let idents: Vec<Ident> = fields
        .iter()
        .map(|f| f.ident.clone().expect("named fields should have idents"))
        .collect();

    // helper macro name: __bind_fields_for_<StructName>
    let macro_name = Ident::new(&format!("__bind_fields_for_{}", struct_ident), struct_ident.span());

    let expanded = quote! {
        // hidden helper macro generated next to the struct
        #[doc(hidden)]
        macro_rules! #macro_name {
            // when caller passes an expression that is an owned/self:
            ($s:expr) => {
                let #struct_ident { #(#idents),* } = $s;
            };
            // when caller passes a reference (&self)
            (&$s:expr) => {
                let #struct_ident { #(#idents),* } = *$s;
            };
            (&mut $s:expr) => {
                let #struct_ident { #(#idents),* } = *$s;
            };
        }
    };

    TokenStream::from(expanded)
}

/// Attribute to put on an `impl` block; it injects a call to the generated helper
/// macro at the top of every method that has a receiver (`self`, `&self`, `&mut self`).
#[proc_macro_attribute]
pub fn bind_methods(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemImpl);

    // figure out the type we are impl'ing for: expect a plain path type like `A`
    let type_ident = match *input.self_ty.clone() {
        Type::Path(tp) => {
            tp.path.segments.last().unwrap().ident.clone()
        }
        _ => {
            return syn::Error::new_spanned(input.self_ty, "Unsupported impl target type")
                .to_compile_error()
                .into();
        }
    };

    let macro_name = Ident::new(&format!("__bind_fields_for_{}", type_ident), type_ident.span());

    // walk methods and inject a call at top of body if method has a receiver
    for item in &mut input.items {
        if let ImplItem::Method(ImplItemMethod { sig, block, .. }) = item {
            // check if method has a receiver
            let has_receiver = sig.receiver().is_some();
            if !has_receiver {
                continue;
            }

            // decide whether to pass `self` or `&self` to the macro.
            // If the receiver is `&self` or `&mut self` we call the macro with `(&self)` (so the macro uses `*`).
            // If the receiver is `self` (by value) we call with `(self)`.
            let receiver_by_ref = sig
                .inputs
                .iter()
                .find_map(|arg| {
                    if let FnArg::Receiver(rcv) = arg {
                        // rcv.reference is Option<( &Token, Option<Lifetime> )>
                        Some(rcv.reference.is_some())
                    } else {
                        None
                    }
                })
                .unwrap_or(false);

            let injection: syn::Stmt = if receiver_by_ref {
                syn::parse_quote! {
                    #macro_name!(&self);
                }
            } else {
                syn::parse_quote! {
                    #macro_name!(self);
                }
            };

            // insert at beginning
            block.stmts.insert(0, injection);
        }
    }

    TokenStream::from(quote! { #input })
}
