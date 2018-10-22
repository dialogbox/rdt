#[derive(Debug)]
enum List<'a, T: 'a> {
    Empty,
    Cons(T, &'a List<'a, T>),
}

#[cfg(test)]
mod test {
    use super::List::{self, *};

    #[test]
    fn new_list() {
        let l: List<i32> = Empty;

        println!("{:?}", l);

        let l = Cons(10, &Cons(20, &Empty));

        println!("{:?}", l);

        match l {
            Empty => println!("Empty"),
            Cons(10, t) => println!("Haha! {:?}", t),
            l => println!("{:?}", l),
        }
    }
}
