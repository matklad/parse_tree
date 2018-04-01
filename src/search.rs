use {Node, TextUnit, TextRange};

pub fn is_leaf(node: Node) -> bool {
    node.children().next().is_none() && !node.range().is_empty()
}

pub fn find_leaf_at_offset(node: Node, offset: TextUnit) -> LeafAtOffset {
    let range = node.range();
    assert!(contains_offset_nonstrict(node.range(), offset),
            "Bad offset: range {:?} offset {:?}", range, offset);
    if range.is_empty() {
        return LeafAtOffset::None;
    }

    if is_leaf(node) {
        return LeafAtOffset::Single(node);
    }

    let mut children = node.children()
        .filter(|child| !child.range().is_empty())
        .filter(|child| contains_offset_nonstrict(child.range(), offset));

    let left = children.next().unwrap();
    let right = children.next();
    assert!(children.next().is_none());
    return if let Some(right) = right {
        match (find_leaf_at_offset(left, offset), find_leaf_at_offset(right, offset)) {
            (LeafAtOffset::Single(left), LeafAtOffset::Single(right)) =>
                LeafAtOffset::Between(left, right),
            _ => unreachable!()
        }
    } else {
        find_leaf_at_offset(left, offset)
    };
}

#[derive(Clone, Copy, Debug)]
pub enum LeafAtOffset<'f> {
    None,
    Single(Node<'f>),
    Between(Node<'f>, Node<'f>)
}

impl<'f> LeafAtOffset<'f> {
    pub fn right_biased(self) -> Option<Node<'f>> {
        match self {
            LeafAtOffset::None => None,
            LeafAtOffset::Single(node) => Some(node),
            LeafAtOffset::Between(_, right) => Some(right)
        }
    }

    pub fn left_biased(self) -> Option<Node<'f>> {
        match self {
            LeafAtOffset::None => None,
            LeafAtOffset::Single(node) => Some(node),
            LeafAtOffset::Between(left, _) => Some(left)
        }
    }
}

impl<'f> Iterator for LeafAtOffset<'f> {
    type Item = Node<'f>;

    fn next(&mut self) -> Option<Node<'f>> {
        match *self {
            LeafAtOffset::None => None,
            LeafAtOffset::Single(node) => { *self = LeafAtOffset::None; Some(node) }
            LeafAtOffset::Between(left, right) => { *self = LeafAtOffset::Single(right); Some(left) }
        }
    }
}

pub fn find_covering_node(root: Node, range: TextRange) -> Node {
    assert!(is_subrange_of(root.range(), range));
    let (left, right) = match (
        find_leaf_at_offset(root, range.start()).right_biased(),
        find_leaf_at_offset(root, range.end()).left_biased()
    ) {
        (Some(l), Some(r)) => (l, r),
        _ => return root
    };

    common_ancestor(left, right)
}


pub fn ancestors(node: Node) -> Ancestors {
    Ancestors(Some(node))
}

pub struct Ancestors<'f>(Option<Node<'f>>);

impl<'f> Iterator for Ancestors<'f> {
    type Item = Node<'f>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.0;
        self.0 = current.and_then(|n| n.parent());
        current
    }
}

fn common_ancestor<'f>(n1: Node<'f>, n2: Node<'f>) -> Node<'f> {
    for p in ancestors(n1) {
        if ancestors(n2).any(|a| a == p) {
            return p;
        }
    }
    panic!("Can't find common ancestor of {:?} and {:?}", n1, n2)
}

pub fn contains_offset_nonstrict(range: TextRange, offset: TextUnit) -> bool {
    range.start() <= offset && offset <= range.end()
}

pub fn is_subrange_of(range: TextRange, sub: TextRange) -> bool {
    range.start() <= sub.start() && sub.end() <= range.end()
}
