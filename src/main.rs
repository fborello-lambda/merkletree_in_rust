mod binary_tree;
mod merkle_tree;
mod merkle_tree_vec;

use crate::merkle_tree::MerkleTreeDs;
use crate::merkle_tree_vec::{keccak256, MerkleTreeVec};
fn main() {
    let mtree = MerkleTreeDs::new(vec_to_string!["D"]);
    let mtree_vec = MerkleTreeVec::new(&slice_to_string!["D"], &keccak256);

    let d_proof = mtree_vec.get_proof("D");
    let verify = mtree_vec.verify(d_proof, "D");

    println!("{mtree}");
    println!("{}", mtree_vec.root);
}
