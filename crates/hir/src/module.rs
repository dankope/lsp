#![allow(clippy::unsafe_derive_deserialize)]

use crate::{scope::ScopeData, symbol::*, syntax::SyntaxInfo, Scope};
use core::ops;
use rhai_rowan::{
    ast::{AstNode, Expr, Rhai, Stmt},
    syntax::SyntaxNode,
};
use serde::{Deserialize, Serialize};
use slotmap::SlotMap;

mod edit;
mod query;
mod remove;
mod infer;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    pub root_scope: Scope,
    pub syntax: Option<SyntaxInfo>,
    pub scopes: SlotMap<Scope, ScopeData>,
    pub symbols: SlotMap<Symbol, SymbolData>,
}

impl ops::Index<Scope> for Module {
    type Output = ScopeData;

    fn index(&self, index: Scope) -> &Self::Output {
        self.scopes.get(index).unwrap()
    }
}

impl ops::Index<Symbol> for Module {
    type Output = SymbolData;

    fn index(&self, index: Symbol) -> &Self::Output {
        self.symbols.get(index).unwrap()
    }
}

impl Module {
    pub fn scopes(&self) -> impl Iterator<Item = (Scope, &ScopeData)> {
        self.scopes.iter()
    }

    #[must_use]
    pub fn contains_scope(&self, scope: Scope) -> bool {
        self.scopes.contains_key(scope)
    }

    #[must_use]
    pub fn scope_count(&self) -> usize {
        self.scopes.len()
    }

    pub fn symbols(&self) -> impl Iterator<Item = (Symbol, &SymbolData)> {
        self.symbols.iter()
    }

    #[must_use]
    pub fn contains_symbol(&self, symbol: Symbol) -> bool {
        self.symbols.contains_key(symbol)
    }

    #[must_use]
    pub fn symbol_count(&self) -> usize {
        self.symbols.len()
    }
}

#[allow(dead_code)]
impl Module {
    pub(crate) fn scope_unchecked(&self, scope: Scope) -> &ScopeData {
        // safety: Internal, we guarantee that the scope exists.
        unsafe { self.scopes.get_unchecked(scope) }
    }

    pub(crate) fn scope_unchecked_mut(&mut self, scope: Scope) -> &mut ScopeData {
        // safety: Internal, we guarantee that the scope exists.
        unsafe { self.scopes.get_unchecked_mut(scope) }
    }

    pub(crate) fn symbol_unchecked(&self, symbol: Symbol) -> &SymbolData {
        // safety: Internal, we guarantee that the symbol exists.
        unsafe { self.symbols.get_unchecked(symbol) }
    }

    pub(crate) fn symbol_unchecked_mut(&mut self, symbol: Symbol) -> &mut SymbolData {
        // safety: Internal, we guarantee that the symbol exists.
        unsafe { self.symbols.get_unchecked_mut(symbol) }
    }
}
