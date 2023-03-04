use crate::{utils::*, Node};


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

    pub fn build_from_arr(&mut self, transactions: Vec<Vec<String>>) {
        for transaction in transactions {
            self.add_node(transaction);
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
        let new_node = Node {
            hash: hash,
            left: None,
            right: None,
        };

        let mut parent_node = &mut self.root;
        let mut is_left_node = true;
        
        loop {
            if is_left_node {
                if parent_node.left.is_none() {
                    parent_node.left = Some(Box::new(new_node));
                    break;
                } else {
                    let left_node = parent_node.left.as_mut().unwrap();
                    let new_parent_hash = calculate_hash(&left_node.hash, &new_node.hash);
                    parent_node.hash = new_parent_hash;
                    parent_node = left_node.as_mut();
                    is_left_node = true;
                }
            } else {
                if parent_node.right.is_none() {
                    parent_node.right = Some(Box::new(new_node));
                    break;
                } else {
                    let right_node = parent_node.right.as_mut().unwrap();
                    let new_parent_hash = calculate_hash(&new_node.hash, &right_node.hash);
                    parent_node.hash = new_parent_hash;
                    parent_node = right_node.as_mut();
                    is_left_node = false;
                }
            }
        }
    }

    pub fn validate_tree(&self) -> bool {
        self.root.is_valid()
    }
}

