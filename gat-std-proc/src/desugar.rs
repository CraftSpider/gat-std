use std::error::Error;
use proc_macro2::TokenStream;
use quote::{quote_spanned, ToTokens};
use syn::{Expr, ExprForLoop, ExprIndex, Item, Macro, Stmt};
use syn::parse::{Parse, ParseStream, Parser};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::visit_mut::VisitMut;

struct Visitor {}

impl Visitor {
    fn new() -> Visitor {
        Visitor {}
    }

    fn rewrite_for(&mut self, expr: &ExprForLoop) -> Expr {
        let pat = &expr.pat;
        let iter = &expr.expr;
        let body = &expr.body;
        let val_spanned = quote_spanned!(
            iter.span() =>
            _iter.next()
        );

        let new_expr: Expr = syn::parse2(quote_spanned!(
            expr.span() =>
            #[allow(unused_imports)]
            {
                use ::gat_std::__impl::{ViaLending, ViaCore};
                use ::gat_std::iter::Iterator as _;
                use ::core::iter::Iterator as _;

                let into_iter = ::gat_std::__impl::IntoIter(#iter);
                let mut _iter = into_iter.select().into_iter(into_iter);
                while let Some(#pat) = #val_spanned #body
            }
        )).unwrap();
        new_expr
    }

    fn rewrite_index(&mut self, expr: &ExprIndex) -> Expr {
        syn::parse2::<Expr>(quote_spanned!(
            expr.span() =>
            compile_error!("GAT desugar requires index have an explicit `&` or `&mut`")
        )).unwrap()
    }

    fn rewrite_ref_index(&mut self, expr: &ExprIndex, mutability: bool) -> Expr {
        let val_expr = &expr.expr;
        let idx_expr = &expr.index;

        let ts = if mutability {
            quote_spanned!(expr.span() => {
                ::gat_std::ops::IndexMut::index_mut(&mut (#val_expr), #idx_expr)
            })
        } else {
            quote_spanned!(expr.span() => {
                ::gat_std::ops::Index::index(&(#val_expr), #idx_expr)
            })
        };

        syn::parse2::<Expr>(ts).unwrap()
    }

    fn rewrite_assign_index(&mut self, expr: &ExprIndex) -> Expr {
        let val_expr = &expr.expr;
        let idx_expr = &expr.index;

        syn::parse2::<Expr>(quote_spanned!(expr.span() =>
            *::gat_std::ops::IndexMut::index_mut(&mut (#val_expr), #idx_expr)
        )).unwrap()
    }
}

impl VisitMut for Visitor {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        match expr {
            Expr::ForLoop(f) => {
                *expr = self.rewrite_for(f);
            }
            Expr::Reference(r) => {
                if let Expr::Index(i) = &*r.expr {
                    *expr = self.rewrite_ref_index(i, r.mutability.is_some())
                }
            }
            Expr::Assign(a) => {
                if let Expr::Index(i) = &*a.left {
                    a.left = Box::new(self.rewrite_assign_index(i))
                }
            }
            Expr::Index(i) => {
                *expr = self.rewrite_index(i);
            }
            _ => (),
        }

        syn::visit_mut::visit_expr_mut(self, expr);
    }

    fn visit_macro_mut(&mut self, mac: &mut Macro) {
        let args = (Punctuated::<Expr, Comma>::parse_separated_nonempty)
            .parse2(mac.tokens.clone());

        if let Ok(mut args) = args {
            for arg in &mut args {
                self.visit_expr_mut(arg);
            }
            mac.tokens = args.into_token_stream();
        }
    }
}

pub enum ItemOrStmt {
    Item(Item),
    Stmt(Stmt),
}

impl Parse for ItemOrStmt {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Item>()
            .map(ItemOrStmt::Item)
            .or_else(|_| input.parse::<Stmt>().map(ItemOrStmt::Stmt))
    }
}

impl ToTokens for ItemOrStmt {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ItemOrStmt::Item(i) => i.to_tokens(tokens),
            ItemOrStmt::Stmt(s) => s.to_tokens(tokens),
        }
    }
}

pub fn _impl(val: TokenStream) -> Result<TokenStream, Box<dyn Error>> {
    let mut is = syn::parse2::<ItemOrStmt>(val)?;
    let mut visitor = Visitor::new();
    match &mut is {
        ItemOrStmt::Item(i) => visitor.visit_item_mut(i),
        ItemOrStmt::Stmt(s) => visitor.visit_stmt_mut(s),
    }
    Ok(is.into_token_stream())
}
