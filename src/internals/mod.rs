use unicode_segmentation::UnicodeSegmentation;
use unicode_segmentation::{ GraphemeIndices as StrGraphemeIndices
                          , UWordBoundIndices as StrUWordBoundIndices
                          };
use metric::{Metric, Measured};

use std::ops;
use std::fmt;
use std::convert;
use std::borrow::{Borrow, ToOwned};

macro_rules! or_zero {
    ($a: expr, $b: expr) => { if $a > $b { $a - $b } else { 0 } }
}

#[cfg(test)] mod test;

mod node;
pub use self::node::*;

#[cfg(feature = "atomic")]      use std::sync::Arc;
#[cfg(not(feature = "atomic"))] use std::rc::Rc;

#[cfg(feature = "tendril")]
use tendril;
#[cfg(all(feature = "tendril", not(feature = "atomic")))]
use tendril::StrTendril;
#[cfg(all(feature = "tendril", feature = "atomic"))]
use tendril::{Atomic, fmt as tendril_fmt};

use self::node::Value::*;

#[cfg(not(feature = "tendril"))]
type LeafRepr = String;

#[cfg(all(feature = "tendril", not(feature = "atomic") ))]
type LeafRepr = StrTendril;

#[cfg(all(feature = "tendril", feature = "atomic"))]
type LeafRepr = tendril::Tendril<tendril_fmt::UTF8, Atomic>;

#[cfg(not(feature = "atomic"))]
#[derive(Clone)]
pub struct NodeLink(Rc<Node>);

#[cfg(feature = "atomic")]
#[derive(Clone)]
pub struct NodeLink(Arc<Node>);

// impl<T> convert::From<T> for NodeLink
// where Node: convert::From<T> {
//     fn from(that: T) -> Self {
//         NodeLink::new(Node::from(that))
//     }
// }

#[cfg(feature = "tendril")]
impl convert::From<String> for NodeLink {
    #[inline] fn from(string: String) -> Self {
        if string.is_empty() {
            NodeLink::default()
        } else {
            let mut strings = string.rsplit('\n');
            let last = Node::new_leaf(strings.next().unwrap());
            strings.map(|s| {
                        let mut r = LeafRepr::from_slice(s);
                        r.push_char('\n');
                        Node::new_leaf(r)
                    })
                 .fold(last, |r, l| Node::new_branch(l, r))
        }
    }
}
#[cfg(not(feature = "tendril")) ]
impl convert::From<String> for NodeLink {
    #[inline] fn from(string: String) -> Self {
        if string.is_empty() {
            NodeLink::default()
        } else {
            let mut strings = string.rsplit('\n');
            let last = Node::new_leaf(strings.next().unwrap());
            strings.map(|s| Node::new_leaf(LeafRepr::from(s) + "\n"))
                   .fold(last, |r, l| Node::new_branch(l, r))
        }
    }
}

impl<'a, S: ?Sized> convert::From<&'a S> for NodeLink
where String: Borrow<S>
    , S: ToOwned<Owned=String> {

    #[inline] fn from(string: &'a S) -> Self {
        NodeLink::from(string.to_owned())
    }
}


#[cfg(feature = "tendril")]
impl convert::From<LeafRepr> for NodeLink {
    #[inline] fn from(string: LeafRepr) -> Self {
        if string.is_empty() {
            NodeLink::default()
        } else {
            let mut strings = string.rsplit('\n');
            let last = Node::new_leaf(strings.next().unwrap());
            strings.map(|s| {
                    let mut r = LeafRepr::from_slice(s);
                    r.push_char('\n');
                    Node::new_leaf(r)
                })
                  .fold(last, |r, l| Node::new_branch(l, r))
        }
    }
}

impl NodeLink {
    #[cfg(not(feature = "atomic"))]
    pub fn new<N>(node: N) -> Self
    where N: convert::Into<Node> { NodeLink(Rc::new(node.into())) }

    #[cfg(feature = "atomic")]
    pub fn new<N>(node: N) -> Self
    where N: convert::Into<Node> { NodeLink(Arc::new(node.into())) }

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
            // let mut leaves: Vec<Option<Node>> =
            //     self.into_leaves().map(Option::Some).collect();
            // let len = leaves.len();
            // fn _rebalance(l: &mut Vec<Option<Node>>, start: usize, end: usize)
            //               -> Node {
            //     match end - start {
            //         1 => l[start].take().unwrap()
            //       , 2 => l[start].take().unwrap() + l[start + 1].take().unwrap()
            //       , n => {
            //             let mid = start + (n / 2);
            //             _rebalance(l, start, mid) + _rebalance(l, mid, end)
            //
            //         }
            //     }
            // };
            // _rebalance(&mut leaves, 0, len)
            self
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
    #[inline]
    pub fn split<M>(&self, index: M) -> (Self, Self)
    where M: Metric
        , Self: Measured<M> {
        match self.value {
            Leaf(_) if self.is_empty() =>
                // splitting an empty leaf node returns two empty leaf nodes
                (Node::empty(), Node::empty())
          , Leaf(_) if self.measure().into() == 1 =>
                (self.clone(), Node::empty())
          , Leaf(ref s) => {
                // splitting a leaf node with length >= 2 returns two new Leaf
                // nodes, one with the left half of the string, and one with
                // the right
                // TODO: make this properly respect metric index boundaries
                let index = self.to_byte_index(index)
                             .expect(
                                &format!( "split: invalid index! {:?} in {:?}"
                                        , index, s));
                let left = Leaf(s[..index].into());
                let right = Leaf(s[index..].into());
                (NodeLink::new(left), NodeLink::new(right))
            }
          , Branch { ref left, ref right }
            // to determine which side of this node we are splitting on,
            // we compare the index to split to this node's weight.
            if index < self.measure_weight() => {
                // if the index is less than this node's weight, then it's in the
                // left subtree. calling `split` on the left child will walk
                // the left subtree to that index
                let (left, left_right) = left.split(index);
                // the left side of the split left child will become the left side
                // of the split pair.
                let right = if left_right.is_empty() {
                    // if the right side of the split is empty, then the right
                    // side of the returned pair is just this node's right child
                    right.clone()
                } else {
                    // otherwise, the right side of the returned pair is a
                    // branch containing the right side of the split node on
                    // the left, and this node's right child on the right
                    Node::new_branch(left_right, right.clone())
                };
                (left, right)
            }
          , Branch { ref left, ref right } => {
            // otherwise, if the index >= this node's weight, the index is
            // somewhere in the right subtree. walk the right subtree,
            // subtracting this node's weight, (the length of it's
            // left subtree) to find the new index in the right subtree.
                let (right_left, right) =
                    right.split(index - self.measure_weight());
                // the right side of the split right child will become the right
                // side of the split
                let left = if right_left.is_empty() {
                    // if the left side of the split right child is empty, then
                    // the left side of the returned pair is just this node's
                    // left child
                    left.clone()
                } else {
                    // otherwise, the left side of the returned pair is a branch
                    // containing the left side of the split node on the right,
                    // and this node's left child on the left
                    Node::new_branch(left.clone(), right_left)
                };
                (left, right)
            }
        }
    }
}

impl ops::Deref for NodeLink {
    type Target = Node;
    fn deref(&self) -> &Node { self.0.as_ref() }
}

impl fmt::Debug for NodeLink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl fmt::Display for NodeLink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
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
//
// impl fmt::Debug for BranchNode {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}({:?}, {:?})", self.weight, self.left, self.right)
//     }
// }
//
// impl fmt::Display for BranchNode {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}{}", self.left, self.right)
//     }
// }


impl Default for NodeLink {
    fn default() -> Self { Node::empty() }
}

impl<M> Measured<M> for NodeLink
where M: Metric
    , Node: Measured<M> {
        #[inline] fn to_byte_index(&self, index: M) -> Option<usize> {
            self.0.to_byte_index(index)
        }
        #[inline] fn measure(&self) -> M { self.0.measure() }
        #[inline] fn measure_weight(&self) -> M { self.0.measure_weight() }
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

impl Node {

    #[inline]
    pub fn empty() -> NodeLink {
        NodeLink::new(Leaf(LeafRepr::new()))
    }

    /// Concatenate two `Node`s to return a new `Branch` node.
    #[inline]
    pub fn new_branch<A, B>(left: A, right: B) -> NodeLink
    where A: convert::Into<NodeLink>
        , B: convert::Into<NodeLink>
        {
        NodeLink::new(Value::new_branch(left.into(), right.into()))
    }

    #[inline]
    // #[cfg(not(feature = "unstable"))]
    pub fn new_leaf<T>(that: T) -> NodeLink
    where T: convert::Into<LeafRepr> {
        NodeLink::new(Leaf(that.into()))
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
        self.measure()
    }

    #[inline] pub fn is_empty(&self) -> bool { self.len() == 0 }


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
            // let mut leaves: Vec<Option<Node>> =
            //     self.into_leaves().map(Option::Some).collect();
            // let len = leaves.len();
            // #[inline]
            // fn _rebalance(l: &mut Vec<Option<Node>>, start: usize, end: usize)
            //               -> Node {
            //     match end - start {
            //         1 => l[start].take().unwrap()
            //       , 2 => l[start].take().unwrap() + l[start + 1].take().unwrap()
            //       , n => {
            //             let mid = start + (n / 2);
            //             _rebalance(l, start, mid) + _rebalance(l, mid, end)
            //
            //         }
            //     }
            // };
            // _rebalance(&mut leaves, 0, len)
            self
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

    // /// Returns a move iterator over all leaf nodes in this `Node`'s subrope
    // #[inline]
    // fn into_leaves(self) -> IntoLeaves {
    //     IntoLeaves(vec![self])
    // }

    unstable_iters! {
        #[doc=
            "Returns an iterator over all the strings in this `Node`s subrope."]
        #[inline]
        pub fn strings<'a>(&'a self) -> impl Iterator<Item=&'a str> + 'a {
            self.leaves().map(|n| match **n {
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

    // TODO: figure out if we can make move iterators work even with Rcs?
    // /// Returns a move iterator over all the strings in this `Node`s subrope'
    // ///
    // /// Consumes `self`.
    // #[inline]
    // #[cfg(all( feature = "unstable"
    //          , not(feature = "tendril") ))]
    // pub fn into_strings(self) -> impl Iterator<Item=String> {
    //     self.into_leaves().map(|n| match n {
    //         Leaf(s) => s
    //         , _ => unreachable!("Node.into_leaves() iterator contained \
    //                              something  that wasn't a leaf. Barring _force \
    //                              majeure_, this should be impossible. \
    //                              Something's broken.")
    //     })
    // }
    //
    //
    // /// Returns a move iterator over all the strings in this `Node`s subrope'
    // ///
    // /// Consumes `self`.
    // #[inline]
    // #[cfg(all( feature = "unstable"
    //          , feature = "tendril" ))]
    // pub fn into_strings(self) -> impl Iterator<Item=String> {
    //     self.into_leaves().map(|n| match n {
    //         Leaf(s) => s.into()
    //         , _ => unreachable!("Node.into_leaves() iterator contained \
    //                              something  that wasn't a leaf. Barring _force \
    //                              majeure_, this should be impossible. \
    //                              Something's broken.")
    //     })
    // }

    // /// Returns a move iterator over all the strings in this `Node`s subrope'
    // ///
    // /// Consumes `self`.
    // #[inline]
    // #[cfg(all( not(feature = "unstable")
    //          , not(feature = "tendril") ))]
    // pub fn into_strings(self) -> Box<Iterator<Item=String>> {
    //     Box::new(self.into_leaves().map(|n| match n {
    //         Leaf(s) => s
    //         , _ => unreachable!("Node.into_leaves() iterator contained \
    //                              something  that wasn't a leaf. Barring _force \
    //                              majeure_, this should be impossible. \
    //                              Something's broken.")
    //     }))
    // }
    //
    //
    // /// Returns a move iterator over all the strings in this `Node`s subrope'
    // ///
    // /// Consumes `self`.
    // #[inline]
    // #[cfg(all( not(feature = "unstable")
    //          , feature = "tendril" ))]
    // pub fn into_strings(self) -> Box<Iterator<Item=String>> {
    //     Box::new(self.into_leaves().map(|n| match n {
    //         Leaf(s) => s.into()
    //         , _ => unreachable!("Node.into_leaves() iterator contained \
    //                              something  that wasn't a leaf. Barring _force \
    //                              majeure_, this should be impossible. \
    //                              Something's broken.")
    //     }))
    // }

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

        #[inline]
        impl split_whitespace<&'a str> for Node {}
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

}

/// An that performs a left traversal over a series of `Node`s
struct Nodes<'a>(Vec<&'a Node>);

impl<'a> Iterator for Nodes<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.0.pop();
        if let Some(&Branch { ref left, ref right }) =
            node.map(ops::Deref::deref) {
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
              , Some(&Node { value: Leaf(ref s), .. }) if s.is_empty() => {}
              , leaf @ Some(&Node { value: Leaf(_), .. })=> return leaf
              , Some(&Node { value: Branch { ref left, ref right }, .. }) => {
                    self.0.push(right);
                    self.0.push(left);
                }
            }
        }
    }
}

// /// A move iterator over a series of leaf `Node`s
// struct IntoLeaves(Vec<Node>);
//
// impl Iterator for IntoLeaves {
//     type Item = Node;
//     #[inline]
//     fn next(&mut self) -> Option<Self::Item> {
//         loop {
//             match self.0.pop() {
//                 None => return None
//               , Some(Leaf(ref s)) if s.is_empty() => {}
//               , leaf @ Some(Leaf(_))=> return leaf
//               , Some(Branch(BranchNode { left, right, .. })) => {
//                     self.0.push(*right);
//                     self.0.push(*left);
//                 }
//             }
//         }
//     }
// }

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

impl ops::Add for NodeLink {
    type Output = Self;
    /// Concatenate two `Node`s, returning a `Branch` node.
    fn add(self, right: Self) -> Self { Node::new_branch(self, right) }
}

impl<'a> ops::Add for &'a NodeLink {
    type Output = NodeLink;
    /// Concatenate two `Node`s, returning a `Branch` node.
    fn add(self, right: Self) -> Self::Output {
        Node::new_branch(self.clone(), right.clone())
    }
}

impl Measured<usize> for String {
    #[inline] fn to_byte_index(&self, index: usize) -> Option<usize>  {
        Some(index)
    }
    #[inline] fn measure(&self) -> usize { self.len() }
    #[inline] fn measure_weight(&self) -> usize { self.len() }
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

impl IsLineEnding for str {
    #[inline]
    fn is_line_ending(self: &Self) -> bool {
        match self {
            "\u{000A}" => true,
            _ => false
        }
    }
}
