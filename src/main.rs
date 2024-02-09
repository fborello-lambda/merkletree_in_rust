mod binary_tree;
mod merkle_tree;
mod merkle_tree_vec;

use crate::merkle_tree::MerkleTreeDs;
fn main() {
    let mtree = MerkleTreeDs::new(vec!["D".to_string(), "F".to_string(), "E".to_string()]);
    println!("{mtree}");
}
