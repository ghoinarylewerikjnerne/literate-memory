use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input, Fields, Ident, ImplItem, Expr, Item,
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

    // Find the main struct (if any) and clone its info.
    let main_struct = match macro_input.items.iter().find_map(|item| {
        if let Item::Struct(s) = item { Some(s.clone()) } else { None }
    }) {
        Some(s) => s,
        None => {
            // No struct found, return original input
            let items = macro_input.items;
            return TokenStream::from(quote! { #(#items)* });
        }
    };

    let struct_ident = &main_struct.ident;
    let fields = match &main_struct.fields {
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
    let field_idents: Vec<&Ident> = fields
        .iter()
        .map(|f| f.ident.as_ref().expect("named fields should have idents"))
        .collect();

    // Collect all method names with `self` receivers from all relevant impl blocks first.
    let all_method_names: Vec<Ident> = macro_input.items.iter().flat_map(|item| {
        if let Item::Impl(impl_def) = item {
            if let syn::Type::Path(type_path) = &*impl_def.self_ty {
                if type_path.path.segments.last().map_or(false, |s| s.ident == *struct_ident) {
                    return impl_def.items.iter().filter_map(|item| {
                        if let ImplItem::Fn(method) = item {
                            if method.sig.receiver().is_some() {
                                return Some(method.sig.ident.clone());
                            }
                        }
                        None
                    }).collect::<Vec<_>>();
                }
            }
        }
        Vec::new()
    }).collect();

    // Iterate through all items mutably, find impls for our struct, and transform them.
    for item in &mut macro_input.items {
        if let Item::Impl(impl_def) = item {
            if let syn::Type::Path(type_path) = &*impl_def.self_ty {
                if type_path.path.segments.last().map_or(false, |s| s.ident == *struct_ident) {
                    // This is a matching impl block, so we process it.
                    for item in &mut impl_def.items {
                        if let ImplItem::Fn(method) = item {
                            if method.sig.receiver().is_some() {
                                // Add the allow attribute
                                let allow_attr: syn::Attribute = syn::parse_quote! { #[allow(unused_variables)] };
                                method.attrs.push(allow_attr);

                                // Insert field bindings
                                for field_ident in field_idents.iter().rev() {
                                    let let_stmt: syn::Stmt = syn::parse_quote! {
                                        let #field_ident = &self.#field_ident;
                                    };
                                    method.block.stmts.insert(0, let_stmt);
                                }

                                // Rewrite method calls
                                let mut visitor = MethodCallVisitor {
                                    method_names: &all_method_names,
                                };
                                visitor.visit_block_mut(&mut method.block);
                            }
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
