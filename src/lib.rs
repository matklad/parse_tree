//! `parse_tree` is a library to represent so-called parse tree.
//! A parse tree is a non-abstract AST: it's a generic syntax tree
//! which remembers all whitespace, comments and other trivia.
extern crate serde;

use std::{cmp, fmt, ops, ptr};

mod text;
mod top_down_builder;
mod bottom_up_builder;
pub mod search;

pub use text::{TextRange, TextUnit};
pub use top_down_builder::TopDownBuilder;
pub use bottom_up_builder::BottomUpBuilder;

/// A type of a syntactic construct, including both leaf tokens
/// and composite nodes, like "a comma" or "a function".
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Symbol(pub u32);

/// A token of source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token {
    /// The kind of token.
    pub symbol: Symbol,
    /// The length of the token.
    pub len: TextUnit,
}

/// The parse tree for a single source file.
#[derive(Debug)]
pub struct ParseTree {
    root: NodeIdx,
    nodes: Vec<NodeData>,
}

impl ParseTree {
    /// The root node of this tree.
    pub fn root<'t>(&'t self) -> Node<'t> {
        assert!(!self.nodes.is_empty());
        Node {
            file: self,
            idx: self.root,
        }
    }
}

/// A reference to a node in a parse tree.
#[derive(Clone, Copy)]
pub struct Node<'t> {
    file: &'t ParseTree,
    idx: NodeIdx,
}

impl<'f> cmp::PartialEq<Node<'f>> for Node<'f> {
    fn eq(&self, other: &Node<'f>) -> bool {
        self.idx == other.idx && ptr::eq(self.file, other.file)
    }
}

impl<'f> cmp::Eq for Node<'f> {}

impl<'t> Node<'t> {
    /// The symbol of the token at this node.
    pub fn symbol(&self) -> Symbol {
        self.data().symbol
    }

    /// The text range covered by the token at this node.
    pub fn range(&self) -> TextRange {
        self.data().range
    }

    /// The parent node of this node.
    pub fn parent(&self) -> Option<Node<'t>> {
        self.as_node(self.data().parent)
    }

    /// The children nodes of this node.
    pub fn children(&self) -> Children<'t> {
        Children {
            next: self.as_node(self.data().first_child),
        }
    }

    fn data(&self) -> &'t NodeData {
        &self.file.nodes[self.idx]
    }

    fn as_node(&self, idx: Option<NodeIdx>) -> Option<Node<'t>> {
        idx.map(|idx| Node {
            file: self.file,
            idx,
        })
    }
}

impl<'t> fmt::Debug for Node<'t> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?}@{:?}", self.symbol(), self.range())
    }
}

/// Debug representation of a subtree at `node`.
pub fn debug_dump<'t>(
    node: Node,
    get_text: &'t Fn(TextRange) -> &'t str,
    get_symbol_name: &'t Fn(Symbol) -> &'t str,
) -> String {
    struct Dumper<'a> {
        buff: String,
        get_text: &'a Fn(TextRange) -> &'a str,
        get_symbol_name: &'a Fn(Symbol) -> &'a str,
    }

    impl<'a> Dumper<'a> {
        fn go(&mut self, node: Node, level: usize) {
            self.buff.push_str(&String::from("  ").repeat(level));
            let s = (self.get_symbol_name)(node.symbol());
            self.buff.push_str(&format!("{}@{:?}", s, node.range()));

            if node.children().next().is_none() {
                let text = (self.get_text)(node.range());
                if !text.chars().all(char::is_whitespace) {
                    self.buff.push_str(&format!(" {:?}", text));
                }
            }
            self.buff.push('\n');
            for child in node.children() {
                self.go(child, level + 1)
            }
        }
    }

    let mut dumper = Dumper { buff: String::new(), get_text, get_symbol_name };
    dumper.go(node, 0);
    return dumper.buff;
}

#[derive(Debug, Clone)]
pub struct Children<'f> {
    next: Option<Node<'f>>,
}

impl<'f> Iterator for Children<'f> {
    type Item = Node<'f>;

    fn next(&mut self) -> Option<Node<'f>> {
        let next = self.next;
        self.next = next.and_then(|node| node.as_node(node.data().next_sibling));
        next
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct NodeIdx(u32);

#[derive(Debug)]
struct NodeData {
    symbol: Symbol,
    range: TextRange,
    parent: Option<NodeIdx>,
    first_child: Option<NodeIdx>,
    next_sibling: Option<NodeIdx>,
}

impl ops::Index<NodeIdx> for Vec<NodeData> {
    type Output = NodeData;

    fn index(&self, NodeIdx(idx): NodeIdx) -> &NodeData {
        &self[idx as usize]
    }
}

impl ops::IndexMut<NodeIdx> for Vec<NodeData> {
    fn index_mut(&mut self, NodeIdx(idx): NodeIdx) -> &mut NodeData {
        &mut self[idx as usize]
    }
}

fn fill<T>(slot: &mut Option<T>, value: T) {
    assert!(slot.is_none());
    *slot = Some(value);
}
