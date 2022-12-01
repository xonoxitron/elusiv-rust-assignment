use super::super::hashers::hash;
use super::super::utils::{is_pow2, split_vec};
use super::{node::Node, Leaf, Root};

#[derive(Debug)]
pub struct MTree {
    pub leaves: Vec<Leaf>,
    pub root: Root,
    pub height: usize,
}

impl MTree {
    pub fn insert(leaves: Vec<&str>) -> Self {
        let leaves_len: u32 = leaves.len() as u32;

        match is_pow2(leaves_len) {
            true => MTree {
                leaves: leaves.iter().map(|i| hash(i.to_string())).collect(),
                root: MTree::make_root(leaves.iter().map(|i| hash(i.to_string())).collect()),
                height: Self::calculate_height(leaves.len()),
            },
            false => {
                panic!("{} is not a power-2 leaf", leaves.len())
            }
        }
    }

    fn calculate_height(leaves_len: usize) -> usize {
        match leaves_len == 1 {
            true => 0,
            false => (leaves_len as f32).sqrt().round() as usize,
        }
    }

    fn make_root(leaves: Vec<Leaf>) -> String {
        if leaves.len() == 1 {
            return leaves[0].to_owned();
        } else {
            let mut new_leaves: Vec<String> = vec![];

            for i in leaves.chunks(2) {
                new_leaves.push(hash(i.concat()));
            }
            return MTree::make_root(new_leaves);
        }
    }

    fn make_proof(leaves: Vec<Leaf>, leaf: Leaf, proof: &mut Vec<Node>) -> Vec<Node> {
        let height = Self::calculate_height(leaves.len());
        if leaves.len() == 2 {
            proof.push(Node::right(&leaves[1], height));
            proof.push(Node::left(&leaves[0], height));
            return proof.to_vec();
        } else {
            let index = match leaves.iter().position(|_leaf| _leaf == &leaf) {
                Some(index) => index,
                None => panic!("leaf: {} does not exist in the tree: {:?}", leaf, leaves),
            };

            let (left, right) = split_vec(&leaves);

            let h = leaves.len() / 2;

            if index < h {
                proof.push(Node::right(&MTree::make_root(right), height));
                return MTree::make_proof(left, leaf, proof);
            } else {
                proof.push(Node::left(&MTree::make_root(left), height));
                return MTree::make_proof(right, leaf, proof);
            }
        }
    }

    pub fn proof(self, leaf: &str) -> Vec<Node> {
        let mut proof = MTree::make_proof(self.leaves, leaf.to_string(), &mut vec![]);
        proof.reverse();
        return proof;
    }

    pub fn get_value(self, index: usize) -> Leaf {
        self.leaves[index].to_owned()
    }

    pub fn get_opening(self, index: usize) -> Vec<Node> {
        let leaf = self.leaves[index].to_owned();
        Self::proof(self, &leaf.to_string())
    }
}
