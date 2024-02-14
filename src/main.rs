mod binary_tree;
mod merkle_tree;
mod merkle_tree_vec;

use crate::merkle_tree::MerkleTreeDs;
use crate::merkle_tree_vec::{keccak256, MerkleTreeVec};
fn main() -> Result<(), String> {
    let mtree = MerkleTreeDs::new(vec_to_string!["D"]);
    let mtree_vec = MerkleTreeVec::new(&slice_to_string!["D"], &keccak256);

    let d_proof = mtree_vec.get_proof("D")?;

    for (i, x) in d_proof.iter().enumerate() {
        println!("proof of D[{i}]: {}", x);
    }
    let verify = mtree_vec.verify(d_proof, "D");

    println!("Is the proof of D right? {}", verify.unwrap());
    println!("{mtree}");
    println!("{}", mtree_vec.root);
    Ok(())
}
