use std::{
    fmt::Write,
};

use {PtNodeId};

mod traits;
mod search;

pub use self::{
    traits::{AsParseTree, GetText, GetSymbol},
    search::{
        is_leaf,
        children, ancestors,
        find_leaf_at_offset,
        find_covering_node
    },
};

/// Debug representation of a subtree at `node`.
pub fn debug_dump<'t>(
    tree: &(impl AsParseTree + GetText + GetSymbol),
) -> String {
    struct Dumper<'a, T: AsParseTree + GetText + GetSymbol + 'a> {
        buff: String,
        tree: &'a T,
    }

    impl<'a, T: AsParseTree + GetText + GetSymbol + 'a> Dumper<'a, T> {
        fn go(&mut self, node_id: PtNodeId, level: usize) {
            let node = &self.tree.as_parse_tree()[node_id];

            for _ in 0..level {
                self.buff.push_str("  ");
            }

            write!(self.buff, "{}@{:?}", self.tree.get_symbol(node_id), node.range())
                .unwrap();

            if node.first_child().is_none() {
                let text = self.tree.get_text(node_id);
                if !text.chars().all(char::is_whitespace) {
                    write!(self.buff, " {:?}", text)
                        .unwrap();
                }
            }

            self.buff.push('\n');
            for child in children(self.tree.as_parse_tree(), node_id) {
                self.go(child, level + 1)
            }
        }
    }

    let mut dumper = Dumper { buff: String::new(), tree };
    dumper.go(tree.as_parse_tree().root(), 0);
    return dumper.buff;
}
