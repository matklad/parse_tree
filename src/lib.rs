//! `parse_tree` is a library to represent so-called parse tree.
//! A parse tree is a non-abstract AST: it's a generic syntax tree
//! which remembers all whitespace, comments and other trivia.
extern crate serde;

use std::{ops};

mod text;
mod top_down_builder;
mod bottom_up_builder;
pub mod algo;

pub use text::{TextRange, TextUnit};
pub use top_down_builder::TopDownBuilder;
pub use bottom_up_builder::BottomUpBuilder;

/// A type of a syntactic construct, including both leaf tokens
/// and composite nodes, like "a comma" or "a function".
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Symbol(pub u16);

/// A token of source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token {
    /// The kind of token.
    pub symbol: Symbol,
    /// The length of the token.
    pub len: TextUnit,
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct PtNodeId(u32);

#[derive(Debug, Clone)]
pub struct PtNode {
    symbol: Symbol,
    range: TextRange,
    parent: Option<PtNodeId>,
    first_child: Option<PtNodeId>,
    next_sibling: Option<PtNodeId>,
}

/// The parse tree for a single source file.
#[derive(Debug, Clone)]
pub struct ParseTree {
    root: PtNodeId,
    nodes: Vec<PtNode>,
}

impl ParseTree {
    /// The root node of this tree.
    pub fn root(&self) -> PtNodeId {
        self.root
    }
}

impl ops::Index<PtNodeId> for ParseTree {
    type Output = PtNode;

    fn index(&self, id: PtNodeId) -> &PtNode {
        &self.nodes[id.0 as usize]
    }
}

impl PtNode {
    /// The symbol of the token at this node.
    pub fn symbol(&self) -> Symbol {
        self.symbol
    }

    /// The text range covered by the token at this node.
    pub fn range(&self) -> TextRange {
        self.range
    }

    /// The parent node of this node.
    pub fn parent(&self) -> Option<PtNodeId> {
        self.parent
    }

    /// The first child of this node.
    pub fn first_child(&self) -> Option<PtNodeId> {
        self.first_child
    }

    /// The next sibling of this node
    pub fn next_sibling(&self) -> Option<PtNodeId> {
        self.next_sibling
    }
}

impl ops::Index<PtNodeId> for Vec<PtNode> {
    type Output = PtNode;

    fn index(&self, PtNodeId(idx): PtNodeId) -> &PtNode {
        &self[idx as usize]
    }
}

impl ops::IndexMut<PtNodeId> for Vec<PtNode> {
    fn index_mut(&mut self, PtNodeId(idx): PtNodeId) -> &mut PtNode {
        &mut self[idx as usize]
    }
}

fn fill<T>(slot: &mut Option<T>, value: T) {
    assert!(slot.is_none());
    *slot = Some(value);
}
