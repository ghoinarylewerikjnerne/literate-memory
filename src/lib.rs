use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input, Fields, FnArg, Ident, ImplItem, ImplItemFn, Expr, Item,
};
use syn::visit_mut::{self, VisitMut};

struct MacroInput {
    items: Vec<Item>,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut items = Vec::new();
        while !input.is_empty() {
            items.push(input.parse()?);
        }
        Ok(MacroInput { items })
    }
}

#[proc_macro]
pub fn bind_fields(input: TokenStream) -> TokenStream {
    let mut macro_input = parse_macro_input!(input as MacroInput);

    // Find the main struct and its index
    let struct_info = macro_input.items.iter().enumerate().find_map(|(i, item)| {
        if let Item::Struct(s) = item { Some((i, s.ident.clone())) } else { None }
    });

    if let Some((struct_index, struct_ident)) = struct_info {
        // Find the corresponding impl block and its index
        let impl_index = macro_input.items.iter().position(|item| {
            if let Item::Impl(i) = item {
                if let syn::Type::Path(type_path) = &*i.self_ty {
                    return type_path.path.is_ident(&struct_ident);
                }
            }
            false
        });

        if let Some(impl_index) = impl_index {
            // We have both indices, now we can get mutable references
            let (struct_item, impl_item) = if struct_index < impl_index {
                let (left, right) = macro_input.items.split_at_mut(impl_index);
                (&mut left[struct_index], &mut right[0])
            } else {
                let (left, right) = macro_input.items.split_at_mut(struct_index);
                (&mut right[0], &mut left[impl_index])
            };

            if let (Item::Struct(struct_def), Item::Impl(impl_def)) = (struct_item, impl_item) {
                let fields = match &struct_def.fields {
                    Fields::Named(named) => &named.named,
                    _ => {
                        return syn::Error::new_spanned(
                            &struct_ident,
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

            let method_names: Vec<Ident> = impl_def
                .items
                .iter()
                .filter_map(|item| {
                    if let ImplItem::Fn(method) = item {
                        if method.sig.receiver().is_some() {
                            Some(method.sig.ident.clone())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
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

                        // Traverse the method body to rewrite method calls
                        let mut visitor = MethodCallVisitor {
                            method_names: &method_names,
                        };
                        visitor.visit_block_mut(block);
                    }
                }
            }
            }
        }
    }

    let items = macro_input.items;
    TokenStream::from(quote! {
        #(#items)*
    })
}

struct MethodCallVisitor<'a> {
    method_names: &'a [Ident],
}

impl<'a> VisitMut for MethodCallVisitor<'a> {
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        if let Expr::Call(expr_call) = node {
            if let Expr::Path(expr_path) = &*expr_call.func {
                if expr_path.path.segments.len() == 1 {
                    let method_name = &expr_path.path.segments[0].ident;
                    if self.method_names.contains(method_name) {
                        let args = &expr_call.args;
                        *node = syn::parse_quote! {
                            self.#method_name(#args)
                        };
                        // We have rewritten the node, no need to traverse further down this branch
                        return;
                    }
                }
            }
        }

        // continue traversing the syntax tree if we didn't rewrite
        visit_mut::visit_expr_mut(self, node);
    }
}
