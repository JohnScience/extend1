#![doc = include_str!("../README.md")]
use proc_macro2 as pm2;

/// Check out the [`crate`] documentation for more information.
pub trait Extend1<T> {
    /// Check out the [`crate`] documentation for more information.
    fn extend1(&mut self, item: T);
}

impl Extend1<pm2::TokenTree> for pm2::TokenStream {
    fn extend1(&mut self, item: pm2::TokenTree) {
        self.extend(core::iter::once(item));
    }
}

impl Extend1<pm2::Group> for pm2::TokenStream {
    fn extend1(&mut self, item: pm2::Group) {
        self.extend1(pm2::TokenTree::Group(item));
    }
}

impl Extend1<pm2::Punct> for pm2::TokenStream {
    fn extend1(&mut self, item: pm2::Punct) {
        self.extend1(pm2::TokenTree::Punct(item));
    }
}

impl Extend1<pm2::Literal> for pm2::TokenStream {
    fn extend1(&mut self, item: pm2::Literal) {
        self.extend1(pm2::TokenTree::Literal(item));
    }
}

impl Extend1<pm2::Ident> for pm2::TokenStream {
    fn extend1(&mut self, item: pm2::Ident) {
        self.extend1(pm2::TokenTree::Ident(item));
    }
}
