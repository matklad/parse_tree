use {Symbol, TextRange, TextUnit, fill};
use super::{PtNode, PtNodeId, ParseTree};

/// A builder for creating parse trees by a top-down walk over
/// the tree nodes. Each `start_internal` call must be paired with a
/// `finish_internal` call. Nodes created within a pair of
/// `start_internal` / `finish_internal` calls become children of
/// the internal node.
#[derive(Debug, Default)]
pub struct TopDownBuilder {
    nodes: Vec<PtNode>,
    in_progress: Vec<(PtNodeId, Option<PtNodeId>)>,
    pos: TextUnit,
}

impl TopDownBuilder {
    /// Create a new builder.
    pub fn new() -> TopDownBuilder {
        Self::default()
    }

    /// Completes the building process and yields a `ParseTree.
    /// Panics if there's unmatched `start_internal` calls.
    pub fn finish(self) -> ParseTree {
        assert!(
            self.in_progress.is_empty(),
            "some nodes in Builder are unfinished: {:?}",
            self.in_progress
                .iter()
                .map(|&(idx, _)| self.nodes[idx].symbol)
                .collect::<Vec<_>>()
        );
        ParseTree { nodes: self.nodes, root: PtNodeId(0) }
    }

    /// Creates a new leaf node.
    pub fn leaf(&mut self, symbol: Symbol, len: TextUnit) {
        let leaf = PtNode {
            symbol,
            range: TextRange::offset_len(self.pos, len),
            parent: None,
            first_child: None,
            next_sibling: None,
        };
        self.pos += len;
        let id = self.push_child(leaf);
        self.add_len(id);
    }

    /// Start a new internal node.
    pub fn start_internal(&mut self, symbol: Symbol) {
        let node = PtNode {
            symbol,
            range: TextRange::offset_len(self.pos, 0.into()),
            parent: None,
            first_child: None,
            next_sibling: None,
        };
        let id = if self.in_progress.is_empty() {
            self.new_node(node)
        } else {
            self.push_child(node)
        };
        self.in_progress.push((id, None))
    }

    /// Complete an internal node.
    /// Panics if there's no matching `start_internal` call.
    pub fn finish_internal(&mut self) {
        let (id, _) = self.in_progress
            .pop()
            .expect("trying to complete a node, but there are no in-progress nodes");
        if !self.in_progress.is_empty() {
            self.add_len(id);
        }
    }

    fn new_node(&mut self, data: PtNode) -> PtNodeId {
        let id = PtNodeId(self.nodes.len() as u32);
        self.nodes.push(data);
        id
    }

    fn push_child(&mut self, mut child: PtNode) -> PtNodeId {
        child.parent = Some(self.current_id());
        let id = self.new_node(child);
        {
            let (parent, sibling) = *self.in_progress.last().unwrap();
            let slot = if let Some(idx) = sibling {
                &mut self.nodes[idx].next_sibling
            } else {
                &mut self.nodes[parent].first_child
            };
            fill(slot, id);
        }
        self.in_progress.last_mut().unwrap().1 = Some(id);
        id
    }

    fn add_len(&mut self, child: PtNodeId) {
        let range = self.nodes[child].range;
        grow(&mut self.current_parent().range, range);
    }

    fn current_id(&self) -> PtNodeId {
        self.in_progress.last().unwrap().0
    }

    fn current_parent(&mut self) -> &mut PtNode {
        let idx = self.current_id();
        &mut self.nodes[idx]
    }
}

fn grow(left: &mut TextRange, right: TextRange) {
    assert_eq!(left.end(), right.start());
    *left = TextRange::from_to(left.start(), right.end())
}
