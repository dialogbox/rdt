#[derive(Debug)]
struct Data(u64, String);

#[derive(Debug)]
struct BST<K, V> {
    root: NodeEntry<K, V>
}

#[derive(Debug)]
enum NodeEntry<K, V> {
    Node(K, V, Box<NodeEntry<K, V>>, Box<NodeEntry<K, V>>),
    Terminal,
}

impl <K, V> BST<K, V> where K: PartialOrd {
    pub fn new() -> BST<K, V> {
        BST::<K, V> { root: NodeEntry::Terminal }
    }

    pub fn add(&mut self, k: K, v: V) {
        self.root = NodeEntry::Node(k, v, Box::new(NodeEntry::Terminal), Box::new(NodeEntry::Terminal));
    }

    fn try_search<'a>(cur: &'a NodeEntry<K, V>, k: K) -> &'a NodeEntry<K, V> {
        match cur {
            NodeEntry::Terminal => &cur,
            NodeEntry::Node(ref ck, _, _, _) if *ck == k => &cur,
            NodeEntry::Node(ref ck, _, ref l, _) if *ck < k => BST::try_search(l, k),
            NodeEntry::Node(_, _, _, ref r) => BST::try_search(r, k),
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

        let mut bst = BST::<&str, Data>::new();

        bst.add("Office Space", Data(10, "Deals with real issues in the workplace.".to_string()));

        println!("{:?}", bst);
    }
}
