use {Symbol, TextRange, TextUnit, fill};
use super::{NodeData, NodeIdx, ParseTree};

/// A builder for creating parse trees by a bottom-up walk over
/// the tree nodes.
#[derive(Debug)]
pub struct BottomUpBuilder {
    nodes: Vec<NodeData>,
    stack: Vec<NodeIdx>,
    pos: TextUnit,
}

impl BottomUpBuilder {
    /// Create a new builder.
    pub fn new() -> BottomUpBuilder {
        BottomUpBuilder {
            nodes: Vec::new(),
            stack: Vec::new(),
            pos: 0.into(),
        }
    }

    /// Completes the building process and yields a `ParseTree.
    /// Panics if there's unmatched `start_internal` calls.
    pub fn finish(mut self, text: String) -> ParseTree {
        assert_eq!(
            self.stack.len(),
            1,
            "some nodes in Builder are unfinished: {:?}",
            self.stack
                .iter()
                .map(|&idx| self.nodes[idx].symbol)
                .collect::<Vec<_>>()
        );
        let nodes = self.nodes;
        let root = self.stack.pop().unwrap();
        ParseTree { nodes, root, text }
    }

    /// Shifts a new leaf node to the stack.
    pub fn shift(&mut self, symbol: Symbol, len: TextUnit) {
        let leaf = NodeData {
            symbol,
            range: TextRange::from_len(self.pos, len),
            parent: None,
            first_child: None,
            next_sibling: None,
        };
        self.pos += len;
        let id = self.new_node(leaf);
        self.stack.push(id);
    }

    /// Reduce top `n_nodes` from the stack to a new node.
    pub fn reduce(&mut self, symbol: Symbol, n_nodes: usize) {
        let n = self.stack.len();
        assert!(0 < n_nodes && n_nodes <= n);
        let range = TextRange::from_to(
            self.nodes[self.stack[n - n_nodes]].range.start(),
            self.nodes[self.stack[n - 1]].range.end(),
        );
        let first_child = self.stack[n - n_nodes];
        let parent = self.new_node(NodeData {
            symbol,
            range,
            parent: None,
            first_child: Some(first_child),
            next_sibling: None,
        });
        {
            let children = &self.stack[n - n_nodes..n];
            for &child in children.iter() {
                fill(&mut self.nodes[child].parent, parent);
            }
            for (&left, &right) in children.iter().zip(children[1..].iter()) {
                fill(&mut self.nodes[left].next_sibling, right);
            }
        }

        for _ in 0..n_nodes {
            self.stack.pop().unwrap();
        }
        self.stack.push(parent);
    }

    fn new_node(&mut self, data: NodeData) -> NodeIdx {
        let id = NodeIdx(self.nodes.len() as u32);
        self.nodes.push(data);
        id
    }
}
