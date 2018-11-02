// Max Heap
#[derive(Debug)]
pub struct Heap<T>(Vec<T>);

impl<T: Ord> Heap<T> {
    pub fn new() -> Heap<T> {
        Heap(Vec::new())
    }

    pub fn push(&mut self, value: T) -> usize {
        self.0.push(value);
        self.rearrange_up()
    }

    pub fn pop(&mut self) -> Option<T> {
        let len = self.0.len();
        if len == 0 {
            return Option::None;
        }

        self.0.swap(0, len-1);
        let result = self.0.pop();

        let len = self.0.len();
        if len > 1 {
            self.rearrange_down();
        }

        result
    }

    // heap rearrange up of last item
    fn rearrange_up(&mut self) -> usize {
        fn parent_of(current: usize) -> usize { current / 2 }

        let v = &mut self.0;
        let mut pos = v.len() -1;

        while pos > 0 {
            let parent = parent_of(pos);

            if v[parent] >= v[pos] {
                return pos;
            }

            v.swap(parent, pos);
            pos = parent;
        }

        return pos;
    }

    // heap rearrange down of first item
    fn rearrange_down(&mut self) -> usize {
        fn lchild_of(current: usize) -> usize { current * 2 + 1 }
        fn rchild_of(current: usize) -> usize { current * 2 + 2 }

        let v = &mut self.0;
        let mut pos = 0;

        let last = v.len() -1;

        while (lchild_of(pos)) <= last { // until pos has no child
            let lchild = lchild_of(pos);
            let rchild = rchild_of(pos);

            if v[pos] >= v[lchild] && (/* if no rchild */ last < rchild || v[pos] >= v[rchild]) {
                return pos;
            }

            if last < rchild || v[lchild] > v[rchild] {
                v.swap(pos, lchild);
                pos = lchild;
            } else {
                v.swap(pos, rchild);
                pos = rchild;
            }
        }

        return pos;
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_heap() {
        let mut heap = Heap::new();
        heap.push(6);
        heap.push(4);
        heap.push(8);
        heap.push(2);
        heap.push(10);
        heap.push(-1);
        heap.push(100);
        heap.push(3);

        assert_eq!(heap.0, vec![100, 10, 6, 8, 4, -1, 2, 3]);
    }

    #[test]
    fn test_pop() {
        let mut heap = Heap::new();
        heap.push(6);
        heap.push(4);
        heap.push(8);
        heap.push(2);
        heap.push(10);
        heap.push(-1);
        heap.push(100);
        heap.push(3);

        assert_eq!(heap.pop(), Option::Some(100));
        assert_eq!(heap.pop(), Option::Some(10));
        assert_eq!(heap.pop(), Option::Some(8));
        assert_eq!(heap.pop(), Option::Some(6));
        assert_eq!(heap.pop(), Option::Some(4));
        assert_eq!(heap.pop(), Option::Some(3));
        assert_eq!(heap.pop(), Option::Some(2));
        assert_eq!(heap.pop(), Option::Some(-1));
        assert_eq!(heap.pop(), Option::None);
    }

}