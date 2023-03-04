use crate::utils::*;

pub struct Node {
    pub hash: [u8; 32],
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    pub fn new(hash: [u8; 32]) -> Node {
        Node {
            hash: hash,
            left: None,
            right: None,
        }
    }

    pub fn is_valid(&self) -> bool {
        if self.left.is_some() && self.right.is_some() {
            let left_node = self.left.as_ref().unwrap();
            let right_node = self.right.as_ref().unwrap();
            let new_parent_hash = calculate_hash(&left_node.hash, &right_node.hash);
            if new_parent_hash != self.hash {
                return false;
            }

            if !left_node.is_valid() {
                return false;
            }

            if !right_node.is_valid() {
                return false;
            }
        }

        true
    }
}
