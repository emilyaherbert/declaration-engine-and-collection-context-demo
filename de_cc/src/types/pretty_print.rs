use crate::collection_context::collection_context::CollectionContext;

pub(crate) trait PrettyPrint {
    fn pretty_print(&self, cc: &CollectionContext) -> String;
}
