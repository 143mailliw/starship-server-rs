macro_rules! basic_node_test {
    ($type:ty $(,)?) => {
        use crate::tree::nodes::ShapeNode;
        use crate::tests::nodes::util::{add_child, remove_child, add_observer_util,
            remove_observer_util};

        #[test]
        fn as_child() {
            add_child::<ShapeNode, $type>();
        }

        #[test]
        fn remove_as_child() {
            remove_child::<ShapeNode, $type>();
        }

        #[test]
        fn add_observer() {
            add_observer_util::<$type>();
        }

        #[test]
        fn remove_observer() {
            remove_observer_util::<$type>();
        }
    };
    ($type:ty, $(,)? children $($tail:tt)*) => {
        basic_node_test!($type, $($tail)*);

        #[test]
        fn as_parent() {
            add_child::<$type, ShapeNode>();
        }
    };
}
