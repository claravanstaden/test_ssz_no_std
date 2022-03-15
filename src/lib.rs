#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;

type Root = [u8; 32];

use ssz_rs_derive::SimpleSerialize;
use ssz_rs::{Sized, Deserialize, MerkleizationContext, Merkleized};
use alloc::vec::Vec;
use milagro_bls::{AggregatePublicKey, AggregateSignature, PublicKey, Signature};

#[derive(Debug, Default, SimpleSerialize)]
pub struct BeaconBlockHeader {
    pub body_root: Root,
}

fn hash_tree_root() {
    let block = BeaconBlockHeader {
        body_root: [
            90, 34, 54, 34, 43, 23, 42, 42, 90, 32, 34, 34, 34, 90, 34, 54, 34, 43, 23, 42, 42, 90, 32, 34, 34, 34, 90, 34, 54, 34, 43, 30
        ]
    };

    let context = MerkleizationContext::new();
    let block_hash_result = block.hash_tree_root(&context);

    match block_hash_result {
        Ok(hash) => hash,
        Err(err) => panic!("error getting hash {}", err),
    };
}

fn bls_signature_verification() {
    let signature: Vec<u8> = vec![62, 32, 30, 34, 65, 39, 36, 35, 36, 63, 62, 65, 62, 37, 39, 61, 39, 61, 38, 65, 33, 39, 37, 39, 32, 30, 66, 64, 38, 65, 36, 30, 63, 35, 66, 35, 64, 39, 34, 34, 33, 66, 35, 38, 64, 34, 32, 31, 38, 36, 66, 37, 37, 33, 63, 36, 61, 64, 65, 32, 62, 64, 32, 36, 33, 65, 32, 66, 65, 36, 64, 62, 64, 63, 34, 37, 66, 31, 34, 38, 66, 38, 37, 31, 65, 64, 39, 61, 30, 30, 62, 38, 61, 63, 38, 62, 31, 37, 61, 34, 30, 64, 36, 35, 63, 38, 64, 30, 32, 31, 32, 30, 63, 30, 30, 64, 63, 61, 37, 37, 34, 39, 35, 38, 38, 38, 33, 36, 36, 62, 34, 63, 63, 63, 31, 30, 66, 31, 63, 36, 64, 61, 61, 30, 32, 64, 62, 36, 61, 37, 35, 31, 36, 35, 35, 35, 63, 61, 30, 36, 36, 35, 62, 63, 61, 39, 32, 61, 36, 34, 37, 62, 35, 66, 33, 61, 35, 31, 34, 66, 61, 30, 38, 33, 66, 64, 63, 35, 33, 62, 36, 65];
    let message: Vec<u8> = vec![36, 39, 32, 34, 31, 65, 37, 31, 34, 36, 63, 64, 63, 63, 35, 61, 35, 64, 64, 63, 39, 61, 36, 30, 62, 61, 62, 38, 66, 33, 37, 38, 63, 30, 32, 37, 31, 65, 35, 34, 38, 30, 36, 35, 61, 33, 38, 62, 63, 63, 36, 30, 36, 32, 34, 65, 31, 64, 62, 65, 64, 39, 37, 66];
    let pubkeys = vec![
        vec![61, 37, 33, 65, 62, 39, 39, 31, 61, 61, 32, 32, 63, 64, 62, 37, 39, 34, 64, 61, 36, 66, 63, 64, 65, 35, 35, 61, 34, 32, 37, 66, 30, 61, 34, 64, 66, 35, 61, 34, 61, 37, 30, 64, 65, 32, 33, 61, 39, 38, 38, 62, 35, 65, 35, 66, 63, 38, 63, 34, 64, 38, 34, 34, 66, 36, 36, 64, 39, 39, 30, 32, 37, 33, 32, 36, 37, 61, 35, 34, 64, 64, 32, 31, 35, 37, 39, 62, 37, 62, 61, 36, 61, 30, 38, 36],
        vec![62, 32, 39, 30, 34, 33, 61, 37, 32, 37, 33, 64, 30, 61, 32, 64, 62, 63, 32, 62, 37, 34, 37, 64, 63, 66, 36, 61, 35, 65, 63, 63, 62, 64, 37, 63, 63, 62, 34, 34, 62, 32, 64, 37, 32, 65, 39, 38, 35, 35, 33, 37, 62, 31, 31, 37, 39, 32, 39, 62, 63, 33, 66, 64, 33, 61, 39, 39, 30, 30, 31, 34, 38, 31, 33, 32, 37, 37, 38, 38, 61, 64, 30, 34, 30, 62, 34, 30, 37, 37, 63, 34, 37, 63, 30, 64],
        vec![62, 39, 32, 38, 66, 33, 62, 65, 62, 39, 33, 35, 31, 39, 65, 65, 63, 66, 30, 31, 34, 35, 64, 61, 39, 30, 33, 62, 34, 30, 61, 34, 63, 39, 37, 64, 63, 61, 30, 30, 62, 32, 31, 66, 31, 32, 61, 63, 30, 64, 66, 33, 62, 65, 39, 31, 31, 36, 65, 66, 32, 65, 66, 32, 37, 62, 32, 61, 65, 36, 62, 63, 64, 34, 63, 35, 62, 63, 32, 64, 35, 34, 65, 66, 35, 61, 37, 30, 36, 32, 37, 65, 66, 63, 62, 37],
        vec![39, 34, 34, 36, 34, 30, 37, 62, 63, 64, 38, 65, 35, 65, 66, 65, 39, 66, 32, 61, 63, 30, 65, 66, 62, 66, 61, 39, 65, 30, 37, 64, 31, 33, 36, 65, 36, 38, 62, 30, 33, 63, 35, 65, 62, 63, 35, 62, 64, 65, 34, 33, 64, 62, 33, 62, 39, 34, 37, 37, 33, 64, 65, 38, 36, 30, 35, 63, 33, 30, 34, 31, 39, 65, 62, 32, 35, 39, 36, 35, 31, 33, 37, 30, 37, 65, 34, 65, 37, 34, 34, 38, 62, 62, 35, 30],
    ];

    let sig = Signature::from_bytes(&signature[..]);

    if let Err(_) = sig {
        panic!("signature could not be verified");
    }

    let agg_sig = AggregateSignature::from_signature(&sig.unwrap());

    let public_keys_res: Result<Vec<PublicKey>, _> =
        pubkeys.iter().map(|bytes| PublicKey::from_bytes(&bytes)).collect();

    if let Err(_) = public_keys_res {
        panic!("signature could not be verified");
    }

    let agg_pub_key_res = AggregatePublicKey::into_aggregate(&public_keys_res.unwrap());

    if let Err(_) = agg_pub_key_res {
        panic!("signature could not be verified");
    }


    if agg_sig.fast_aggregate_verify_pre_aggregated(&message.as_slice(), &agg_pub_key_res.unwrap()) {
        panic!("signature could not be verified");
    }
}

#[cfg(test)]
mod tests {
    use crate::BeaconBlockHeader;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
