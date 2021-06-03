pub struct Xorll {
    pub next: Box<Xorll>,
    pub prev: Box<Xorll>,
    pub value: u32
}

pub struct Node<T> {
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
    pub value: T
}

impl Node<u32> {
    pub fn print_node_value(&self) {
        println!("{}", self.value)
    }
    pub fn has_left(&self) -> bool {
        match self.left {
            None => false,
            Some(_) => true
        }
    }
    pub fn has_right(&self) -> bool {
        match self.right {
            None => false,
            Some(_) => true
        }
    }
    pub fn serialize_tree_recursive(&self, result: Option<String>) -> String {
        let mut ret = match result {
            Some(val) => val,
            None => String::from("")
        };
        ret.push(':');
        ret.push_str(&self.value.to_string());
        if self.has_left() {
            ret = Node::serialize_tree_recursive(self.left.as_ref().unwrap(), Some(ret));
        }
        else {
            ret.push_str(":-1")
        }
        if self.has_right() {
            ret = Node::serialize_tree_recursive(self.right.as_ref().unwrap(), Some(ret));
        }
        else {
            ret.push_str(":-1")
        }

        return ret;
    }


    
//     pub fn serialize_tree(&self) -> String {
//         let mut result = String::from(self.value.to_string());
//         let mut node = self;
//         let mut callstack = vec![];
//         loop {
//             loop {
//                 result.push_str(&node.value.to_string());
//                 callstack.push(node);
//                 if node.has_left() {
//                     node = node.left.as_ref().unwrap();
//                 }
//                 else {
//                     break;
//                 }
//             }
//             if node.has_right() {
//                 node = node.right.as_ref().unwrap();
//             }
//             else {

//             }
//             // else {
//             //     if callstack.len() > 0 {
//             //         node = callstack.pop().as_ref().unwrap();
//             //     }
//             //     else {
//             //         break;
//             //     }
//             // }
//         }
//         result
//     }
}

// pub fn deserialize(serialized_tree: String) -> Node<u32> {
//     Node{
//         left: Some(Box::new(Node {
//             left: None,
//             right: None,
//             value: 1
//         })),
//         right: None,
//         value: 26
//     }
// }