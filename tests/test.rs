extern crate parse_tree;
use parse_tree::{Symbol};


#[test]
fn smoke() {
    let text = "46 * 2";
    let whitespace = Symbol::new(0, "whitespace");
    let number = Symbol::new(1, "number");
    let start = Symbol::new(2, "star");
    let mul_expr = Symbol::new(3, "mul_expr");

    let tree = {
        let mut builder = parse_tree::Builder::new();
        builder.start_internal(mul_expr);
        builder.leaf(number, 2.into());
        builder.leaf(whitespace, 1.into());
        builder.leaf(start, 1.into());
        builder.leaf(whitespace, 1.into());
        builder.leaf(number, 1.into());
        builder.finish_internal();
        builder.finish()
    };

    let debug = parse_tree::debug_dump(tree.root(), text);
    assert_eq!(debug.trim(), r#"
mul_expr@[0; 6)
  number@[0; 2) "46"
  whitespace@[2; 3)
  star@[3; 4) "*"
  whitespace@[4; 5)
  number@[5; 6) "2"
"#.trim());
}
