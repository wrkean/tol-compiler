use crate::{
    ast::stmt::Stmt,
    module::Module,
    sema::{analyzer_ctx::AnalyzerCtx, name_resolver::NameResolver, type_checker::TypeChecker},
};

pub mod analyzer_ctx;
pub mod name_resolver;
pub mod type_checker;

pub struct SemanticAnalyzer<'m> {
    analyzer_ctx: AnalyzerCtx,
    modul: &'m mut Module,
}

impl<'m> SemanticAnalyzer<'m> {
    pub fn new(modul: &'m mut Module) -> Self {
        Self {
            analyzer_ctx: AnalyzerCtx::new(),
            modul,
        }
    }

    pub fn run(&mut self) {
        self.resolve_names();
        self.type_check();
    }

    fn resolve_names(&mut self) {
        let mut resolver = NameResolver::new(&mut self.analyzer_ctx, self.modul);
        resolver.run();
    }

    fn type_check(&mut self) {
        let mut type_checker = TypeChecker::new(&mut self.analyzer_ctx, self.modul);
        type_checker.run();
    }
}
