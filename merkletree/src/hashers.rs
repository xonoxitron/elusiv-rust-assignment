use crate::mtree::Leaf;

fn to_keccak256(message: String) -> Leaf {
    use tiny_keccak::{Hasher, Keccak};
    let mut k256 = Keccak::v256();
    let mut result = [0; 32];
    k256.update(message.as_bytes());
    k256.finalize(&mut result);
    hex::encode(result)
}

/* different hashers can be implemented here */

pub fn hash(message: String) -> Leaf {
    to_keccak256(message)
}
