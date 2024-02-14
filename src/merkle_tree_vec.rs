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

#[macro_export]
macro_rules! slice_to_string {
    ($($x:expr),*) => ([$($x.to_string()),*]);
}

#[macro_export]
macro_rules! vec_to_string {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

// This code assumes that the length of initial_leaves is a power of 2 (i.e., initial_leaves.len() == 2^N).
// This condition must be satisfied for optimal performance.
// Otherwise, additional copy operations may be required at each level of the computation as needed.
impl<'a> MerkleTreeVec<'a> {
    pub fn new(
        initial_leaves: &[String],
        hash_fn: &'a dyn Fn(&str) -> String,
    ) -> MerkleTreeVec<'a> {
        let mut upper_children: Vec<String> = Vec::new();

        let hashed_initial_leaves = initial_leaves
            .iter()
            .map(|s| hash_fn(s))
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
                initial_leaves: initial_leaves.into(),
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
            initial_leaves: initial_leaves.into(),
            hash_fn,
        }
    }

    pub fn push_to_initial(&mut self, new_leaves: &mut Vec<String>) {
        let mut new_initial_leaves = self.initial_leaves.clone();
        new_initial_leaves.append(new_leaves);

        *self = MerkleTreeVec::new(&new_initial_leaves, self.hash_fn);
    }

    pub fn get_proof(&self, item: &str) -> Result<Vec<String>, String> {
        let mut upper_children: Vec<String> = Vec::new();
        let mut proof: Vec<String> = Vec::new();
        let mut index_item = self.get_index(item)?;

        let hashed_initial_leaves = self
            .initial_leaves
            .iter()
            .map(|s| (self.hash_fn)(s))
            .collect::<Vec<String>>();

        for (index, chunk) in hashed_initial_leaves.chunks(2).enumerate() {
            let left = chunk[0].clone();
            // "Balance" the tree -> Duplicate if there is no right leaf
            let right = if chunk.len() > 1 {
                chunk[1].clone()
            } else {
                chunk[0].clone()
            };

            // Check if the item's index falls within the current chunk
            let chunk_start_index = index * 2;
            if chunk_start_index <= index_item && index_item < chunk_start_index + 2 {
                // Calculate the relative index within the chunk
                let relative_index = index_item - chunk_start_index;

                // Add the sibling to the proof vector based on the relative index within the chunk
                if relative_index == 0 {
                    proof.push(right.clone());
                } else {
                    proof.push(left.clone());
                }

                // Update index_item to the index of the parent node
                index_item = index;
            }

            let combined_hash = (self.hash_fn)(&format!("{}{}", left, right));

            upper_children.push(combined_hash);
        }

        if upper_children.len() == 1 {
            return Ok(proof);
        }

        while upper_children.len() > 1 {
            let mut new_upper_children: Vec<String> = Vec::new();

            for (index, chunk) in upper_children.chunks(2).enumerate() {
                let left = chunk[0].clone();
                // "Balance" the tree -> Duplicate if there is no right leaf
                let right = if chunk.len() > 1 {
                    chunk[1].clone()
                } else {
                    chunk[0].clone()
                };

                let combined_hash = (self.hash_fn)(&format!("{}{}", left, right));

                // Check if the item's index falls within the current chunk
                let chunk_start_index = index * 2;
                if chunk_start_index <= index_item && index_item < chunk_start_index + 2 {
                    // Calculate the relative index within the chunk
                    let relative_index = index_item - chunk_start_index;

                    // Add the sibling to the proof vector based on the relative index within the chunk
                    if relative_index == 0 {
                        proof.push(right.clone());
                    } else {
                        proof.push(left.clone());
                    }

                    // Update index_item to the index of the parent node
                    index_item = index;
                }

                new_upper_children.push(combined_hash);
            }
            upper_children = new_upper_children;
        }
        Ok(proof)
    }

    pub fn verify(&self, proof: Vec<String>, item: &str) -> Result<bool, String> {
        let index = self.get_index(item)?;
        let mut check_root = keccak256(item);
        let mut combined_hash;
        for h in &proof {
            if index % 2 == 0 {
                combined_hash = (self.hash_fn)(&format!("{}{}", check_root, h));
            } else {
                combined_hash = (self.hash_fn)(&format!("{}{}", h, check_root));
            }

            check_root = combined_hash;
        }
        Ok(check_root == self.root)
    }

    pub fn get_index(&self, item: &str) -> Result<usize, String> {
        let index = self.initial_leaves.iter().position(|x| x == item);
        match index {
            Some(x) => return Ok(x),
            None => return Err("Item not found".to_string()),
        }
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

        let mtree = MerkleTreeVec::new(&[d, e], &keccak256);

        assert_eq!(mtree.root, cmp);
    }
    #[test]
    fn test_new_with_duplication() {
        let d = "D".to_string();
        let d_hash = keccak256(&d);

        let cmp = keccak256(&format!("{}{}", d_hash, d_hash));

        let mtree = MerkleTreeVec::new(&[d], &keccak256);

        assert_eq!(mtree.root, cmp);
    }
    #[test]
    fn test_push() {
        let d = "D".to_string();
        let e = "E".to_string();

        let d_hash = keccak256(&d);
        let e_hash = keccak256(&e);

        let cmp = keccak256(&format!("{}{}", d_hash, e_hash));

        let mut mtree = MerkleTreeVec::new(&[d], &keccak256);
        mtree.push_to_initial(&mut vec![e]);

        assert_eq!(mtree.root, cmp);
    }

    #[test]
    fn test_verify() {
        let d = "D".to_string();
        let e = "E".to_string();
        let f = "F".to_string();
        let g = "G".to_string();

        let e_hash = keccak256(&e);
        let f_hash = keccak256(&f);
        let g_hash = keccak256(&g);

        let fg_hash = keccak256(&format!("{}{}", f_hash, g_hash));

        let proof = vec![e_hash, fg_hash];

        let mtree = MerkleTreeVec::new(&[d.clone(), e, f, g], &keccak256);

        assert!(mtree.verify(proof, "D").unwrap());
    }
    #[test]
    fn test_get_proof_of4() {
        let d = "D".to_string();
        let e = "E".to_string();
        let f = "F".to_string();
        let g = "G".to_string();

        let e_hash = keccak256(&e);
        let f_hash = keccak256(&f);
        let g_hash = keccak256(&g);

        let fg_hash = keccak256(&format!("{}{}", f_hash, g_hash));

        let proof_verify = vec![e_hash, fg_hash];

        let mtree = MerkleTreeVec::new(&[d.clone(), e, f, g], &keccak256);
        let proof = mtree.get_proof(&d.clone());
        assert_eq!(proof.unwrap(), proof_verify);
    }
    #[test]
    fn test_get_proof_of8() {
        let d = "D".to_string();
        let e = "E".to_string();
        let f = "F".to_string();
        let g = "G".to_string();
        let h = "H".to_string();
        let i = "I".to_string();
        let j = "J".to_string();
        let k = "K".to_string();

        let d_hash = keccak256(&d);
        let e_hash = keccak256(&e);
        let f_hash = keccak256(&f);
        let g_hash = keccak256(&g);
        let h_hash = keccak256(&h);
        let i_hash = keccak256(&i);
        let j_hash = keccak256(&j);
        let k_hash = keccak256(&k);

        let de_hash = keccak256(&format!("{}{}", d_hash, e_hash));
        let fg_hash = keccak256(&format!("{}{}", f_hash, g_hash));
        let jk_hash = keccak256(&format!("{}{}", j_hash, k_hash));
        let defg_hash = keccak256(&format!("{}{}", de_hash, fg_hash));

        let proof_verify = vec![i_hash, jk_hash, defg_hash];

        let mtree = MerkleTreeVec::new(&[d.clone(), e, f, g, h.clone(), i, j, k], &keccak256);
        let proof = mtree.get_proof(&h);
        assert_eq!(proof.unwrap(), proof_verify);
    }
    #[test]
    fn test_get_proof_of8_with_dup() {
        let d = "D".to_string();
        let e = "E".to_string();
        let f = "F".to_string();
        let g = "G".to_string();
        let h = "H".to_string();
        let i = "I".to_string();

        let d_hash = keccak256(&d);
        let e_hash = keccak256(&e);
        let f_hash = keccak256(&f);
        let g_hash = keccak256(&g);
        let h_hash = keccak256(&h);
        let i_hash = keccak256(&i);

        let de_hash = keccak256(&format!("{}{}", d_hash, e_hash));
        let fg_hash = keccak256(&format!("{}{}", f_hash, g_hash));
        let hi_hash = keccak256(&format!("{}{}", h_hash, i_hash));
        let defg_hash = keccak256(&format!("{}{}", de_hash, fg_hash));

        let proof_verify = vec![h_hash, hi_hash, defg_hash];

        let mtree = MerkleTreeVec::new(&[d, e, f, g, h, i.clone()], &keccak256);
        let proof = mtree.get_proof(&i);
        assert_eq!(proof.unwrap(), proof_verify);
    }
}
