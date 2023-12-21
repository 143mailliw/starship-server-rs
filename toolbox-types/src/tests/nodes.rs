#[macro_use]
mod macros;
mod util;

mod shape {
    basic_node_test!(ShapeNode, children);
}

mod text {
    use crate::tree::nodes::TextNode;
    basic_node_test!(TextNode);
}
