mod binary_tree;
mod merkle_tree;
mod merkle_tree_vec;

use crate::merkle_tree::MerkleTreeDs;
use crate::merkle_tree_vec::{keccak256, MerkleTreeVec};
fn main() {
    let mtree = MerkleTreeDs::new(vec!["D".to_string()]);
    let mtree_vec = MerkleTreeVec::new(vec!["D".to_string()], &keccak256);
    println!("{mtree}");
    println!("{}", mtree_vec.root);
}
