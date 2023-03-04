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

    pub fn get_proof(&self, transaction: Vec<String>) -> Vec<[u8; 32]> {
        let mut proof: Vec<[u8; 32]> = Vec::new();
        let hash = hash_data(transaction);
        let mut node = &self.root;

        loop {
            if node.left.is_some() && node.right.is_some() {
                let left_node = node.left.as_ref().unwrap();
                let right_node = node.right.as_ref().unwrap();

                if left_node.hash == hash {
                    proof.push(right_node.hash);
                    break;
                } else if right_node.hash == hash {
                    proof.push(left_node.hash);
                    break;
                } else {
                    let new_parent_hash = calculate_hash(&left_node.hash, &right_node.hash);
                    if new_parent_hash == hash {
                        proof.push(left_node.hash);
                        proof.push(right_node.hash);
                        break;
                    } else {
                        if hash < new_parent_hash {
                            node = left_node.as_ref();
                        } else {
                            node = right_node.as_ref();
                        }
                    }
                }
            } else {
                break;
            }
        }

        proof
    }

    pub fn contains_hash(&self, hash: [u8; 32]) -> bool {
        let mut node = &self.root;

        loop {
            if node.left.is_some() && node.right.is_some() {
                let left_node = node.left.as_ref().unwrap();
                let right_node = node.right.as_ref().unwrap();

                if left_node.hash == hash || right_node.hash == hash {
                    return true;
                } else {
                    let new_parent_hash = calculate_hash(&left_node.hash, &right_node.hash);
                    if new_parent_hash == hash {
                        return true;
                    } else {
                        if hash < new_parent_hash {
                            node = left_node.as_ref();
                        } else {
                            node = right_node.as_ref();
                        }
                    }
                }
            } else {
                break;
            }
        }

        false
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
