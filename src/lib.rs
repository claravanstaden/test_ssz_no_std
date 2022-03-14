#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;

type Root = [u8; 32];
use ssz_rs_derive::SimpleSerialize;
use ssz_rs::{Sized, Deserialize};
use alloc::vec::Vec;

#[derive(Debug, Default, SimpleSerialize)]
pub struct BeaconBlockHeader {
    pub body_root: Root,
}

#[cfg(test)]
mod tests {
    use ssz_rs::MerkleizationContext;
    use ssz_rs::Merkleized;
    use crate::BeaconBlockHeader;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_hashes_to_a_merkle_root() {
        let block = BeaconBlockHeader{ body_root: [
            90, 34, 54, 34, 43, 23, 42, 42, 90, 32, 34, 34, 34, 90, 34, 54, 34, 43, 23, 42, 42, 90, 32, 34, 34, 34, 90, 34, 54, 34, 43, 30
        ] };

        let context = MerkleizationContext::new();
        let block_hash_result = block.hash_tree_root(&context);

        match block_hash_result {
            Ok(hash) => hash,
            Err(err) => panic!("error getting hash {}", err),
        };
    }
}
