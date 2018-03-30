#[macro_use]
extern crate parse_tree;


#[test]
fn smoke() {
    symbols! {
        register
        WHITESPACE 0
        NUMBER 1
        STAR 2
        MUL_EXPR 3
    }

    register();

    let text = "46 * 2";

    let tree = {
        let mut builder = parse_tree::Builder::new();
        builder.start_internal(MUL_EXPR);
        builder.leaf(NUMBER, 2.into());
        builder.leaf(WHITESPACE, 1.into());
        builder.leaf(STAR, 1.into());
        builder.leaf(WHITESPACE, 1.into());
        builder.leaf(NUMBER, 1.into());
        builder.finish_internal();
        builder.finish()
    };

    let debug = parse_tree::debug_dump(tree.root(), text);
    assert_eq!(debug.trim(), r#"
MUL_EXPR@[0; 6)
  NUMBER@[0; 2) "46"
  WHITESPACE@[2; 3)
  STAR@[3; 4) "*"
  WHITESPACE@[4; 5)
  NUMBER@[5; 6) "2"
"#.trim());
}
