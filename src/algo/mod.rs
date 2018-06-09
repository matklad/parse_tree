use std::{
    io,
    fmt::Write,
};

use {PtNode, PtNodeId};

mod traits;
mod search;

pub use self::{
    traits::{AsParseTree, WriteText, WriteSymbol},
    search::{
        is_leaf,
        children, ancestors,
        find_leaf_at_offset,
    },
};

/// Debug representation of a subtree at `node`.
pub fn debug_dump<'t>(
    tree: &(impl AsParseTree + WriteText + WriteSymbol),
) -> String {
    struct Dumper<'a, T: AsParseTree + WriteText + WriteSymbol + 'a> {
        buff: String,
        tree: &'a T,
    }

    impl<'a, T: AsParseTree + WriteText + WriteSymbol + 'a> Dumper<'a, T> {
        fn go(&mut self, node_id: PtNodeId, level: usize) {
            let node = &self.tree.as_parse_tree()[node_id];

            for _ in 0..level {
                self.buff.push_str("  ");
            }

            self.tree.write_symbol(node_id, &mut self.buff)
                .unwrap();
            write!(self.buff, "@{:?}", node.range())
                .unwrap();

            if node.first_child().is_none() {
                let text = self.tree.pt_node_text(node_id);
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
