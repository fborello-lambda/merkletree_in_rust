mod binary_tree;
mod merkle_tree;
use crate::merkle_tree::MerkleTree;
fn main() {
    let mtree = MerkleTree::new(vec!["D".to_string()]);

    println!("{mtree}");
}
