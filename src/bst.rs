#[derive(Debug)]
struct Data(u64, String);

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
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn new_tree() {
        let mut movie_reviews = BTreeMap::new();

        movie_reviews.insert("Office Space", Data(10, "Deals with real issues in the workplace.".to_string()));
        movie_reviews.insert("Pulp Fiction", Data(2, "Masterpiece.".to_string()));
        movie_reviews.insert("The Godfather", Data(1, "Very enjoyable..".to_string()));
        movie_reviews.insert("The Blues Brothers", Data(99, "Eye lyked it alot.".to_string()));

        println!("{:?}", movie_reviews);

        let mut bst = BST::Teminal;

        bst.add("Office Space");
        bst.add("Pulp Fiction");
        bst.add("The Godfather");
        bst.add("The Blues Brothers");

        println!("{:?}", bst.get("Pulp Fiction"));

        if let Option::Some(ref mut node) = bst.get("Pulp Fiction") {
            node.v = "Pulp Fiction2";
        }

        println!("{:?}", bst);

    }
}
