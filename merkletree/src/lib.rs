pub mod hashers;
pub mod mtree;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::mtree::{node::Node, tree::MTree};

    #[test]
    fn insert_single_leaf() {
        let t1h0 = MTree::insert(vec!["alpha"]);
        println!("{:?}", t1h0);
        assert_eq!(t1h0.leaves.len(), 1)
    }

    #[test]
    fn insert_multiple_leaves() {
        let t2h1 = MTree::insert(vec!["alpha", "beta"]);
        println!("{:?}", t2h1);
        assert_eq!(t2h1.leaves.len(), 2);

        let t3h2 = MTree::insert(vec!["alpha", "beta", "gamma", "delta"]);
        println!("{:?}", t3h2);
        assert_eq!(t3h2.leaves.len(), 4);

        let t4h3 = MTree::insert(vec![
            "alpha", "beta", "gamma", "delta", "epsilon", "zeta", "eta", "theta",
        ]);
        println!("{:?}", t4h3);
        assert_eq!(t4h3.leaves.len(), 8);

        let t5h4 = MTree::insert(vec![
            "alpha", "beta", "gamma", "delta", "epsilon", "zeta", "eta", "theta", "iota", "kappa",
            "lambda", "mi", "ni", "xi", "omicron", "pi",
        ]);
        println!("{:?}", t5h4);
        assert_eq!(t5h4.leaves.len(), 16);
    }

    #[test]
    fn get_root() {
        let merkletree = MTree::insert(vec!["alpha", "beta"]);
        println!("{:?}", merkletree);
        assert_eq!(
            merkletree.root,
            "b719c680a9a944124f8a83a08286633dd93b29430ad86f42725390a567ec86aa"
        )
    }

    #[test]
    fn get_value() {
        let merkletree = MTree::insert(vec![
            "alpha", "beta", "gamma", "delta", "epsilon", "zeta", "eta", "theta",
        ]);
        println!("{:?}", merkletree);
        assert_eq!(
            merkletree.get_value(4),
            "232e392fb36817e5ae726f4e9ef62081bc74bb3198fdba05c3b10a525ff4eb2b"
        )
    }

    #[test]
    fn get_opening() {
        let merkletree = MTree::insert(vec!["alpha", "beta", "gamma", "delta"]);
        println!("{:?}", merkletree);
        let opening = merkletree.get_opening(3);
        println!("{:?}", opening);
        assert_eq!(
            opening,
            vec![
                Node::left(
                    "8ec2a252e6833332e4d11a94054bb27c27c7220a2efddc664860650b61f98fc2",
                    1
                ),
                Node::right(
                    "e0d399e873cde9f1130182a2b70db45e021df5a2f404fa14e8b2f7481c10f1d3",
                    1
                ),
                Node::left(
                    "b719c680a9a944124f8a83a08286633dd93b29430ad86f42725390a567ec86aa",
                    2
                )
            ]
        )
    }
}
