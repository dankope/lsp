use crate::{source::SourceInfo, HashSet, IndexSet, Symbol};

slotmap::new_key_type! { pub struct Scope; }

#[derive(Debug, Default, Clone)]
#[non_exhaustive]
pub struct ScopeData {
    pub source: SourceInfo,
    pub parent: Option<ScopeParent>,
    pub symbols: IndexSet<Symbol>,
    pub hoisted_symbols: HashSet<Symbol>,
}

impl ScopeData {
    /// Iterate over all direct symbols in the scope.
    pub fn iter_symbols(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.symbols
            .iter()
            .copied()
            .chain(self.hoisted_symbols.iter().copied())
    }

    /// Total count of direct symbols in the scope.
    #[must_use]
    pub fn symbol_count(&self) -> usize {
        self.symbols.len() + self.hoisted_symbols.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.symbol_count() == 0
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ScopeParent {
    Scope(Scope),
    Symbol(Symbol),
}

impl ScopeParent {
    #[must_use]
    pub fn as_scope(&self) -> Option<&Scope> {
        if let Self::Scope(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[must_use]
    pub fn as_symbol(&self) -> Option<&Symbol> {
        if let Self::Symbol(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl From<Scope> for ScopeParent {
    fn from(s: Scope) -> Self {
        Self::Scope(s)
    }
}

impl From<Symbol> for ScopeParent {
    fn from(s: Symbol) -> Self {
        Self::Symbol(s)
    }
}
