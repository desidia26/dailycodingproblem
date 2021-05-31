// Good morning! Here's your coding interview problem for today.

// This problem was asked by Google.

// Given the root to a binary tree, implement serialize(root), which serializes
// the tree into a string, and deserialize(s), which deserializes the string
// back into the tree.
use local_lib::Node;

fn main() {
    let root_node = Node{
        left: Some(Box::new(Node {
            left: None,
            right: Some(Box::new(Node {
                left: None,
                right: None,
                value: 7
            })),
            value: 1
        })),
        right: Some(Box::new(Node {
            left: None,
            right: None,
            value: 3
        })),
        value: 26
    };
    println!("{}",root_node.serialize_tree_recursive(None));
}