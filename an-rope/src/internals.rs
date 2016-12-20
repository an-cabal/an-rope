use std::ops;
use std::mem;

use self::Node::*;

/// A `Node` in the `Rope`'s tree.
///
/// A `Node` is either a `Leaf` holding a `String`, or a
/// a `Branch` concatenating together two `Node`s.
#[derive(Debug, Clone)]
pub enum Node {
    /// A leaf node
    Leaf(String)
  , /// A branch concatenating together `l`eft and `r`ight nodes.
    Branch(BranchNode)
}

#[derive(Debug, Clone)]
pub struct BranchNode {
    /// The length of this node
    len: usize
  , /// The weight of a node is the summed weight of its left subtree
    weight: usize
  , /// The left branch node
    pub left: Box<Node>
  , /// The right branch node
    pub right: Box<Node>
}


impl Default for Node {
    fn default() -> Self { Node::empty() }
}

/// Returns the _n_th fibonacci number.
// TODO: replace with an iterative implementation and/or lookup table?
fn fibonacci(n: usize) -> usize {
    match n { 0 => 1
            , 1 => 1
            , _ => fibonacci(n - 1) + fibonacci(n - 2)
            }
}

impl BranchNode {

    #[inline]
    fn new(left: Node, right: Node) -> Self {
        BranchNode { len: left.len() + right.len()
                   , weight: left.subtree_weight()
                   , left: box left
                   , right: box right
                   }
    }

    #[inline]
    fn take_left(&mut self) -> Node {
        mem::replace(self.left.as_mut(), Node::empty())
    }

    #[inline]
    fn take_right(&mut self) -> Node {
        mem::replace(self.right.as_mut(), Node::empty())
    }

    fn split(&mut self, index: usize) -> &mut Node {
        if index < self.weight {
            // split the left node
            let mut left = self.take_left();
            left.split(index);
            // replacing *self with a new BranchNode will update the
            // node's weight automagically
            *self = BranchNode::new(left, self.take_right());
            self.left.as_mut()
        } else {
            // split the right node
            let mut right = self.take_right();
            right.split(index);
            // replacing *self with a new BranchNode will update the
            // node's weight automagically
            *self = BranchNode::new(self.take_left(), right);
            self.left.as_mut()
        }
    }

}

impl Node {
    fn split_leaf(&mut self, index: usize) -> bool {
        // we mutably borrow `self` here. This precludes us from changing it
        // directly as in `*self = ...`, because the borrow checker won't allow
        // it. Therefore, the assignment to `self` must be outside the `if let`
        // clause.
        *self = if let Leaf(ref mut s) = *self {

            // if this node is a Leaf, take the String out of it
            // (note that empty strings don't allocate).
            let string = mem::replace(s, String::new());
            // split the string into left and right parts...
            let left = Leaf(string[index..].to_string());
            let right = Leaf(string[..index].to_string());
            // construct the new Branch node that will be reassigned to `self`
            Node::new_branch(left, right)
        } else {
            // if this node is not a Leaf, we return immediately, thus skipping
            // the assignment
            return false
        };
        return true
    }

    pub fn split(&mut self, index: usize) -> &mut Node {
        if self.split_leaf(index) == true {
            self
        } else {
            if let &mut Branch(ref mut node) = self {
                node.split(index)
            } else {
                unreachable!()
            }
        }
    }

    pub fn empty() -> Self {
        Leaf(String::new())
    }

    /// Concatenate two `Node`s to return a new `Branch` node.
    #[inline]
    pub fn new_branch(left: Self, right: Self) -> Self {
        Branch(BranchNode::new(left, right))
    }

    #[inline]
    pub fn concat(&mut self, right: Self) {
        *self = Node::new_branch(mem::replace(self, Node::empty()), right)
    }

    #[inline]
    pub const fn new_leaf(string: String) -> Self {
        Leaf(string)
    }

    /// Returns true if this node is balanced
    ///
    /// "We define the depth of a leaf to be 0, and the depth of a
    /// concatenation to be one plus the maximum depth of its children. Let
    /// _Fn_ be the _n_th Fibonacci number. A rope of depth _n_ is balanced if
    /// its length is at least _Fn_+2, e.g. a balanced rope of depth 1 must
    /// have length at least 2. Note that balanced ropes may contain unbalanced
    /// subropes."
    /// – from "Ropes: An Alternative to Strings"
    #[inline]
    pub fn is_balanced(&self) -> bool {
        self.len() >= fibonacci(self.depth() + 2)
    }


    /// Returns the depth in the tree of a node
    #[inline]
    fn depth(&self) -> usize {
        use std::cmp::max;

        match self {
            &Node::Leaf(_) => 0
          , &Node::Branch(BranchNode { ref left, ref right, .. }) =>
                max(left.depth(), right.depth()) + 1
            }
    }


    /// Returns the length of a node
    //  TODO: do we want to cache this?
    pub fn len(&self) -> usize {
        match self { &Leaf(ref s) => s.len()
                   , &Branch(BranchNode { ref left, ref right, .. }) =>
                        left.len() + right.len()
                    }
    }

    /// Calculates the weight of a node
    #[inline]
    fn subtree_weight (&self) -> usize {
        match self { &Leaf(ref s) => s.len()
                   , &Branch(BranchNode { ref left, .. }) =>
                        left.subtree_weight()
                    }
    }


    /// Rebalance the subrope starting at this `Node`, returning a new `Node`
    ///
    /// From "Ropes: An Alternative to Strings":
    /// > "The rebalancing operation maintains an ordered sequence of (empty
    /// > or) balanced ropes, one for each length interval [_Fn_, _Fn_+1), for
    /// > _n_ >= 2. We traverse the rope from left to right, inserting each
    /// > leaf into the appropriate sequence position, depending on its length.
    ///
    /// > The concatenation of the sequence of ropes in order of decreasing
    /// > length is equivalent to the prefix of the rope we have traversed so
    /// > far. Each new leaf _x_ is with_insert_ropeed into the appropriate entry of the
    /// > sequence. Assume that _x_’s length is in the interval [_Fn_, _Fn_+1),
    /// > and thus it should be put in slot _n_ (which also corresponds to
    /// > maximum depth _n_ − 2). If all lower and equal numbered levels are
    /// > empty, then this can be done directly. If not, then we concatenate
    /// > ropes in slots 2, ... ,(_n_ − 1) (concatenating onto the left), and
    /// > concatenate _x_ to the right of the result. We then continue to
    /// > concatenate ropes from the sequence in increasing order to the left
    /// > of this result, until the result fits into an empty slot in the
    /// > sequence."
    pub fn rebalance(self) -> Self {
        // TODO: this is a huge mess, I based it on the IBM Java implementation
        //       please refactor until it stops being ugly!
        //        - eliza, 12/17/2016

        if self.is_balanced() {
            // the subrope is already balanced, do nothing
            self
        } else {
            let mut leaves: Vec<Option<Node>> =
                self.into_leaves().map(Option::Some).collect();
            let len = leaves.len();
            fn _rebalance(l: &mut Vec<Option<Node>>, start: usize, end: usize)
                          -> Node {
                match end - start {
                    1 => l[start].take().unwrap()
                  , 2 => l[start].take().unwrap() + l[start + 1].take().unwrap()
                  , n => {
                        let mid = start + (n / 2);
                        _rebalance(l, start, mid) + _rebalance(l, mid, end)

                    }
                }
            };
            _rebalance(&mut leaves, 0, len)
        }
    }

    /// Returns an iterator over all leaf nodes in this `Node`'s subrope
    fn leaves<'a>(&'a self) -> Leaves<'a> {
        Leaves(vec![self])
    }

    /// Returns a move iterator over all leaf nodes in this `Node`'s subrope
    fn into_leaves(self) -> IntoLeaves {
        IntoLeaves(vec![self])
    }


    /// Returns an iterator over all the strings in this `Node`s subrope'
    #[inline]
    pub fn strings<'a>(&'a self) -> impl Iterator<Item=&'a str> {
        box self.leaves().map(|n| match n {
            &Leaf(ref s) => s.as_ref()
          , _ => unreachable!("Node.leaves() iterator contained something \
                               that wasn't a leaf. Barring _force majeure_, \
                               this should be impossible. Something's broken.")
        })
    }

    /// Returns a move iterator over all the strings in this `Node`s subrope'
    ///
    /// Consumes `self`.
    #[inline]
    pub fn into_strings(self) -> impl Iterator<Item=String> {
        box self.into_leaves().map(|n| match n {
            Leaf(s) => s
            , _ => unreachable!("Node.into_leaves() iterator contained \
                                 something  that wasn't a leaf. Barring _force \
                                 majeure_, this should be impossible. \
                                 Something's broken.")
        })
    }

    str_iters! {
        #[doc="Returns an iterator over all the bytes in this `Node`'s \
               subrope \n\

               \nAs a Rope consists of a sequence of bytes, we can iterate \
               through a rope by byte. This method returns such an iterator."]
        #[inline]
        impl bytes<u8> for Node {}
        #[doc="Returns an iterator over all the characters in this `Node`'s \
               subrope \n\

               \nAs a `Rope` consists of valid UTF-8, we can iterate through a \
               `Rope` by `char`. This method returns such an iterator. \n\

               \nIt's important to remember that `char` represents a Unicode \
               Scalar Value, and may not match your idea of what a \
               'character' is. Iteration over grapheme clusters may be what \
               you actually want."]
        #[inline]
        impl chars<char> for Node {}
        // TODO: this is actually Wrong, the indices will wrap around once we
        //       iterate into the next leaf node. we'll need to write our own
        //       char_indices iterator that tracks the character's index in the
        //       global Rope. shouldn't be too hard, just a fold...
        //          - eliza, 12/18/2016
        // #[inline]
        // impl char_indices<(usize, char)> for Node {}
        #[inline]
        impl split_whitespace<&'a str> for Node {}
        #[inline]
        impl lines<&'a str> for Node {}
    }

    // /// Returns n iterator over the bytes of this `Node`'s subrope
    // ///
    // ///
    // #[inline]
    // pub fn bytes<'a>(&'a self) -> impl Iterator<Item=u8> + 'a {
    //     self.strings().flat_map(str::bytes)
    // }

    /// Returns an iterator over the grapheme clusters of this `Node`'s subrope'
    ///
    /// This is the iterator returned by `Node::into_iter`.
    #[inline]
    pub fn graphemes<'a>(&'a self) -> impl Iterator<Item=&'a str> {
        // the compiler won't let me mark this as unimplemented using the
        // unimplemented!() macro, due to Reasons (i suspect relating to
        // returning `impl Trait`)
        //  - eliza, 12/18/2016
        panic!("Unimplemented!");
        self.strings()
    }

}

/// An iterator over a series of leaf `Node`s
struct Leaves<'a>(Vec<&'a Node>);

impl<'a> Iterator for Leaves<'a> {
    type Item = &'a Node;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.pop() {
            None => None
          , Some(leaf @ &Leaf(_)) => if leaf.len() == 0 { None }
                                     else { Some(leaf) }
          , Some(&Branch(BranchNode { ref left, ref right, .. })) => {
                self.0.push(left);
                self.0.push(right);
                self.next()
            }
        }
    }
}

/// A move iterator over a series of leaf `Node`s
struct IntoLeaves(Vec<Node>);

impl Iterator for IntoLeaves {
    type Item = Node;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.pop() {
            None => None
          , Some(leaf @ Leaf(_)) => if leaf.len() == 0 { None }
                                    else { Some(leaf) }
          , Some(Branch(BranchNode { box left, box right, .. })) => {
                self.0.push(right);
                self.0.push(left);
                self.next()
            }
        }
    }
}


impl ops::Add for Node {
    type Output = Self;
    /// Concatenate two `Node`s, returning a `Branch` node.
    fn add(self, right: Self) -> Self { Node::new_branch(self, right) }
}
