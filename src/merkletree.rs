use sha2::{Digest, Sha256};
use crate::utils::*;

struct Node {
    hash: [u8; 32],
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

pub struct MerkleTree {
    root: Node,
    number_of_nodes: usize,
}

impl MerkleTree {
    pub fn new() -> MerkleTree {
        MerkleTree {
            root: Node {
                hash: [0; 32],
                left: None,
                right: None,
            },
            number_of_nodes: 1,
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


    pub fn add_first_node(&mut self, transaction: Vec<String>) {
        let hash = self.hash_data(transaction);

        self.root = Node {
            hash: hash,
            left: None,
            right: None,
        }
    }

    fn hash_data(&self, transaction: Vec<String>) -> [u8; 32] {
        let mut hasher = Sha256::new();

        for item in transaction {
            hasher.update(item);
        }

        let result = hasher.finalize();

        result.into()
    }
}
