#[derive(Debug)]
enum BST<T> {
    Teminal,
    Node(Box<TreeNode<T>>)
}

#[derive(Debug)]
struct TreeNode<T> {
    v: T,
    l: BST<T>,
    r: BST<T>
}

impl <T: Ord> BST<T> {
    fn add(&mut self, value: T) {
        match *self {
            BST::Teminal =>
                *self = BST::Node(Box::new(TreeNode {
                    v: value,
                    l: BST::Teminal,
                    r: BST::Teminal,
                })),
            BST::Node(ref mut node) =>
                if value <= node.v {
                    node.l.add(value);
                } else {
                    node.r.add(value);
                }
        }
    }

    fn get(&mut self, value: T) -> Option<&mut TreeNode<T>> {
        match *self {
            BST::Teminal => Option::None,
            BST::Node(ref mut node) if value == node.v => Option::Some(node),
            BST::Node(ref mut node) if value > node.v => node.r.get(value),
            BST::Node(ref mut node) => node.l.get(value),
        }
    }

    fn delete(&mut self, value: T) {
        panic!("Not implemented!");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_tree() {
        let mut bst = BST::Teminal;

        bst.add("Office Space");
        bst.add("Pulp Fiction");
        bst.add("The Godfather");
        bst.add("The Blues Brothers");

        assert_eq!(bst.get("Office Space").unwrap().v, "Office Space");
        assert_eq!(bst.get("Pulp Fiction").unwrap().v, "Pulp Fiction");
        assert_eq!(bst.get("The Godfather").unwrap().v, "The Godfather");
        assert_eq!(bst.get("The Blues Brothers").unwrap().v, "The Blues Brothers");

        if let Option::Some(ref mut node) = bst.get("Pulp Fiction") {
            node.v = "Pulp Fiction2";
        }

        assert_eq!(bst.get("Pulp Fiction").is_none(), true);
        assert_eq!(bst.get("Pulp Fiction2").unwrap().v, "Pulp Fiction2");
    }

    #[test]
    fn tree_delete() {
        let mut bst = BST::Teminal;

        bst.add(10);
        bst.add(11);
        bst.add(6);
        bst.add(4);
        bst.add(2);
        bst.add(3);
        bst.add(5);

        assert_eq!(bst.get(10).unwrap().v, 10);
        assert_eq!(bst.get(11).unwrap().v, 11);
        assert_eq!(bst.get(6).unwrap().v,  6);
        assert_eq!(bst.get(4).unwrap().v,  4);
        assert_eq!(bst.get(2).unwrap().v,  2);
        assert_eq!(bst.get(3).unwrap().v,  3);
        assert_eq!(bst.get(5).unwrap().v,  5);

        bst.delete(4);

        assert_eq!(bst.get(4).is_none(), true);
    }
}