#[derive(Debug)]
pub enum BST<T> {
    Teminal,
    Node(Box<TreeNode<T>>),
}

#[derive(Debug)]
pub struct TreeNode<T> {
    v: T,
    l: BST<T>,
    r: BST<T>,
}

impl<T: Ord> BST<T> {
    pub fn add(&mut self, value: T) {
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

    pub fn get(&self, value: T) -> Option<&TreeNode<T>> {
        match *self {
            BST::Teminal => Option::None,
            BST::Node(ref node) if value > node.v => node.r.get(value),
            BST::Node(ref node) if value < node.v => node.l.get(value),
            BST::Node(ref node) => Option::Some(node),
        }
    }

    pub fn get_mut(&mut self, value: T) -> Option<&mut TreeNode<T>> {
        match *self {
            BST::Teminal => Option::None,
            BST::Node(ref mut node) if value > node.v => node.r.get_mut(value),
            BST::Node(ref mut node) if value < node.v => node.l.get_mut(value),
            BST::Node(ref mut node) => Option::Some(node),
        }
    }

    fn pop_min(self) -> (BST<T>, Option<T>) {
        match self {
            BST::Teminal => (BST::Teminal, Option::None),
            BST::Node(node) => {
                let node = *node;
                let TreeNode { v, l, r } = node;

                match (r, l) {
                    (BST::Teminal, BST::Teminal) => (BST::Teminal, Option::Some(v)),
                    (r @ BST::Node(_), BST::Teminal) => (r, Option::Some(v)),
                    (r @ _, l @ BST::Node(_)) => {
                        let (l, minv) = l.pop_min();
                        (BST::Node(Box::new(TreeNode { v, l, r })), minv)
                    }
                }
            }
        }
    }

    fn pop_max(self) -> (BST<T>, Option<T>) {
        match self {
            BST::Teminal => (BST::Teminal, Option::None),
            BST::Node(node) => {
                let node = *node;
                let TreeNode { v, l, r } = node;

                match (l, r) {
                    (BST::Teminal, BST::Teminal) => (BST::Teminal, Option::Some(v)),
                    (l @ BST::Node(_), BST::Teminal) => (l, Option::Some(v)),
                    (l @ _, r @ BST::Node(_)) => {
                        let (r, maxv) = r.pop_max();
                        (BST::Node(Box::new(TreeNode { v, l, r })), maxv)
                    }
                }
            }
        }
    }

    pub fn delete(self, value: T) -> BST<T> {
        match self {
            BST::Teminal => self,
            BST::Node(node) => {
                if value > node.v {
                    // https://stackoverflow.com/a/28469639
                    // Once nll is stabilized this could be simply like this
                    // let TreeNode { v, l, r } = *node;
                    let node = *node;
                    let TreeNode { v, l, r } = node;

                    let r = r.delete(value);
                    BST::Node(Box::new(TreeNode { v, l, r }))
                } else if value < node.v {
                    let node = *node;
                    let TreeNode { v, l, r } = node;
                    let l = l.delete(value);
                    BST::Node(Box::new(TreeNode { v, l, r }))
                } else {
                    // https://en.wikipedia.org/wiki/Binary_search_tree#Deletion
                    // T. Hibbard in 1962
                    let node = *node;
                    let TreeNode { v: _, l, r } = node;

                    match (l, r) {
                        (BST::Teminal, BST::Teminal) => BST::Teminal,
                        (l @ BST::Node(_), BST::Teminal) => l,
                        (BST::Teminal, r @ BST::Node(_)) => r,
                        (l @ BST::Node(_), r @ BST::Node(_)) => {
                            let (l, maxl) = l.pop_max();
                            BST::Node(Box::new(TreeNode { v: maxl.unwrap(), l, r }))
                        }
                    }
                }
            }
        }
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

        if let Option::Some(ref mut node) = bst.get_mut("Pulp Fiction") {
            node.v = "Pulp Fiction2";
        }

        assert_eq!(bst.get("Pulp Fiction").is_none(), true);
        assert_eq!(bst.get("Pulp Fiction2").unwrap().v, "Pulp Fiction2");
    }

    #[test]
    fn test_pop_min() {
        let bst = BST::Teminal;

        let (mut bst, min) = bst.pop_min();
        assert_eq!(min, Option::None);

        //       10
        //      /  \
        //     6   11
        //    /
        //   4
        //  / \
        // 2   5
        //  \
        //   3
        bst.add(10);
        bst.add(11);
        bst.add(6);
        bst.add(4);
        bst.add(2);
        bst.add(3);
        bst.add(5);

        let (bst, min) = bst.pop_min();
        assert_eq!(min, Option::Some(2));
        let (bst, min) = bst.pop_min();
        assert_eq!(min, Option::Some(3));
        let (bst, min) = bst.pop_min();
        assert_eq!(min, Option::Some(4));
        let (bst, min) = bst.pop_min();
        assert_eq!(min, Option::Some(5));
        let (bst, min) = bst.pop_min();
        assert_eq!(min, Option::Some(6));
        let (bst, min) = bst.pop_min();
        assert_eq!(min, Option::Some(10));
        let (bst, min) = bst.pop_min();
        assert_eq!(min, Option::Some(11));
        let (bst, min) = bst.pop_min();
        assert_eq!(min, Option::None);

        match bst {
            BST::Node(_) => panic!("BST should be empty"),
            _ => ()
        }
    }

    #[test]
    fn test_pop_max() {
        let bst = BST::Teminal;

        let (mut bst, max) = bst.pop_max();
        assert_eq!(max, Option::None);

        //       10
        //      /  \
        //     6   11
        //    /
        //   4
        //  / \
        // 2   5
        //  \
        //   3
        bst.add(10);
        bst.add(11);
        bst.add(6);
        bst.add(4);
        bst.add(2);
        bst.add(3);
        bst.add(5);

        let (bst, max) = bst.pop_max();
        assert_eq!(max, Option::Some(11));
        let (bst, max) = bst.pop_max();
        assert_eq!(max, Option::Some(10));
        let (bst, max) = bst.pop_max();
        assert_eq!(max, Option::Some(6));
        let (bst, max) = bst.pop_max();
        assert_eq!(max, Option::Some(5));
        let (bst, max) = bst.pop_max();
        assert_eq!(max, Option::Some(4));
        let (bst, max) = bst.pop_max();
        assert_eq!(max, Option::Some(3));
        let (bst, max) = bst.pop_max();
        assert_eq!(max, Option::Some(2));
        let (bst, max) = bst.pop_max();
        assert_eq!(max, Option::None);

        match bst {
            BST::Node(_) => panic!("BST should be empty"),
            _ => ()
        }
    }

    #[test]
    fn tree_delete() {
        let mut bst = BST::Teminal;

        //       10
        //      /  \
        //     6   11
        //    /
        //   4
        //  / \
        // 2   5
        //  \
        //   3
        bst.add(10);
        bst.add(11);
        bst.add(6);
        bst.add(4);
        bst.add(2);
        bst.add(3);
        bst.add(5);

        assert_eq!(bst.get(10).unwrap().v, 10);
        assert_eq!(bst.get(11).unwrap().v, 11);
        assert_eq!(bst.get(6).unwrap().v, 6);
        assert_eq!(bst.get(4).unwrap().v, 4);
        assert_eq!(bst.get(2).unwrap().v, 2);
        assert_eq!(bst.get(3).unwrap().v, 3);
        assert_eq!(bst.get(5).unwrap().v, 5);

        bst = bst.delete(3);
        bst = bst.delete(11);
        bst = bst.delete(6);

        assert_eq!(bst.get(3).is_none(), true);
        assert_eq!(bst.get(11).is_none(), true);
        assert_eq!(bst.get(6).is_none(), true);

        bst = bst.delete(4);

        assert_eq!(bst.get(4).is_none(), true);

        let mut bst = BST::Teminal;

        //       10
        //      /  \
        //     6   11
        //    /
        //   4
        //  / \
        // 2   5
        //  \
        //   3
        bst.add(10);
        bst.add(11);
        bst.add(6);
        bst.add(4);
        bst.add(2);
        bst.add(3);
        bst.add(5);

        //     6
        //    / \
        //   4   11
        //  / \
        // 2   5
        //  \
        //   3
        bst = bst.delete(10);
        assert_eq!(bst.get(10).is_none(), true);

        //     6
        //    / \
        //   3   11
        //  / \
        // 2   5
        bst = bst.delete(4);
        assert_eq!(bst.get(4).is_none(), true);

        //     6
        //    / \
        //   2   11
        //    \
        //     5
        bst = bst.delete(3);
        assert_eq!(bst.get(3).is_none(), true);
    }

    #[test]
    fn test_delete_the_only_node() {
        let mut bst = BST::Teminal;
        bst.add(10);
        bst = bst.delete(10);

        assert_eq!(bst.get(10).is_none(), true);

        match bst {
            BST::Node(_) => panic!("BST should be empty"),
            _ => ()
        }
    }
}