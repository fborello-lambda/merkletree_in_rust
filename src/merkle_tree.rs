extern crate crypto;
use std::fmt;

use self::crypto::digest::Digest;
use crypto::sha3::Sha3;

#[derive(Clone, Debug)]
pub struct Node {
    pub hash: String,
    pub left_node: Leaf,
    pub right_node: Leaf,
}

type Leaf = Option<Box<Node>>;

#[derive(Debug)]
pub struct MerkleTree {
    pub root: Leaf,
}

// How can i "hide" this enum? -> How to make it more private?
enum Dir {
    Left,
    Right,
    Root,
}

impl fmt::Display for MerkleTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(root) = &self.root {
            writeln!(f, "Merkle Tree:")?;
            MerkleTree::print_node(f, root, 0, Dir::Root)?;
        } else {
            write!(f, "Empty Merkle Tree")?;
        }
        Ok(())
    }
}

impl MerkleTree {
    pub fn new(initial_leaves: Vec<String>) -> MerkleTree {
        //todo!("Check Length");
        let mut upper_children: Vec<Leaf> = Vec::new();

        let hashed_values = initial_leaves
            .clone()
            .iter_mut()
            .map(|x| {
                let mut sha3 = Sha3::keccak256();
                sha3.input_str(x);
                sha3.result_str()
            })
            .collect::<Vec<String>>();

        for chunk in hashed_values.chunks(2) {
            // "Balance" the tree -> Duplicate if there is no right leaf
            let right = if chunk.len() > 1 {
                chunk[1].clone()
            } else {
                chunk[0].clone()
            };
            let child_left = Box::new(Node {
                hash: chunk[0].clone(),
                left_node: None,
                right_node: None,
            });

            let child_right = Box::new(Node {
                hash: right.clone(),
                left_node: None,
                right_node: None,
            });

            let combined_hash = {
                let mut sha3 = Sha3::keccak256();
                sha3.input_str(&format!("{}{}", chunk[0].clone(), right));
                sha3.result_str()
            };

            let upper = Box::new(Node {
                hash: combined_hash,
                left_node: Some(child_left),
                right_node: Some(child_right),
            });
            upper_children.push(Some(upper));
        }

        if upper_children.len() == 1 {
            return MerkleTree {
                root: upper_children.pop().unwrap(),
            };
        }

        while upper_children.len() > 1 {
            let mut new_upper_children: Vec<Leaf> = Vec::new();

            for chunk in upper_children.chunks(2) {
                let left = chunk[0].clone().unwrap();
                // "Balance" the tree -> Duplicate if there is no right leaf
                let right = if chunk.len() > 1 {
                    chunk[1].clone().unwrap()
                } else {
                    chunk[0].clone().unwrap()
                };

                let combined_hash = {
                    let mut sha3 = Sha3::keccak256();
                    sha3.input_str(&format!("{}{}", left.hash, right.hash));
                    sha3.result_str()
                };

                let upper = Box::new(Node {
                    hash: combined_hash,
                    left_node: Some(left.clone()),
                    right_node: Some(right.clone()),
                });
                new_upper_children.push(Some(upper));
            }
            upper_children = new_upper_children;
        }

        MerkleTree {
            root: upper_children.pop().unwrap(),
        }
    }

    fn print_node(
        f: &mut fmt::Formatter<'_>,
        node: &Node,
        level: usize,
        left_right: Dir,
    ) -> fmt::Result {
        let indent = "    ".repeat(level);

        let dir = match left_right {
            Dir::Left => "left",
            Dir::Right => "right",
            Dir::Root => "root",
        };
        writeln!(
            f,
            "{} {}- {}",
            indent,
            dir,
            node.hash.chars().take(5).collect::<String>()
        )?;
        if let Some(left_node) = &node.left_node {
            MerkleTree::print_node(f, left_node.as_ref(), level + 1, Dir::Left)?;
        } else {
            writeln!(f, "{}left- None", "    ".repeat(level + 1))?;
        }
        if let Some(right_node) = &node.right_node {
            MerkleTree::print_node(f, right_node.as_ref(), level + 1, Dir::Right)?;
        } else {
            writeln!(f, "{}right- None", "    ".repeat(level + 1))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::MerkleTree;
    use super::*;
    #[test]
    #[allow(dead_code)]
    #[allow(unused_variables)]
    fn test_new_no_duplication() {
        let d = "D".to_string();
        let e = "E".to_string();

        let d_hash = {
            let mut sha3 = Sha3::keccak256();
            sha3.input_str(&d.clone());
            sha3.result_str()
        };
        let e_hash = {
            let mut sha3 = Sha3::keccak256();
            sha3.input_str(&e.clone());
            sha3.result_str()
        };

        let mut sha3 = Sha3::keccak256();
        sha3.input_str(&format!("{}{}", d_hash, e_hash));

        let mtree = MerkleTree::new(vec![d, e]);

        assert_eq!(mtree.root.unwrap().hash, sha3.result_str());
    }
    #[test]
    fn test_new_with_duplication() {
        let d = "D".to_string();

        let d_hash = {
            let mut sha3 = Sha3::keccak256();
            sha3.input_str(&d.clone());
            sha3.result_str()
        };

        let mut sha3 = Sha3::keccak256();
        sha3.input_str(&format!("{}{}", d_hash, d_hash));

        let mtree = MerkleTree::new(vec![d]);

        assert_eq!(mtree.root.unwrap().hash, sha3.result_str());
    }
}
