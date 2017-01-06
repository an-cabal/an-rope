use unicode_segmentation::UnicodeSegmentation;
use unicode_segmentation::{ GraphemeIndices as StrGraphemeIndices
                          , UWordBoundIndices as StrUWordBoundIndices
                          };
use metric::{Grapheme, Line, Metric, Measured};
use unicode::GraphemeIndex;

use std::ops;
use std::fmt;
use std::convert;
#[cfg(feature = "tendril")]
use tendril;

use self::Node::*;

#[cfg(not(feature = "tendril"))]
type LeafRepr = String;

#[cfg(feature = "tendril")]
type LeafRepr = tendril::StrTendril;

/// A `Node` in the `Rope`'s tree.
///
/// A `Node` is either a `Leaf` holding a `String`, or a
/// a `Branch` concatenating together two `Node`s.
#[derive(Clone)]
#[derive(Debug)]
pub enum Node {
    /// A leaf node
    Leaf(LeafRepr)
  , /// A branch concatenating together `l`eft and `r`ight nodes.
    Branch(BranchNode)
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.strings()
            .fold(Ok(()), |r, string| r.and_then(|_| write!(f, "{}", string)))
    }
}

// impl fmt::Display for Node {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             Leaf(ref string) => write!(f, "{}", string)
//           , Branch(ref branch) => write!(f, "{}", branch)
//         }
//     }
// }

#[derive(Clone)]
pub struct BranchNode {
    /// The length of this node
    len: usize
  , /// The length of this node in graphemes
    grapheme_len: Grapheme
  , /// The weight of a node is the summed weight of its left subtree
    weight: usize
  , /// The number of started lines of this node
    nlines: Line
  , /// The number of started lines in the node's left subtree
    wlines: Line
  , /// The left branch node
    pub left: Box<Node>
  , /// The right branch node
    pub right: Box<Node>
}


impl fmt::Debug for BranchNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({:?}, {:?})", self.weight, self.left, self.right)
    }
}

impl fmt::Display for BranchNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.left, self.right)
    }
}


impl Default for Node {
    fn default() -> Self { Node::empty() }
}

impl Metric for Grapheme {

    #[inline] fn is_splittable() -> bool { false }

    /// Returns true if index `i` in `node` is a boundary along this `Metric`
    fn is_boundary<M: Measured<Self>>(node: &M, i: usize) -> bool {
        unimplemented!()
    }
}

impl Measured<Grapheme> for str {
    /// Convert the `Metric` into a byte index into the given `Node`
    ///
    /// # Returns
    /// - `Some` with the byte index of the beginning of the `n`th  element
    ///    in `node` measured by this `Metric`, if there is an `n`th element
    /// - `None` if there is no `n`th element in `node`
    fn to_byte_index(&self, index: Grapheme) -> Option<usize>  {
        let i: usize = index.into();
        // TODO: CACHE THIS YOU ASSHOLE
        if self.graphemes(true).count() == i {
            Some(self.len())
        } else {
            self.grapheme_indices(true)
                .position(|(offset, _)| offset == i)
        }
    }


    #[inline]
    fn measure(&self) -> Grapheme {
        Grapheme::from(self.graphemes(true).count())
    }

    #[inline]
    fn measure_weight(&self) -> Grapheme {
        Grapheme::from(self.graphemes(true).count())
    }
}

impl Measured<Grapheme> for String {
    fn to_byte_index(&self, index: Grapheme) -> Option<usize>  {
        let i: usize = index.into();
        // TODO: CACHE THIS YOU ASSHOLE
        if self.graphemes(true).count() == i {
            Some(self.len())
        } else {
            self.grapheme_indices(true)
                .position(|(offset, _)| offset == i)
                // .map(|(offset, _)| offset)
        }
    }

    #[inline]
    fn measure(&self) -> Grapheme {
        Grapheme::from(self.graphemes(true).count())
    }

    #[inline]
    fn measure_weight(&self) -> Grapheme {
        Grapheme::from(self.graphemes(true).count())
    }
}

impl Measured<Grapheme> for BranchNode {

    fn to_byte_index(&self, index: Grapheme) -> Option<usize>  {
        unimplemented!()
    }

    #[inline] fn measure(&self) -> Grapheme {
        self.grapheme_len
    }

    #[inline] fn measure_weight(&self) -> Grapheme { self.left.measure() }
}

impl Measured<Grapheme> for Node {
    fn to_byte_index(&self, index: Grapheme) -> Option<usize>  {
        unimplemented!()
    }

    #[inline] fn measure(&self) -> Grapheme {
        match *self { Leaf(ref s) => s.measure(), Branch(ref n) => n.measure() }
    }

    #[inline] fn measure_weight(&self) -> Grapheme {
        match *self { Leaf(ref s) => s.measure_weight()
                    , Branch(ref n) => n.measure_weight() }
    }

}


impl Metric for Line {

    #[inline] fn is_splittable() -> bool { true }

    /// Returns true if index `i` in `node` is a boundary along this `Metric`
    fn is_boundary<M: Measured<Self>>(node: &M, i: usize) -> bool {
        unimplemented!()
    }
}

impl Measured<Line> for str {
    // This can only handle line endings at the end of a string.
    fn to_byte_index(&self, index: Line) -> Option<usize>  {
        match index.into() {
            0 => Some(self.len())
          , _ => None
        }
    }

    #[inline]
    fn measure(&self) -> Line {
        Line::from(
            if self.chars().last().unwrap_or('\0').is_line_ending() { 1
            } else { 0 })
    }

    #[inline]
    fn measure_weight(&self) -> Line {
        Line::from(
            if self.chars().last().unwrap_or('\0').is_line_ending() { 1
            } else { 0 })
    }
}

impl Measured<Line> for String {
    // This can only handle line endings at the end of a string.
    fn to_byte_index(&self, index: Line) -> Option<usize>  {
        match index.into() {
            0 => Some(self.len())
          , _ => None
        }
    }

    #[inline]
    fn measure(&self) -> Line {
        Line::from(
            if self.chars().last().unwrap_or('\0').is_line_ending() { 1
            } else { 0 })
    }

    #[inline]
    fn measure_weight(&self) -> Line {
        Line::from(
            if self.chars().last().unwrap_or('\0').is_line_ending() { 1
            } else { 0 })
    }
}

impl Measured<Line> for BranchNode {
    fn to_byte_index(&self, index: Line) -> Option<usize>  {
        unimplemented!()
    }

    #[inline] fn measure(&self) -> Line {
        self.nlines
    }

    #[inline] fn measure_weight(&self) -> Line {
        self.wlines
    }
}

impl Measured<Line> for Node {
    fn to_byte_index(&self, index: Line) -> Option<usize>  {
        unimplemented!()
    }

    #[inline] fn measure(&self) -> Line {
        match *self { Leaf(ref s) => s.measure(), Branch(ref n) => n.measure() }
    }

    #[inline] fn measure_weight(&self) -> Line {
        match *self { Leaf(ref s) => s.measure_weight()
                    , Branch(ref n) => n.measure_weight() }
    }

}


#[cfg(feature = "rebalance")]
const FIB_LOOKUP: [usize; 93] = [
 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765, 10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040, 1346269, 2178309, 3524578, 5702887, 9227465, 14930352, 24157817, 39088169, 63245986, 102334155, 165580141, 267914296, 433494437, 701408733, 1134903170, 1836311903, 2971215073, 4807526976, 7778742049, 12586269025, 20365011074, 32951280099, 53316291173, 86267571272, 139583862445, 225851433717, 365435296162, 591286729879, 956722026041, 1548008755920, 2504730781961, 4052739537881, 6557470319842, 10610209857723, 17167680177565, 27777890035288, 44945570212853, 72723460248141, 117669030460994, 190392490709135, 308061521170129, 498454011879264, 806515533049393, 1304969544928657, 2111485077978050, 3416454622906707, 5527939700884757, 8944394323791464, 14472334024676221, 23416728348467685, 37889062373143906, 61305790721611591, 99194853094755497, 160500643816367088, 259695496911122585, 420196140727489673, 679891637638612258, 1100087778366101931, 1779979416004714189, 2880067194370816120, 4660046610375530309, 7540113804746346429 ];


/// Returns the _n_th fibonacci number.
#[inline]
#[cfg(feature = "rebalance")]
fn fibonacci(n: usize) -> usize {
    if n <= 93 { FIB_LOOKUP[n] }
    else { fibonacci(n - 1) + fibonacci(n - 2) }
}

macro_rules! or_zero {
    ($a: expr, $b: expr) => { if $a > $b { $a - $b } else { 0 } }
}

impl BranchNode {

    #[inline]
    fn new(left: Node, right: Node) -> Self {
        let grapheme_left : Grapheme = left.measure();
        let grapheme_right : Grapheme = right.measure();
        let line_left : Line = left.measure();
        let line_right : Line = right.measure();
        BranchNode { len: left.len() + right.len()
                   , grapheme_len: grapheme_left + grapheme_right
                   , weight: left.subtree_weight()
                   , nlines: line_left + line_right
                   , wlines: left.measure()
                   , left: Box::new(left)
                   , right: Box::new(right)
                   }
    }


    /// Split this branch node on the specified `index`.
    ///
    /// This function walks the tree from this node until it reaches the index
    /// to split on, and then it splits the leaf node containing that index.
    ///
    /// # Returns
    /// A tuple containing the left and right sides of the split node. These are
    /// returned as a tuple rather than as a new branch, since the expected use
    /// case for this function is splitting a node so that new text can be
    /// inserted between the two split halves.
    ///
    /// # Time complexity
    /// O(log _n_)
    fn split<M: Metric>(self, index: M) -> (Node, Node)
    where Node: Measured<M>
        , BranchNode: Measured<M>
        , String: Measured<M>
        , M: convert::Into<usize>
        , M: Copy {
        let weight = (&self).measure_weight();
        // to determine which side of this node we are splitting on, we compare
        // the index to split to this node's weight.
        if index < weight {
            // if the index is less than this node's weight, then it's in the
            // left subtree. calling `split` on the left child will walk
            // the left subtree to that index
            let (left, left_right) = self.left.split(index);
            // the left side of the split left child will become the left side
            // of the split pair.
            let right = if left_right.is_empty() {
                // if the right side of the split is empty, then the right
                // side of the returned pair is just this node's right child
                *self.right
            } else {
                // otherwise, the right side of the returned pair is a branch
                // containing the right side of the split node on the left,
                // and this node's right child on the right
                Node::new_branch(left_right, *self.right)
            };
            (left, right)
        } else {
            // otherwise, if the index >= this node's weight, the index is
            // somewhere in the right subtree. walk the right subtree,
            // subtracting this node's weight, (the length of it's left subtree)
            // to find the new index in the right subtree.
            let (right_left, right) = self.right.split(index - weight);
            // the right side of the split right child will become the right
            // side of the split

            let left = if right_left.is_empty() {
                // if the left side of the split right child is empty, then the
                // left side of the returned pair is just this node's left child
                *self.left
            } else {
                // otherwise, the left side of the returned pair is a branch
                // containing the left side of the split node on the right,
                // and this node's left child on the left
                Node::new_branch(*self.left, right_left)
            };
            (left, right)
        }
    }

}

impl Node {

    #[inline] pub fn grapheme_len(&self) -> Grapheme {
        // todo: refactor
        use unicode::Unicode;
        match *self {
            Branch(BranchNode { grapheme_len, ..}) => grapheme_len
          , Leaf(ref s) => Grapheme::from(s.grapheme_len())
        }
    }

    pub fn spanning(&self, i: usize, span_len: usize) -> (&Node, usize) {
        assert!(self.len() >= span_len);
        match *self {
            Branch(BranchNode { ref right, ref left, weight, .. })
            if weight < i => {
                // if this node is a branch, and the weight is less than the
                // index, where the span begins, then the first index of the
                // span is on the right side
                let span_i = or_zero!(i, left.len());
                assert!(or_zero!(right.len(), span_i) >= span_len);
                right.spanning(span_i, span_len)
            }
          , Branch(BranchNode { ref left, .. })
            // if the left child is long enough to contain the entire span,
            // walk to the left child
            if or_zero!(left.len(), i) >= span_len => left.spanning(i, span_len)
         ,  Leaf(_) | Branch (_)=>
            // if this function has walked as far as a leaf node,
            // then that leaf must be the spanning node. return it;
            //
            // otherwise, if the node is a branch node and the span is longer
            // than the left child, then this node must be the minimum
            // spanning node
            (self, i)
        }
    }

    pub fn spanning_mut(&mut self, i: usize, span_len: usize)
                        -> (&mut Node, usize) {
        assert!(self.len() >= span_len);
        match *self {
            Branch(BranchNode { ref mut right, ref left, weight, .. })
            if weight < i => {
                // if this node is a branch, and the weight is less than the
                // index, where the span begins, then the first index of the
                // span is on the right side
                let span_i = or_zero!(i, left.len());
                assert!(or_zero!(right.len(), span_i) >= span_len);
                right.spanning_mut(span_i, span_len)
            }
          , Branch(BranchNode { ref mut left, .. })
            // if the left child is long enough to contain the entire span,
            // walk to the left child
            if or_zero!(left.len(), i) >= span_len =>
                left.spanning_mut(i, span_len)
          , Leaf(_) | Branch (_)=>
            // if this function has walked as far as a leaf node,
            // then that leaf must be the spanning node. return it;
            //
            // otherwise, if the node is a branch node and the span is longer
            // than the left child, then this node must be the minimum
            // spanning node
                (self, i)
        }
    }

    /// Split this `Node`'s subtree on the specified `index`.
    ///
    /// Consumes `self`.
    ///
    /// This function walks the tree from this node until it reaches the index
    /// to split on, and then it splits the leaf node containing that index.
    ///
    /// # Returns
    /// A tuple containing the left and right sides of the split node. These are
    /// returned as a tuple rather than as a new branch, since the expected use
    /// case for this function is splitting a node so that new text can be
    /// inserted between the two split halves.
    ///
    /// # Time complexity
    /// O(log _n_)
    pub fn split<M: Metric>(self, index: M) -> (Node, Node)
    where Self: Measured<M>
        , BranchNode: Measured<M>
        , String: Measured<M>
        , M: convert::Into<usize>
        , M: Copy {
        match self {
            Leaf(ref s) if s.is_empty() =>
                // splitting an empty leaf node returns two empty leaf nodes
                (Node::empty(), Node::empty())
          , Leaf(ref s) if s.measure().into() == 1 =>
                (Leaf(s.clone()), Node::empty())
          , Leaf(s) => {
                // splitting a leaf node with length >= 2 returns two new Leaf
                // nodes, one with the left half of the string, and one with
                // the right
                // TODO: make this properly respect metric index boundaries
                let index = s.to_byte_index(index)
                                 .expect(&format!( "invalid index! {} in {:?}"
                                                  , index.into(), s));
                let left = Leaf(s[..index].into());
                let right = Leaf(s[index..].into());
                (left, right)
            }
          , Branch(node) =>
                // otherwise, just delegate out to `BranchNode::split()`
                node.split(index)
        }
    }

    #[inline]
    pub fn empty() -> Self {
        Leaf("".into())
    }

    /// Concatenate two `Node`s to return a new `Branch` node.
    #[inline]
    pub fn new_branch(left: Self, right: Self) -> Self {
        Branch(BranchNode::new(left, right))
    }


    #[inline]
    #[cfg(feature = "unstable")]
    pub const fn new_leaf(string: LeafRepr) -> Self {
        Leaf(string)
    }

    #[inline]
    #[cfg(not(feature = "unstable"))]
    pub fn new_leaf(string: LeafRepr) -> Self {
        Leaf(string)
    }

    /// Returns true if this node is balanced
    ///
    /// > We define the depth of a leaf to be 0, and the depth of a
    /// > concatenation to be one plus the maximum depth of its children. Let
    /// > _Fn_ be the _n_th Fibonacci number. A rope of depth _n_ is balanced if
    /// > its length is at least _Fn_+2, e.g. a balanced rope of depth 1 must
    /// > have length at least 2. Note that balanced ropes may contain
    /// > unbalanced subropes.
    /// – from "Ropes: An Alternative to Strings"
    #[inline]
    #[cfg(feature = "rebalance")]
    pub fn is_balanced(&self) -> bool {
        self.len() >= fibonacci(self.depth() + 2)
        // true
    }

    /// Returns true if this node is balanced
    ///
    /// > We define the depth of a leaf to be 0, and the depth of a
    /// > concatenation to be one plus the maximum depth of its children. Let
    /// > _Fn_ be the _n_th Fibonacci number. A rope of depth _n_ is balanced if
    /// > its length is at least _Fn_+2, e.g. a balanced rope of depth 1 must
    /// > have length at least 2. Note that balanced ropes may contain
    /// > unbalanced subropes.
    /// – from "Ropes: An Alternative to Strings"
    #[inline]
    #[cfg(not(feature = "rebalance"))]
    pub fn is_balanced(&self) -> bool {
        true
    }

    /// Returns the depth in the tree of a node
    #[inline]
    #[cfg(feature = "rebalance")]
    fn depth(&self) -> usize {
        use std::cmp::max;

        match *self { Leaf(_) => 0
                    , Branch(BranchNode { ref left, ref right, .. }) =>
                        max(left.depth(), right.depth()) + 1
                    }
    }


    /// Returns the length of a node
    #[inline]
    pub fn len(&self) -> usize {
        match *self { Leaf(ref s) => s.len()
                    , Branch(BranchNode { len, ..}) => len
                    }
    }

    #[inline] pub fn is_empty(&self) -> bool { self.len() == 0 }

    /// Calculates the weight of a node
    #[inline]
    fn subtree_weight (&self) -> usize {
        match *self { Leaf(ref s) => s.len()
                    , Branch(BranchNode { ref left, .. }) => left.len()
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
            #[inline]
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

    /// Returns an iterator that performs an in-order traversal over all the
    /// `Nodes` in this `Node`'s subtree
    #[inline]
    fn nodes(&self) -> Nodes {
        Nodes(vec!(self))
    }

    /// Returns an iterator over all leaf nodes in this `Node`'s subrope
    #[inline]
    fn leaves(&self) -> Leaves {
        Leaves(vec![self])
    }

    /// Returns a move iterator over all leaf nodes in this `Node`'s subrope
    #[inline]
    fn into_leaves(self) -> IntoLeaves {
        IntoLeaves(vec![self])
    }

    unstable_iters! {
        #[doc=
            "Returns an iterator over all the strings in this `Node`s subrope."]
        #[inline]
        pub fn strings<'a>(&'a self) -> impl Iterator<Item=&'a str> + 'a {
            self.leaves().map(|n| match *n {
                Leaf(ref s) => s.as_ref()
              , _ => unreachable!("Node.leaves() iterator contained something \
                                   that wasn't a leaf. Barring _force majeure_, \
                                   this should be impossible. Something's broken.")
            })
        }

        #[inline]
        pub fn char_indices<'a>(&'a self)
                               -> impl Iterator<Item=(usize, char)> + 'a {
             self.chars().enumerate()
        }
    }

    /// Returns a move iterator over all the strings in this `Node`s subrope'
    ///
    /// Consumes `self`.
    #[inline]
    #[cfg(all( feature = "unstable"
             , not(feature = "tendril") ))]
    pub fn into_strings(self) -> impl Iterator<Item=String> {
        self.into_leaves().map(|n| match n {
            Leaf(s) => s
            , _ => unreachable!("Node.into_leaves() iterator contained \
                                 something  that wasn't a leaf. Barring _force \
                                 majeure_, this should be impossible. \
                                 Something's broken.")
        })
    }


    /// Returns a move iterator over all the strings in this `Node`s subrope'
    ///
    /// Consumes `self`.
    #[inline]
    #[cfg(all( feature = "unstable"
             , feature = "tendril" ))]
    pub fn into_strings(self) -> impl Iterator<Item=String> {
        self.into_leaves().map(|n| match n {
            Leaf(s) => s.into()
            , _ => unreachable!("Node.into_leaves() iterator contained \
                                 something  that wasn't a leaf. Barring _force \
                                 majeure_, this should be impossible. \
                                 Something's broken.")
        })
    }

    /// Returns a move iterator over all the strings in this `Node`s subrope'
    ///
    /// Consumes `self`.
    #[inline]
    #[cfg(all( not(feature = "unstable")
             , not(feature = "tendril") ))]
    pub fn into_strings(self) -> Box<Iterator<Item=String>> {
        Box::new(self.into_leaves().map(|n| match n {
            Leaf(s) => s
            , _ => unreachable!("Node.into_leaves() iterator contained \
                                 something  that wasn't a leaf. Barring _force \
                                 majeure_, this should be impossible. \
                                 Something's broken.")
        }))
    }


    /// Returns a move iterator over all the strings in this `Node`s subrope'
    ///
    /// Consumes `self`.
    #[inline]
    #[cfg(all( not(feature = "unstable")
             , feature = "tendril" ))]
    pub fn into_strings(self) -> Box<Iterator<Item=String>> {
        Box::new(self.into_leaves().map(|n| match n {
            Leaf(s) => s.into()
            , _ => unreachable!("Node.into_leaves() iterator contained \
                                 something  that wasn't a leaf. Barring _force \
                                 majeure_, this should be impossible. \
                                 Something's broken.")
        }))
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
        // #[inline]
        // impl lines<&'a str> for Node {}
    }

    // /// Returns n iterator over the bytes of this `Node`'s subrope
    // ///
    // ///
    // #[inline]
    // pub fn bytes<'a>(&'a self) -> impl Iterator<Item=u8> + 'a {
    //     self.strings().flat_map(str::bytes)
    // }

    unicode_seg_iters! {
        #[doc=
            "Returns an iterator over the [grapheme clusters][graphemes] of \
             `self`.\n\

             [graphemes]: \
             http://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries\
             \n\
             The iterator is over the  *extended grapheme clusters*; as \
             [UAX#29]\
             (http://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries)\
             recommends extended grapheme cluster boundaries for general \
             processing."]
        #[inline]
        impl graphemes for Node { extend }
    }
    unicode_seg_iters! {
        #[doc=
            "Returns an iterator over the words of `self`, separated on \
            [UAX#29 word boundaries]\
            (http://www.unicode.org/reports/tr29/#Word_Boundaries).\n\n\

            Here, \"words\" are just those substrings which, after splitting on\
            UAX#29 word boundaries, contain any alphanumeric characters. That \
            is, the substring must contain at least one character with the \
            [Alphabetic](http://unicode.org/reports/tr44/#Alphabetic) \
            property, or with [General_Category=Number]\
            (http://unicode.org/reports/tr44/#General_Category_Values)."]
        #[inline]
        impl unicode_words for Node {}
        #[doc=
            "Returns an iterator over substrings of `self` separated on \
            [UAX#29 word boundaries]\
            (http://www.unicode.org/reports/tr29/#Word_Boundaries). \n\n\
            The concatenation of the substrings returned by this function is \
            just the original string."]
        #[inline]
        impl split_word_bounds for Node {}
    }

    pub fn grapheme_indices(&self) -> GraphemeIndices {
        let mut strings = self.strings();
        let first_string = strings.next()
            .expect("grapheme_indices called on empty rope!");
        GraphemeIndices { strings: Box::new(strings)
                        , graphemes: first_string.grapheme_indices(true)
                        , char_length_so_far: 0
                        , curr_length: first_string.len() }
    }

    pub fn split_word_bound_indices(&self) -> UWordBoundIndices {
        let mut strings = self.strings();
        let first_string = strings.next()
            .expect("split_word_bound_indices called on empty rope!");
        UWordBoundIndices { strings: Box::new(strings)
                          , bounds: first_string.split_word_bound_indices()
                          , char_length_so_far: 0
                          , curr_length: first_string.len() }
    }

    // #[cfg(not(feature = "unstable"))]
    // #[inline]
    // pub fn grapheme_indices<'a>(&'a self)
    //                         -> Box<Iterator<Item=(usize, &'a str)> + 'a> {
    //     let strings = self.strings();
    //     let first_graphemes = strings.next()
    //                                  .unwrap_or_else(Iterator::empty());
    //     Box::new(GraphemeIndices { strings: strings
    //                              , graphemes: first_graphemes
    //                              , char_length_so_far: 0})
    // }
    //
    // #[cfg(feature = "unstable")]
    // #[inline]
    // pub fn split_word_bound_indices<'a>(&'a self)
    //                        -> impl Iterator<Item=(usize, &'a str)> + 'a {
    //     let s: String = self.strings().collect();
    //     // TODO: rewrite this to not collect into a  string
    //     s.split_word_bound_indices()
    // }
    //
    // #[cfg(not(feature = "unstable"))]
    // #[inline]
    // pub fn split_word_bound_indices<'a>(&'a self)
    //                         -> Box<Iterator<Item=(usize, &'a str)> + 'a> {
    //     let s: String = self.strings().collect();
    //     // TODO: rewrite this to not collect into a  string
    //     Box::new(s.split_word_bound_indices())
    // }

}

/// An that performs a left traversal over a series of `Node`s
struct Nodes<'a>(Vec<&'a Node>);

impl<'a> Iterator for Nodes<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.0.pop();
        if let Some(&Branch(BranchNode { ref left, ref right, ..})) = node {
            self.0.push(right);
            self.0.push(left);
        };
        node
    }
}

/// An iterator over a series of leaf `Node`s
// TODO: this _could_ be implemented as `nodes.filter(node.is_leaf)`
struct Leaves<'a>(Vec<&'a Node>);

impl<'a> Iterator for Leaves<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.pop() {
                None => return None
              , Some(&Leaf(ref s)) if s.is_empty() => {}
              , leaf @ Some(&Leaf(_))=> return leaf
              , Some(&Branch(BranchNode { ref left, ref right, .. })) => {
                    self.0.push(right);
                    self.0.push(left);
                }
            }
        }
    }
}

/// A move iterator over a series of leaf `Node`s
struct IntoLeaves(Vec<Node>);

impl Iterator for IntoLeaves {
    type Item = Node;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.pop() {
                None => return None
              , Some(Leaf(ref s)) if s.is_empty() => {}
              , leaf @ Some(Leaf(_))=> return leaf
              , Some(Branch(BranchNode { left, right, .. })) => {
                    self.0.push(*right);
                    self.0.push(*left);
                }
            }
        }
    }
}

pub struct GraphemeIndices<'a> {
    strings: Box<Iterator<Item = &'a str> + 'a >
  , graphemes: StrGraphemeIndices<'a>
  , char_length_so_far: usize
  , curr_length: usize
}

impl<'a> Iterator for GraphemeIndices<'a> {
    type Item = (usize, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        self.graphemes.next()
            .map(|(i, s)| (i + self.char_length_so_far, s))
            .or_else(|| {
                self.strings.next()
                    .and_then(|s| { self.char_length_so_far += self.curr_length;
                                    self.curr_length = s.len();
                                    self.graphemes = s.grapheme_indices(true);
                                    self.next() })
            })
    }
}

pub struct UWordBoundIndices<'a> {
    strings: Box<Iterator<Item = &'a str> + 'a >
  , bounds: StrUWordBoundIndices<'a>
  , char_length_so_far: usize
  , curr_length: usize
}

impl<'a> Iterator for UWordBoundIndices<'a> {
    type Item = (usize, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        self.bounds.next()
            .map(|(i, s)| (i + self.char_length_so_far, s))
            .or_else(|| {
                self.strings.next()
                    .and_then(|s| { self.char_length_so_far += self.curr_length;
                                    self.curr_length = s.len();
                                    self.bounds = s.split_word_bound_indices();
                                    self.next() })
            })
    }
}

impl ops::Add for Node {
    type Output = Self;
    /// Concatenate two `Node`s, returning a `Branch` node.
    fn add(self, right: Self) -> Self { Node::new_branch(self, right) }
}


impl ops::AddAssign for Node {
    /// Concatenate two `Node`s
    fn add_assign(&mut self, right: Self) {
        use std::mem::replace;
        *self = Node::new_branch(replace(self, Node::empty()), right)
     }

}


impl ops::Index<usize> for Node {
    type Output = str;

    fn index(&self, i: usize) -> &str {
        let grapheme_len : Grapheme = self.measure();
        let len = grapheme_len.into();
        assert!( i < len
               , "Node::index: index {} out of bounds (length {})", i, len);
        match *self {
            Leaf(ref string) => {
                let index: usize =
                    GraphemeIndex::from(i).to_char_index(string).into();
                string.graphemes(true).nth(index - 1).expect("oob!") }
          , Branch(BranchNode { ref right, .. }) if len < i =>
                &right[i - len]
          , Branch(BranchNode { ref left, .. }) => &left[i]
        }
    }
}


pub trait IsLineEnding { fn is_line_ending(&self) -> bool; }

impl IsLineEnding for char {
    #[inline]
    fn is_line_ending(self: &char) -> bool {
        match *self {
            '\u{000A}' => true,
            _ => false
        }
    }
}
