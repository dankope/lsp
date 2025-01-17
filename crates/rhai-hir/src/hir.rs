mod add;
mod errors;
mod query;
mod remove;
mod resolve;

use core::ops;

use crate::{
    module::ModuleData,
    scope::ScopeData,
    source::{Source, SourceData},
    symbol::*,
    ty::{Type, TypeData},
    Module, Scope,
};

use rhai_rowan::syntax::SyntaxNode;
use slotmap::{Key, SlotMap};
use url::Url;

#[derive(Debug, Clone)]
pub struct Hir {
    static_module: Module,
    virtual_source: Source,
    modules: SlotMap<Module, ModuleData>,
    scopes: SlotMap<Scope, ScopeData>,
    symbols: SlotMap<Symbol, SymbolData>,
    sources: SlotMap<Source, SourceData>,
    types: SlotMap<Type, TypeData>,
    builtin_types: BuiltinTypes,
}

impl Default for Hir {
    fn default() -> Self {
        let mut this = Self {
            static_module: Default::default(),
            virtual_source: Default::default(),
            modules: Default::default(),
            scopes: Default::default(),
            symbols: Default::default(),
            sources: Default::default(),
            types: Default::default(),
            builtin_types: BuiltinTypes::uninit(),
        };
        this.prepare();
        this
    }
}

static_assertions::assert_impl_all!(Hir: Send, Sync);

impl Hir {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Hir {
    pub fn clear(&mut self) {
        self.symbols.clear();
        self.scopes.clear();
        self.modules.clear();
        self.sources.clear();
        self.types.clear();
        self.builtin_types = BuiltinTypes::uninit();
        self.static_module = Module::null();
        self.prepare();
    }

    #[must_use]
    #[inline]
    pub fn symbol(&self, symbol: Symbol) -> Option<&SymbolData> {
        self.symbols.get(symbol)
    }

    #[inline]
    pub fn symbols(&self) -> impl Iterator<Item = (Symbol, &SymbolData)> {
        self.symbols.iter()
    }

    #[must_use]
    #[inline]
    pub fn scope(&self, scope: Scope) -> Option<&ScopeData> {
        self.scopes.get(scope)
    }

    #[inline]
    pub fn scopes(&self) -> impl Iterator<Item = (Scope, &ScopeData)> {
        self.scopes.iter()
    }

    #[must_use]
    #[inline]
    pub const fn static_module(&self) -> Module {
        self.static_module
    }

    #[must_use]
    #[inline]
    pub fn module(&self, module: Module) -> Option<&ModuleData> {
        self.modules.get(module)
    }

    #[inline]
    pub fn modules(&self) -> impl Iterator<Item = (Module, &ModuleData)> {
        self.modules.iter()
    }

    #[inline]
    pub fn sources(&self) -> impl Iterator<Item = (Source, &SourceData)> {
        self.sources.iter()
    }

    #[must_use]
    pub fn source_of(&self, url: &Url) -> Option<Source> {
        self.sources()
            .find_map(|(s, data)| if data.url == *url { Some(s) } else { None })
    }

    #[inline]
    fn symbol_mut(&mut self, symbol: Symbol) -> &mut SymbolData {
        self.symbols.get_mut(symbol).unwrap()
    }

    #[inline]
    fn scope_mut(&mut self, scope: Scope) -> &mut ScopeData {
        self.scopes.get_mut(scope).unwrap()
    }

    #[inline]
    fn source_mut(&mut self, source: Source) -> &mut SourceData {
        self.sources.get_mut(source).unwrap()
    }

    #[inline]
    fn module_mut(&mut self, module: Module) -> &mut ModuleData {
        self.modules.get_mut(module).unwrap()
    }

    fn prepare(&mut self) {
        self.ensure_static_module();
        self.ensure_virtual_source();
        self.ensure_builtin_types();
    }
}

impl ops::Index<Scope> for Hir {
    type Output = ScopeData;

    fn index(&self, index: Scope) -> &Self::Output {
        self.scopes.get(index).unwrap()
    }
}

impl ops::Index<Symbol> for Hir {
    type Output = SymbolData;

    fn index(&self, index: Symbol) -> &Self::Output {
        let sym = self.symbols.get(index).unwrap();

        if let SymbolKind::Virtual(VirtualSymbol::Proxy(proxy)) = &sym.kind {
            return self.symbols.get(proxy.target).unwrap();
        }

        sym
    }
}

impl ops::Index<Module> for Hir {
    type Output = ModuleData;

    fn index(&self, index: Module) -> &Self::Output {
        self.modules.get(index).unwrap()
    }
}

impl ops::Index<Source> for Hir {
    type Output = SourceData;

    fn index(&self, index: Source) -> &Self::Output {
        self.sources.get(index).unwrap()
    }
}

impl ops::Index<Type> for Hir {
    type Output = TypeData;

    fn index(&self, index: Type) -> &Self::Output {
        self.types.get(index).unwrap()
    }
}

/// Built-in (primitive) types are treated as any other type
/// but always exist in the HIR and cannot be removed.
///
/// This struct keeps track of their keys.
#[derive(Debug, Clone, Copy)]
pub struct BuiltinTypes {
    pub module: Type,
    pub int: Type,
    pub float: Type,
    pub bool: Type,
    pub char: Type,
    pub string: Type,
    pub timestamp: Type,
    pub void: Type,
    pub unknown: Type,
    pub never: Type,
}

impl BuiltinTypes {
    fn uninit() -> Self {
        Self {
            module: Default::default(),
            int: Default::default(),
            float: Default::default(),
            bool: Default::default(),
            char: Default::default(),
            string: Default::default(),
            timestamp: Default::default(),
            void: Default::default(),
            unknown: Default::default(),
            never: Default::default(),
        }
    }

    #[must_use]
    fn is_uninit(&self) -> bool {
        // We don't check all of the fields,
        // as this is not exposed and we always
        // initialize all of them.
        self.module.is_null()
    }
}
