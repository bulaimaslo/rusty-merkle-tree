use crate::utils::*;

struct Node {
    hash: [u8; 32],
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

pub struct MerkleTree {
    root: Node,
}

impl MerkleTree {
    pub fn new() -> MerkleTree {
        MerkleTree {
            root: Node {
                hash: [0; 32],
                left: None,
                right: None,
            },
        }
    }

    pub fn print(&self) {
        self.print_node(&self.root);
    }

    fn print_node(&self, node: &Node) {
        print_hash(&node.hash);
        if let Some(left) = &node.left {
            self.print_node(left);
        }
        if let Some(right) = &node.right {
            self.print_node(right);
        }
    }

    pub fn add_node(&mut self, transaction: Vec<String>) {
        let hash = hash_data(transaction);

        let node = Node {
            hash: hash,
            left: None,
            right: None,
        };

        self.add_node_to_tree(node);
    }

    fn add_node_to_tree(&mut self, node: Node) {
        let mut current_node = &mut self.root;

        while current_node.left.is_some() && current_node.right.is_some() {
            current_node = current_node.left.as_mut().unwrap();
        }

        if current_node.left.is_none() {
            current_node.left = Some(Box::new(node));
        } else {
            current_node.right = Some(Box::new(node));
        }

        todo!() // update hash
    }
}

