use std::cmp::Ord;
#[derive(Debug)]
struct Tree<T> {
    value: T,
    left: Option<Box<Tree<T>>>,
    right: Option<Box<Tree<T>>>,
}

impl<T: Ord> Tree<T> {
    fn new(value: T) -> Tree<T> {
        Tree {
            value: value,
            left: None,
            right: None,
        }
    }
    fn append(&mut self, value: T) {
        if value < self.value {
            self.left = Some(Box::new(Tree::new(value)));
        } else {
            self.right = Some(Box::new(Tree::new(value)));
        }
    }
}

fn main() {
    let mut t = Tree::new(5);
    t.append(10);
    t.append(3);
    println!("{:?}", t);
}