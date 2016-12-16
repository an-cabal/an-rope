//! An rope.
//!
//! A rope is an efficient data structure for large mutable strings. It's
//! essentially a binary tree whose leaves are strings.

#[derive(Debug)]
enum Node<T> { /// A leaf node
               Leaf(T)
             , /// A branch node
               Branch(Box<Node<T>>, Box<Node<T>>)
             , None
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
