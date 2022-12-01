use crate::mtree::Leaf;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub r: Option<Leaf>,
    pub l: Option<Leaf>,
    pub h: usize,
}

impl Node {
    pub fn none() -> Node {
        Node {
            r: None,
            l: None,
            h: 0,
        }
    }

    pub fn left(l: &str, h: usize) -> Node {
        Node {
            r: None,
            l: Some(l.to_string()),
            h,
        }
    }

    pub fn right(r: &str, h: usize) -> Node {
        Node {
            r: Some(r.to_string()),
            l: None,
            h,
        }
    }
}
