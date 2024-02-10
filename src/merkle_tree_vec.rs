extern crate crypto;

use self::crypto::digest::Digest;
use crypto::sha3::Sha3;

pub struct MerkleTreeVec<'a> {
    pub root: String,
    pub initial_leaves: Vec<String>,
    hash_fn: &'a dyn Fn(&str) -> String,
}

pub fn keccak256(s: &str) -> String {
    let mut sha3 = Sha3::keccak256();
    sha3.input_str(s);
    sha3.result_str()
}

// This code assumes that the length of initial_leaves is a power of 2 (i.e., initial_leaves.len() == 2^N).
// This condition must be satisfied for optimal performance.
// Otherwise, additional copy operations may be required at each level of the computation as needed.
impl<'a> MerkleTreeVec<'a> {
    pub fn new(
        initial_leaves: Vec<String>,
        hash_fn: &'a dyn Fn(&str) -> String,
    ) -> MerkleTreeVec<'a> {
        let mut upper_children: Vec<String> = Vec::new();

        let hashed_initial_leaves = initial_leaves
            .clone()
            .into_iter()
            .map(|s| hash_fn(&s))
            .collect::<Vec<String>>();

        for chunk in hashed_initial_leaves.chunks(2) {
            let left = chunk[0].clone();
            // "Balance" the tree -> Duplicate if there is no right leaf
            let right = if chunk.len() > 1 {
                chunk[1].clone()
            } else {
                chunk[0].clone()
            };

            let combined_hash = hash_fn(&format!("{}{}", left, right));

            upper_children.push(combined_hash);
        }

        if upper_children.len() == 1 {
            return MerkleTreeVec {
                root: upper_children.pop().unwrap(),
                initial_leaves,
                hash_fn,
            };
        }

        while upper_children.len() > 1 {
            let mut new_upper_children: Vec<String> = Vec::new();

            for chunk in upper_children.chunks(2) {
                let left = chunk[0].clone();
                // "Balance" the tree -> Duplicate if there is no right leaf
                let right = if chunk.len() > 1 {
                    chunk[1].clone()
                } else {
                    chunk[0].clone()
                };

                let combined_hash = hash_fn(&format!("{}{}", left, right));

                new_upper_children.push(combined_hash);
            }
            upper_children = new_upper_children;
        }

        MerkleTreeVec {
            root: upper_children.pop().unwrap(),
            initial_leaves,
            hash_fn,
        }
    }

    pub fn push_to_initial(&mut self, new_leaves: &mut Vec<String>) {
        let mut new_initial_leaves = self.initial_leaves.clone();
        new_initial_leaves.append(new_leaves);

        *self = MerkleTreeVec::new(new_initial_leaves, self.hash_fn);
    }

    pub fn in_merkletree(self, value_n_hashed_path: Vec<String>) -> bool {
        let mut check_root = keccak256(value_n_hashed_path.first().unwrap());

        for h in &value_n_hashed_path[1..] {
            println!("{h}");
            let combined_hash = keccak256(&format!("{}{}", check_root, h));

            check_root = combined_hash;
        }
        check_root == self.root
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::MerkleTreeVec;
    use super::*;
    #[test]
    fn test_new_no_duplication() {
        let d = "D".to_string();
        let e = "E".to_string();

        let d_hash = keccak256(&d);
        let e_hash = keccak256(&e);

        let cmp = keccak256(&format!("{}{}", d_hash, e_hash));

        let mtree = MerkleTreeVec::new(vec![d, e], &keccak256);

        assert_eq!(mtree.root, cmp);
    }
    #[test]
    fn test_new_with_duplication() {
        let d = "D".to_string();
        let d_hash = keccak256(&d);

        let cmp = keccak256(&format!("{}{}", d_hash, d_hash));

        let mtree = MerkleTreeVec::new(vec![d], &keccak256);

        assert_eq!(mtree.root, cmp);
    }
    #[test]
    fn test_push() {
        let d = "D".to_string();
        let e = "E".to_string();

        let d_hash = keccak256(&d);
        let e_hash = keccak256(&e);

        let cmp = keccak256(&format!("{}{}", d_hash, e_hash));

        let mut mtree = MerkleTreeVec::new(vec![d], &keccak256);
        mtree.push_to_initial(&mut vec![e]);

        assert_eq!(mtree.root, cmp);
    }

    #[test]
    fn test_in_merkletree() {
        let d = "D".to_string();
        let e = "E".to_string();
        let f = "F".to_string();
        let g = "G".to_string();

        let e_hash = keccak256(&e);
        let f_hash = keccak256(&f);
        let g_hash = keccak256(&g);

        let fg_hash = keccak256(&format!("{}{}", f_hash, g_hash));

        let values_n_path = vec![d.clone(), e_hash, fg_hash];

        let mtree = MerkleTreeVec::new(vec![d, e, f, g], &keccak256);

        assert!(mtree.in_merkletree(values_n_path));
    }
}
