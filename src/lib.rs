//! `parse_tree` is a library to represent so-called parse tree.
//! A parse tree is a non-abstract AST: it's a generic syntax tree
//! which remembers all whitespace, comments and other trivia.
#[macro_use]
extern crate lazy_static;

use std::{cmp, fmt, ops, ptr};
use std::sync::Mutex;
use std::collections::hash_map::{HashMap, Entry};

mod text;
mod builder;

pub use text::{TextRange, TextUnit};
pub use builder::Builder;

/// A type of a syntactic construct, including both leaf tokens
/// and composite nodes, like "a comma" or "a function".
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Symbol(
    #[doc(hidden)]
    pub u32
);

struct SymbolInfo {
    name: &'static str
}

lazy_static! {
    static ref SYMBOLS: Mutex<HashMap<Symbol, SymbolInfo>>
        = Mutex::new(HashMap::new());
}

#[doc(hidden)]
pub fn register_symbol(symbol: Symbol, name: &'static str) {
    let mut symbols = SYMBOLS.lock().unwrap();
    match symbols.entry(symbol) {
        Entry::Occupied(_) => {
            panic!("Duplicate symbol {} {}", symbol.0, name);
        }
        Entry::Vacant(entry) => {
            entry.insert(SymbolInfo { name });
        }
    }
}

#[macro_export]
macro_rules! symbols {
    ( $register:ident $($name:ident $id:expr)*) => {
        $(
            const $name: $crate::Symbol = $crate::Symbol($id);
        )*

        pub fn $register() {
            static INIT: ::std::sync::Once = ::std::sync::ONCE_INIT;
            INIT.call_once(|| {
                $(
                    $crate::register_symbol($name, stringify!($name));
                )*
            })
        }
    };
}

impl Symbol {
    pub fn name(&self) -> &'static str {
        SYMBOLS.lock().unwrap()[self].name
    }
}

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
    nodes: Vec<NodeData>,
}

impl ParseTree {
    /// The root node of this tree.
    pub fn root<'t>(&'t self) -> Node<'t> {
        assert!(!self.nodes.is_empty());
        Node {
            file: self,
            idx: NodeIdx(0),
        }
    }
}

/// A reference to a node in a parse tree.
#[derive(Clone, Copy)]
pub struct Node<'t> {
    file: &'t ParseTree,
    idx: NodeIdx,
}

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
        write!(fmt, "{}@{:?}", self.symbol().name(), self.range())
    }
}

/// Debug representation of a subtree at `node`.
pub fn debug_dump(node: Node, text: &str) -> String {
        let mut result = String::new();
        go(node, &mut result, 0, text);
        return result;

        fn go(node: Node, buff: &mut String, level: usize, text: &str) {
            buff.push_str(&String::from("  ").repeat(level));
            buff.push_str(&format!("{:?}", node));

            if node.children().next().is_none() {
                let node_text = &text[node.range()];
                if !node_text.chars().all(char::is_whitespace) {
                    buff.push_str(&format!(" {:?}", node_text));
                }
            }
            buff.push('\n');
            for child in node.children() {
                go(child, buff, level + 1, text)
            }
        }
}

impl<'f> cmp::PartialEq<Node<'f>> for Node<'f> {
    fn eq(&self, other: &Node<'f>) -> bool {
        self.idx == other.idx && ptr::eq(self.file, other.file)
    }
}

impl<'f> cmp::Eq for Node<'f> {}

#[derive(Debug)]
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
