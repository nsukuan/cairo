use diagnostics::DiagnosticLocation;
use syntax::node::ids::SyntaxStablePtrId;
use syntax::node::TypedSyntaxNode;

use crate::db::DefsGroup;
use crate::ids::ModuleId;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct StableLocation {
    module_id: ModuleId,
    stable_ptr: SyntaxStablePtrId,
}
impl StableLocation {
    pub fn from_ast<TNode: TypedSyntaxNode>(module_id: ModuleId, node: &TNode) -> Self {
        Self { module_id, stable_ptr: node.as_syntax_node().stable_ptr() }
    }

    pub fn diagnostic_location(&self, db: &(dyn DefsGroup + 'static)) -> DiagnosticLocation {
        let file_id = db.module_file(self.module_id).expect("Module in diagnostic does not exist");
        let syntax_node = db
            .file_syntax(file_id)
            .expect("File for diagnostic not found")
            .as_syntax_node()
            .lookup_ptr(db.as_syntax_group(), self.stable_ptr);
        DiagnosticLocation { file_id, span: syntax_node.span(db.as_syntax_group()) }
    }
}