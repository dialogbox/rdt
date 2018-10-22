#[derive(Debug)]
struct BST<T: PartialOrd> {
    root: Option<&'a mut TreeNode<T>>,
}

#[derive(Debug)]
struct TreeNode<'a, T: PartialOrd> {
    value: T,
    left: Option<&'a mut TreeNode<T>>,
    right: Option<&'a mut TreeNode<T>>),
}

impl<T: PartialOrd> BST<T> {
    fn add(&mut self, n: T) {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_tree() {
        let t = &BST::<i64> {
            root: TreeNode::Teminal,
        };

        println!("{:?}", t);
    }
}
