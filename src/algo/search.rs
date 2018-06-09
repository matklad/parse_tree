use {ParseTree, PtNodeId, TextUnit, TextRange};

pub fn is_leaf(tree: &ParseTree, node_id: PtNodeId) -> bool {
    let node = &tree[node_id];
    node.first_child().is_none() && node.range().is_empty()
}

pub fn children(tree: &ParseTree, node_id: PtNodeId) -> Children {
    impl<'a> Iterator for Children<'a> {
        type Item = PtNodeId;

        fn next(&mut self) -> Option<PtNodeId> {
            let curr = self.curr.take()?;
            self.curr = self.tree[curr].next_sibling();
            Some(curr)
        }
    }

    Children { tree, curr: tree[node_id].first_child() }
}

pub struct Children<'a> {
    tree: &'a ParseTree,
    curr: Option<PtNodeId>,
}


pub fn find_leaf_at_offset(tree: &ParseTree, offset: TextUnit) -> LeafAtOffset {
    return go(tree, tree.root(), offset);

    fn go(tree: &ParseTree, node_id: PtNodeId, offset: TextUnit) -> LeafAtOffset {
        let node = &tree[node_id];
        let range = node.range();
        assert!(contains_offset_nonstrict(node.range(), offset),
                "Bad offset: range {:?} offset {:?}", range, offset);
        if range.is_empty() {
            return LeafAtOffset::None;
        }

        if is_leaf(tree, node_id) {
            return LeafAtOffset::Single(node_id);
        }
        let mut children = children(tree, node_id)
            .filter(|&child_id| {
                let range = tree[child_id].range();
                !range.is_empty() && contains_offset_nonstrict(range, offset)
            });

        let left = children.next().unwrap();
        let right = children.next();
        assert!(children.next().is_none());
        return if let Some(right) = right {
            match (go(tree, left, offset), go(tree, right, offset)) {
                (LeafAtOffset::Single(left), LeafAtOffset::Single(right)) =>
                    LeafAtOffset::Between(left, right),
                _ => unreachable!()
            }
        } else {
            go(tree, left, offset)
        };
    }
}

#[derive(Clone, Copy, Debug)]
pub enum LeafAtOffset {
    None,
    Single(PtNodeId),
    Between(PtNodeId, PtNodeId),
}

impl LeafAtOffset {
    pub fn right_biased(self) -> Option<PtNodeId> {
        match self {
            LeafAtOffset::None => None,
            LeafAtOffset::Single(node) => Some(node),
            LeafAtOffset::Between(_, right) => Some(right)
        }
    }

    pub fn left_biased(self) -> Option<PtNodeId> {
        match self {
            LeafAtOffset::None => None,
            LeafAtOffset::Single(node) => Some(node),
            LeafAtOffset::Between(left, _) => Some(left)
        }
    }
}

impl Iterator for LeafAtOffset {
    type Item = PtNodeId;

    fn next(&mut self) -> Option<PtNodeId> {
        match *self {
            LeafAtOffset::None => None,
            LeafAtOffset::Single(node) => {
                *self = LeafAtOffset::None;
                Some(node)
            }
            LeafAtOffset::Between(left, right) => {
                *self = LeafAtOffset::Single(right);
                Some(left)
            }
        }
    }
}

pub fn find_covering_node(tree: &ParseTree, range: TextRange) -> PtNodeId {
    assert!(is_subrange_of(tree[tree.root()].range(), range));
    let (left, right) = match (
        find_leaf_at_offset(tree, range.start()).right_biased(),
        find_leaf_at_offset(tree, range.end()).left_biased()
    ) {
        (Some(l), Some(r)) => (l, r),
        _ => return tree.root(), // empty tree
    };

    common_ancestor(tree, left, right)
}


pub fn ancestors(tree: &ParseTree, node_id: PtNodeId) -> Ancestors {
    Ancestors { tree, curr: Some(node_id) }
}

pub struct Ancestors<'a> {
    tree: &'a ParseTree,
    curr: Option<PtNodeId>,
}

impl<'f> Iterator for Ancestors<'f> {
    type Item = PtNodeId;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.curr.take()?;
        self.curr = self.tree[curr].parent;
        Some(curr)
    }
}

fn common_ancestor(tree: &ParseTree, n1: PtNodeId, n2: PtNodeId) -> PtNodeId {
    for p in ancestors(tree, n1) {
        if ancestors(tree, n2).any(|a| a == p) {
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
