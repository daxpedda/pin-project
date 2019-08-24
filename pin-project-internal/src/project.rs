use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{
    parse::Nothing,
    punctuated::Punctuated,
    token::Or,
    visit_mut::{self, VisitMut},
    *,
};

use crate::utils::{proj_generics, proj_ident, proj_lifetime_name, VecExt, DEFAULT_LIFETIME_NAME};

/// The attribute name.
const NAME: &str = "project";

pub(super) fn attribute(input: TokenStream) -> TokenStream {
    parse(input).unwrap_or_else(|e| e.to_compile_error())
}

fn parse(input: TokenStream) -> Result<TokenStream> {
    let mut stmt = syn::parse2(input)?;
    match &mut stmt {
        Stmt::Expr(Expr::Match(expr)) | Stmt::Semi(Expr::Match(expr), _) => {
            expr.replace(&mut Register::default())
        }
        Stmt::Local(local) => local.replace(&mut Register::default()),
        Stmt::Item(Item::Fn(ItemFn { block, .. })) => Dummy.visit_block_mut(block),
        Stmt::Item(Item::Impl(item)) => item.replace(&mut Register::default()),
        _ => {}
    }

    Ok(stmt.into_token_stream())
}

trait Replace {
    /// Replace the original ident with the ident of projected type.
    fn replace(&mut self, register: &mut Register);
}

impl Replace for ItemImpl {
    fn replace(&mut self, _: &mut Register) {
        let PathSegment { ident, arguments } = match &mut *self.self_ty {
            Type::Path(TypePath { qself: None, path }) => path.segments.last_mut().unwrap(),
            _ => return,
        };

        replace_ident(ident);

        let mut lifetime_name = String::from(DEFAULT_LIFETIME_NAME);
        proj_lifetime_name(&mut lifetime_name, &self.generics.params);
        self.items
            .iter_mut()
            .filter_map(|i| if let ImplItem::Method(i) = i { Some(i) } else { None })
            .for_each(|item| proj_lifetime_name(&mut lifetime_name, &item.sig.generics.params));
        let lifetime = Lifetime::new(&lifetime_name, Span::call_site());

        proj_generics(&mut self.generics, syn::parse_quote!(#lifetime));

        match arguments {
            PathArguments::None => {
                *arguments = PathArguments::AngleBracketed(syn::parse_quote!(<#lifetime>));
            }
            PathArguments::AngleBracketed(args) => {
                args.args.insert(0, syn::parse_quote!(#lifetime));
            }
            PathArguments::Parenthesized(_) => unreachable!(),
        }
    }
}

impl Replace for Local {
    fn replace(&mut self, register: &mut Register) {
        // We need to use two 'if let' expressions
        // here, since we can't pattern-match through
        // a Box
        if let Some((_, expr)) = &mut self.init {
            if let Expr::Match(expr) = &mut **expr {
                expr.replace(register);
            }
        }

        // TODO: If `pat` is a replaceable pattern, submit an error and
        // suggest splitting the initializer into separate let bindings.
        self.pat.replace(register);
    }
}

impl Replace for ExprMatch {
    fn replace(&mut self, register: &mut Register) {
        self.arms.iter_mut().for_each(|Arm { pat, .. }| pat.replace(register))
    }
}

impl Replace for Punctuated<Pat, Or> {
    fn replace(&mut self, register: &mut Register) {
        self.iter_mut().for_each(|pat| pat.replace(register));
    }
}

impl Replace for Pat {
    fn replace(&mut self, register: &mut Register) {
        match self {
            Pat::Ident(PatIdent { subpat: Some((_, pat)), .. })
            | Pat::Reference(PatReference { pat, .. })
            | Pat::Box(PatBox { pat, .. })
            | Pat::Type(PatType { pat, .. }) => pat.replace(register),

            Pat::Struct(PatStruct { path, .. })
            | Pat::TupleStruct(PatTupleStruct { path, .. })
            | Pat::Path(PatPath { qself: None, path, .. }) => path.replace(register),

            _ => {}
        }
    }
}

impl Replace for Path {
    fn replace(&mut self, register: &mut Register) {
        let len = match self.segments.len() {
            // 1: struct
            // 2: enum
            len @ 1 | len @ 2 => len,
            // other path
            _ => return,
        };

        if register.0.is_none() || register.eq(&self.segments[0].ident, len) {
            register.update(&self.segments[0].ident, len);
            replace_ident(&mut self.segments[0].ident);
        }
    }
}

fn replace_ident(ident: &mut Ident) {
    *ident = proj_ident(ident);
}

#[derive(Default)]
struct Register(Option<(Ident, usize)>);

impl Register {
    fn update(&mut self, ident: &Ident, len: usize) {
        if self.0.is_none() {
            self.0 = Some((ident.clone(), len));
        }
    }

    fn eq(&self, ident: &Ident, len: usize) -> bool {
        match &self.0 {
            Some((i, l)) => *l == len && ident == i,
            None => false,
        }
    }
}

// =================================================================================================
// visitor

struct Dummy;

impl VisitMut for Dummy {
    fn visit_stmt_mut(&mut self, stmt: &mut Stmt) {
        macro_rules! parse_attr {
            ($this:expr) => {{
                $this.attrs.find_remove(NAME).map_or_else(
                    || Ok(()),
                    |attr| {
                        syn::parse2::<Nothing>(attr.tokens)
                            .map(|_| $this.replace(&mut Register::default()))
                    },
                )
            }};
        }

        visit_mut::visit_stmt_mut(self, stmt);

        if let Err(e) = match stmt {
            Stmt::Expr(Expr::Match(expr)) | Stmt::Semi(Expr::Match(expr), _) => parse_attr!(expr),
            Stmt::Local(local) => parse_attr!(local),
            _ => return,
        } {
            *stmt = Stmt::Expr(syn::parse2(e.to_compile_error()).unwrap())
        }
    }

    fn visit_item_mut(&mut self, _: &mut Item) {
        // Do not recurse into nested items.
    }
}
