extern crate parse_tree;

use parse_tree::{
    Symbol, ParseTree, PtNodeId,
    algo::{AsParseTree, GetSymbol, GetText},
};

const WHITESPACE: Symbol = Symbol(0);
const NUMBER: Symbol = Symbol(1);
const STAR: Symbol = Symbol(2);
const MUL_EXPR: Symbol = Symbol(3);

struct TestTree {
    text: String,
    tree: ParseTree,
}

impl AsParseTree for TestTree {
    fn as_parse_tree(&self) -> &ParseTree {
        &self.tree
    }
}

impl GetText for TestTree {
    fn get_text(&self, id: PtNodeId) -> String {
        let range = self.tree[id].range();
        self.text[range].to_owned()
    }
}

impl GetSymbol for TestTree {
    fn get_symbol(&self, id: PtNodeId) -> String {
        let symbol = self.tree[id].symbol();
        let name = match symbol {
            WHITESPACE => "WHITESPACE",
            NUMBER => "NUMBER",
            STAR => "STAR",
            MUL_EXPR => "MUL_EXPR",
            _ => panic!("unknown symbol: {:?}", symbol)
        };
        name.to_owned()
    }
}

#[test]
fn top_down() {
    let text = "46 * 2";

    let tree = {
        let mut builder = parse_tree::TopDownBuilder::new();
        builder.start_internal(MUL_EXPR);
        builder.leaf(NUMBER, 2.into());
        builder.leaf(WHITESPACE, 1.into());
        builder.leaf(STAR, 1.into());
        builder.leaf(WHITESPACE, 1.into());
        builder.leaf(NUMBER, 1.into());
        builder.finish_internal();
        builder.finish()
    };
    let tree = TestTree {
        text: text.to_string(),
        tree
    };

    let debug = parse_tree::algo::debug_dump(&tree);
    assert_eq!(
        debug.trim(),
        r#"
MUL_EXPR@[0; 6)
  NUMBER@[0; 2) "46"
  WHITESPACE@[2; 3)
  STAR@[3; 4) "*"
  WHITESPACE@[4; 5)
  NUMBER@[5; 6) "2"
"#.trim()
    );
}

#[test]
fn bottom_up() {
    let text = "46 * 2";

    let tree = {
        let mut builder = parse_tree::BottomUpBuilder::new();
        builder.shift(NUMBER, 2.into());
        builder.shift(WHITESPACE, 1.into());
        builder.shift(STAR, 1.into());
        builder.shift(WHITESPACE, 1.into());
        builder.shift(NUMBER, 1.into());
        builder.reduce(MUL_EXPR, 5);
        builder.finish()
    };

    let tree = TestTree {
        text: text.to_string(),
        tree
    };

    let debug = parse_tree::algo::debug_dump(&tree);
    assert_eq!(
        debug.trim(),
        r#"
MUL_EXPR@[0; 6)
  NUMBER@[0; 2) "46"
  WHITESPACE@[2; 3)
  STAR@[3; 4) "*"
  WHITESPACE@[4; 5)
  NUMBER@[5; 6) "2"
"#.trim()
    );
}
