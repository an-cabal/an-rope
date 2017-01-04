use super::Rope;
use std::iter;
use internals::Node;
use internals::Node::Leaf;
use metric::Line;
use metric::Measured;

use quickcheck::{Arbitrary, Gen};

impl Arbitrary for Rope {
    fn arbitrary<G: Gen>(g: &mut G) -> Rope {
        Rope::from(String::arbitrary(g))
    }

    fn shrink(&self) -> Box<Iterator<Item=Rope>> {
        // Shrink a string by shrinking a vector of its characters.
        let chars: Vec<char> = self.chars().collect();
        Box::new(chars.shrink().map(|x| x.into_iter().collect::<Rope>()))
    }

}


#[test]
fn line_split_test_1() {
    let l1 = Node::new_leaf("asdf".to_string());
    let l2 = Node::new_leaf("qwer".to_string());
    let b = Node::new_branch(l1, l2);
    let (left, right) = b.split(Line::from(0));
    assert_eq!(left.strings().collect::<String>(), "asdfqwer");
    if let Leaf(s) = right {
        assert_eq!(s, "");
    } else { assert!(false) }
}

#[test]
fn line_split_test_2() {
    let l1 = Node::new_leaf("asdf".to_string());
    let l2 = Node::new_leaf("qwer\n".to_string());
    let b = Node::new_branch(l1, l2);
    let (left, right) = b.split(Line::from(0));
    assert_eq!(left.strings().collect::<String>(), "asdfqwer\n");
    if let Leaf(s) = right {
        assert_eq!(s, "");
    } else { assert!(false) }
}

#[test]
fn line_split_test_3() {
    let l1 = Node::new_leaf("asdf\n".to_string());
    let l2 = Node::new_leaf("qwer\n".to_string());
    let b = Node::new_branch(l1, l2);
    let (left, right) = b.split(Line::from(0));
    if let Leaf(s) = left {
        assert_eq!(s, "asdf\n");
    } else { assert!(false) }
    if let Leaf(s) = right {
        assert_eq!(s, "qwer\n");
    } else { assert!(false) }
}

#[test]
#[should_panic(expected = "invalid index!")]
fn line_split_test_4() {
    let l1 = Node::new_leaf("asdf".to_string());
    let l2 = Node::new_leaf("qwer".to_string());
    let b = Node::new_branch(l1, l2);
    let (left, right) = b.split(Line::from(1));
}

#[test]
fn line_split_test_5() {
    let l1 = Node::new_leaf("asdf".to_string());
    let l2 = Node::new_leaf("qwer\n".to_string());
    let b = Node::new_branch(l1, l2);
    let (left, right) = b.split(Line::from(1));
    assert_eq!(left.strings().collect::<String>(), "asdfqwer\n");
    if let Leaf(s) = right {
        assert_eq!(s, "");
    } else { assert!(false) }
}

#[test]
fn line_split_test_6() {
    let l1 = Node::new_leaf("asdf\n".to_string());
    let l2 = Node::new_leaf("qwer\n".to_string());
    let b = Node::new_branch(l1, l2);
    let (left, right) = b.split(Line::from(1));
    assert_eq!(left.strings().collect::<String>(), "asdf\nqwer\n");
    if let Leaf(s) = right {
        assert_eq!(s, "");
    } else { assert!(false) }
}

#[test]
fn line_split_test_7() {
    let l1 = Node::new_leaf("asdf\n".to_string());
    let l2 = Node::new_leaf("qwer\n".to_string());
    let b = Node::new_branch(l1, l2);
    let (left, right) = b.split(Line::from(0));
    if let Leaf(s) = left {
        assert_eq!(s, "asdf\n");
    } else { assert!(false) }
    if let Leaf(s) = right {
        assert_eq!(s, "qwer\n");
    } else { assert!(false) }
}

#[test]
fn line_split_test_8() {
    let l1 = Node::new_leaf("".to_string());
    let l2 = Node::new_leaf("qwer\n".to_string());
    let b = Node::new_branch(l1, l2);
    let (left, right) = b.split(Line::from(0));
    assert_eq!(left.strings().collect::<String>(), "qwer\n");
    if let Leaf(s) = right {
        assert_eq!(s, "");
    } else { assert!(false) }
}

#[test]
fn line_split_test_9() {
    let l1 = Node::new_leaf("asdf\n".to_string());
    let l2 = Node::new_leaf("qwer".to_string());
    let l3 = Node::new_leaf("yxcv\n".to_string());
    let b1 = Node::new_branch(l1, l2);
    let b2 = Node::new_branch(b1, l3);
    let (left, right) = b2.split(Line::from(0));
    if let Leaf(s) = left {
        assert_eq!(s, "asdf\n");
    } else { assert!(false) }
    assert_eq!(right.strings().collect::<String>(), "qweryxcv\n");
}

#[test]
fn line_split_test_10() {
    let l1 = Node::new_leaf("asdf".to_string());
    let l2 = Node::new_leaf("qwer\n".to_string());
    let l3 = Node::new_leaf("yxcv\n".to_string());
    let b1 = Node::new_branch(l2, l3);
    let b2 = Node::new_branch(l1, b1);
    let (left, right) = b2.split(Line::from(0));
    assert_eq!(left.strings().collect::<String>(), "asdfqwer\n");
    if let Leaf(s) = right {
        assert_eq!(s, "yxcv\n");
    } else { assert!(false) }
}


#[test]
fn delete_test_1() {
    let mut r = Rope::from("this is not fine".to_string());
    r.delete((8..12));
    assert_eq!(&r, "this is fine");
}

#[test]
fn delete_test_2() {
    let mut r = Rope::new();
    r.delete((0..0));
    assert_eq!(&r, "");
}

// this range syntax only works on nightly rust
#[cfg(feature = "unstable")]
#[test]
fn delete_test_3() {
    let mut r = Rope::from("this is not fine".to_string());
    r.delete((..));
    assert_eq!(&r, "");
}

// this range syntax only works on nightly rust
#[cfg(feature = "unstable")]
#[test]
fn delete_test_4() {
    let mut r = Rope::from("this is not fine".to_string());
    r.delete((11..));
    assert_eq!(&r, "this is not");
}

// this range syntax only works on nightly rust
#[cfg(feature = "unstable")]
#[test]
fn delete_test_5() {
    let mut r = Rope::from("this is not fine".to_string());
    r.delete((..5));
    assert_eq!(&r, "is not fine");
}

// this range syntax only works on nightly rust
#[cfg(feature = "unstable")]
#[test]
#[should_panic(expected = "do not lie on character boundary")]
fn delete_test_6() {
    let mut r = Rope::from("this is not fine".to_string());
    r.delete((..42));
}

#[test]
#[should_panic(expected = "attempt to subtract with overflow")]
fn delete_test_7() {
    let mut r = Rope::from("this is not fine".to_string());
    r.delete((12..8)); // lol, fuck you
}

#[test]
fn fmt_debug_test_1() {
    let s = format!("{:?}", Rope::new());
    assert_eq!(s, "Rope[\"\"] Leaf(\"\")");
}

#[test]
fn fmt_debug_test_2() {
    let s = format!("{:?}", Rope::from("NERD!!!".to_string()));
    assert_eq!(s, "Rope[\"NERD!!!\"] Leaf(\"NERD!!!\")");
}

#[test]
fn fmt_debug_test_3() {
    let r1 = Rope::from("Hello, ".to_string());
    let r2 = Rope::from("World!".to_string());
    let r = r1 + r2;
    let s = format!("{:?}", r);
    assert_eq!(s, "Rope[\"Hello, World!\"] \
                        Branch(7(Leaf(\"Hello, \"), Leaf(\"World!\")))");
}

#[test]
fn rebalance_test_1() {
    let mut r = Rope::from("This is a large string \
                        that will need to be rebalanced.".to_string());
    r.rebalance();
    assert!(r.is_balanced());
}

#[test]
fn rebalance_test_2() {
    let mut r =
        Rope::from("Lorem ipsum dolor sit amet, consectetur adipiscing eli\
                    t, sed do eiusmod tempor incididunt ut labore et dolor\
                    e magna aliqua. Ut enim ad minim veniam, quis nostrud \
                    exercitation ullamco laboris nisi ut aliquip ex ea com\
                    modo consequat. Duis aute irure dolor in reprehenderit\
                     in voluptate velit esse cillum dolore eu fugiat nulla\
                     pariatur. Excepteur sint occaecat cupidatat non proid\
                    ent, sunt in culpa qui officia deserunt mollit anim id\
                     est laborum.".to_string());
    r.rebalance();
    assert!(r.is_balanced());
}

#[test]
fn big_rebalance() {
    let s: String = iter::repeat('a').take(10_000).collect();
    let mut r = Rope::from(s);
    r.rebalance();
    assert!(r.is_balanced());
}

#[test]
fn repeated_concat_left_rebalance() {
    let s: String = iter::repeat('a').take(10_000).collect();
    let mut r = Rope::from(s);
    for _ in 1..1000 {
        r = r + iter::repeat('a').take(100).collect::<String>();
    }
    assert!(r.is_balanced());
}

#[test]
fn repeated_concat_right_rebalance() {
    let s: String = iter::repeat('a').take(10_000).collect();
    let mut r = Rope::from(s);
    for _ in 1..1000 {
        let s2 = iter::repeat('a').take(100).collect::<String>();
        r = Rope::from(s2) + r;
    }
    assert!(r.is_balanced());
}

#[test]
fn append_empty_rope() {
    let mut an_rope = Rope::from("");
    an_rope.append(Rope::from("abcd"));
    assert_eq!(&an_rope, "abcd");

    let mut an_rope = Rope::from("abcd");
    an_rope.append(Rope::from(""));
    assert_eq!(&an_rope, "abcd");
}

#[test]
fn append() {
    let mut an_rope = Rope::from(String::from("abcd"));
    an_rope.append(Rope::from(String::from("efgh")));
    assert_eq!(an_rope, Rope::from(String::from("abcdefgh")) );
}

#[test]
fn with_append_empty_rope() {
    let an_rope = Rope::from("");
    let another_rope = an_rope.with_append(Rope::from("abcd"));
    assert_eq!(&another_rope, "abcd");
    assert_eq!(&an_rope, "");

    let an_rope = Rope::from("abcd");
    let an_rope = an_rope.with_append(Rope::from(""));
    assert_eq!(&an_rope, "abcd");
}

#[test]
fn with_append() {
    let an_rope = Rope::from("abcd");
    let another_rope = an_rope.with_append(Rope::from("efgh"));
    assert_eq!(&another_rope, "abcdefgh");
    assert_eq!(&an_rope, "abcd");
}

#[test]
fn prepend_empty_rope() {
    let mut an_rope = Rope::from("");
    an_rope.prepend(Rope::from("abcd"));
    assert_eq!(&an_rope, "abcd");


    let mut an_rope = Rope::from("abcd");
    an_rope.prepend(Rope::from(""));
    assert_eq!(&an_rope, "abcd");
}


#[test]
fn with_prepend_empty_rope() {
    let an_rope = Rope::from("");
    let another_rope = an_rope.with_prepend(Rope::from("abcd"));
    assert_eq!(&an_rope, "");
    assert_eq!(&another_rope, "abcd");

    let an_rope = Rope::from("abcd");
    let another_rope = an_rope.with_prepend(Rope::from(""));
    assert_eq!(&an_rope, "abcd");
    assert_eq!(&another_rope, &an_rope);
    assert_eq!(&another_rope, "abcd");
}

#[test]
fn with_prepend() {
    let an_rope = Rope::from("efgh");
    let another_rope = an_rope.with_prepend(Rope::from("abcd"));
    assert_eq!(&an_rope, "efgh");
    assert_eq!(&another_rope, "abcdefgh");
}

#[test]
fn prepend() {
    let mut an_rope = Rope::from(String::from("efgh"));
    an_rope.prepend(Rope::from(String::from("abcd")));
    assert_eq!(&an_rope, "abcdefgh");
}

#[test]
fn merge_rebalance_test() {
    let s = "Lorem ipsum dolor sit amet, consectetur adipiscing eli\
             t, sed do eiusmod tempor incididunt ut labore et dolor\
             e magna aliqua. Ut enim ad minim veniam, quis nostrud \
             exercitation ullamco laboris nisi ut aliquip ex ea com\
             modo consequat. Duis aute irure dolor in reprehenderit\
              in voluptate velit esse cillum dolore eu fugiat nulla\
              pariatur. Excepteur sint occaecat cupidatat non proid\
             ent, sunt in culpa qui officia deserunt mollit anim id\
              est laborum.";

     let t = Rope::from(s.to_owned());
     let mut u = t + s;
     u.rebalance();
     assert!(u.is_balanced());
}

#[test]
fn with_insert_rope_balance_test() {
    let s: String = iter::repeat('a').take(10).collect();
    let mut r_1 = Rope::from(s);
    for _ in 0..99 {
        let t: String = iter::repeat('a').take(10).collect();
        r_1 = r_1.with_insert_rope(5, Rope::from(t));
    }
    //  this isn't necessary, with_insert_rope() will automatically rebalance
    //      - eliza, 12/18/2016
    // r_1.rebalance();
    assert!(r_1.is_balanced());
}

#[test]
fn with_insert_rope_test_1() {
    let s: String = iter::repeat('a').take(1_000).collect();
    let r_1 = Rope::from(s);
    let mut r_2 = Rope::new();
    for _ in 0..100 {
        let t: String = iter::repeat('a').take(10).collect();
        r_2 = r_2.with_insert_rope(0, Rope::from(t));
    }
    assert_eq!(r_1, r_2);
}

#[test]
fn with_insert_rope_test_2() {
    let s: String = iter::repeat('a').take(10).collect();
    let mut r_1 = Rope::from(s);
    for _ in 0..99 {
        let t: String = iter::repeat('a').take(10).collect();
        r_1 = r_1.with_insert_rope(5, Rope::from(t));
    }

    let q: String = iter::repeat('a').take(1_000).collect();
    let r_2 = Rope::from(q);
    assert_eq!(r_1, r_2);
}

#[test]
fn mutable_insert_rope_test_1() {
    let mut s_1 = Rope::from(String::from("aaaaa"));
    let mut s_2 = Rope::from(String::from("bbbbb"));
    let s_3 = Rope::from(String::from("ccccc"));
    s_2.insert_rope(0, s_3);
    s_1.insert_rope(0, s_2);
    assert_eq!(&s_1, "cccccbbbbbaaaaa");
}

#[test]
fn mutable_insert_str_test_1() {
    let mut s = Rope::from("aaaaa");
    s.insert_str(0, "bbbbb");
    s.insert_str(10, "ccccc");
    assert_eq!(&s, "bbbbbaaaaaccccc");
}

#[test]
fn mutable_insert_char_test_1() {
    let mut s = Rope::from("aaaaa");
    for _ in 0..5 { s.insert(0, 'b')}
    for _ in 0..5 { s.insert(10, 'c')}
    assert_eq!(&s, "bbbbbaaaaaccccc");
}

#[test]
fn mutable_insert_char_test_2() {
    // this is the same as with_insert_char_test1() except mutable
    let mut s = Rope::from("aaaaa");
    assert_eq!(&s, "aaaaa");
    s.insert(5, 'b');
    assert_eq!(&s, "aaaaab");
    s.insert(4, 'b');
    assert_eq!(&s, "aaaabab");
    s.insert(3, 'b');
    assert_eq!(&s, "aaababab");
    s.insert(2, 'b');
    assert_eq!(&s, "aabababab");
    s.insert(1, 'b');
    assert_eq!(&s, "ababababab");

}

#[test]
fn with_insert_char_test_1() {
    let s = Rope::from("aaaaa");
    let s_1 = s.with_insert(5, 'b');
    let s_2 = s_1.with_insert(4, 'b');
    let s_3 = s_2.with_insert(3, 'b');
    let s_4 = s_3.with_insert(2, 'b');
    let s_5 = s_4.with_insert(1, 'b');
    assert_eq!(&s, "aaaaa");
    assert_eq!(&s_1, "aaaaab");
    assert_eq!(&s_2, "aaaabab");
    assert_eq!(&s_3, "aaababab");
    assert_eq!(&s_4, "aabababab");
    assert_eq!(&s_5, "ababababab");

}

#[test]
fn with_insert_str_test_1() {
    let s = Rope::from("aaaaa");
    let s_1 = s.with_insert_str(5, "ccccc");
    let s_2 = s_1.with_insert_str(5, "bbbbb");
    assert_eq!(&s, "aaaaa");
    assert_eq!(&s_1, "aaaaaccccc");
    assert_eq!(&s_2, "aaaaabbbbbccccc");
}


#[test]
fn rope_lines_iter() {
    let s = "line a\n\
             line b\n\
             line c\n\
             line d";
    let r = Rope::from(s);
    assert_eq!(r.lines().collect::<Vec<_>>(), s.lines().collect::<Vec<_>>());
    let r = Rope::from("line a\n") +
            Rope::from("line b\n") +
            Rope::from("line c\n") +
            Rope::from("line d\n");
    assert_eq!(r.lines().collect::<Vec<_>>(), s.lines().collect::<Vec<_>>());
}

#[test]
fn rope_lines_iter_split_on_node() {
    use super::internals::Node;
    use super::internals::Node::Leaf;
    let s = "line a\n\
             line b\n\
             line c\n";
    let r = Rope {
        root: Node::new_branch(
                Node::new_branch( Leaf("line".to_string())
                                , Leaf(" a\n".to_string())
                                )
              , Node::new_branch( Leaf("line b\n".to_string())
                                , Node::new_branch( Leaf("li".to_string())
                                                  , Leaf("ne c\n".to_string())
                                                  )
                                )
              )
    };
    assert_eq!(r.lines().collect::<Vec<_>>(), s.lines().collect::<Vec<_>>());
}


#[test]
fn rope_char_indices() {
    let rope = Rope::from("aaaaa")
        .with_append(Rope::from("bbbbbb"))
        .with_append(Rope::from("cccccccccccc"))
        .with_append(Rope::from("defgdefgaabababab"));
    let string = String::from("aaaaabbbbbbccccccccccccdefgdefgaabababab");
    let indices = rope.char_indices().zip(string.char_indices());
    for ((ridx, rch), (sidx, sch)) in indices {
        assert_eq!(rch, sch);
        assert_eq!(ridx, sidx);
    }
}

mod properties {
    use ::Rope;
    use quickcheck::{TestResult, quickcheck};
    quickcheck! {
        fn rope_concat_is_string_concat(a: String, b: String) -> bool {
            let r_a = Rope::from(a.clone()); let r_b = Rope::from(b.clone());
            &(r_a + r_b) == &(a + b.as_str())
        }

        fn rope_with_append_prepend_is_symmetric(a: Rope, b: Rope) -> bool {
            a.with_append(b.clone()) == b.with_prepend(a.clone()) &&
            a.with_prepend(b.clone()) == b.with_append(a.clone())
        }

        fn rope_append_prepend_is_symmetric(a: Rope, b: Rope) -> bool {
            a.clone().append(b.clone()) == b.clone().prepend(a.clone()) &&
            a.clone().prepend(b.clone()) == b.clone().append(a.clone())
        }

        fn rope_append_is_string_push_str(a: String, b: String) -> bool {
            let mut rope = Rope::from(a.clone());
            rope.append(Rope::from(b.clone()));
            let mut string = a;
            string.push_str(&b[..]);
            rope == string
        }

        fn rope_add_assign_is_string_push_str(a: String, b: String) -> bool {
            let mut rope = Rope::from(a.clone());
            rope += b.clone();
            let mut string = a;
            string.push_str(&b[..]);
            rope == string
        }

    }

    #[ignore]
    fn rope_indexing_is_string_indexing() {
        fn prop(string: String, i: usize) -> TestResult {
            use ::unicode::Unicode;
            if i >= string.grapheme_len() || !string.is_char_boundary(i)  ||
                // ignore the Dread String Of 85 Nulls
                string.matches("\u{0}").count() > 1
            {
                return TestResult::discard()
            }
            let rope = Rope::from(string.clone());
            TestResult::from_bool(&rope[i] == &string[i..i+1])
        }
        quickcheck(prop as fn(String, usize) -> TestResult);
    }

    #[ignore]
    fn rope_insert_char_is_string_insert_char() {
        fn prop(a: String, ch: char, i: usize) -> TestResult {
            // if the index is greater than the string's length...
            if i > a.len()
                    // ...or the index falls in the middle of a char...
                || !a.is_char_boundary(i)
                    // ...or QuickCheck made the Dread String of 85 Nulls...
                || a.matches("\u{0}").count() > 1
            {
                // ..skip the test
                return TestResult::discard()
            }

            let mut rope = Rope::from(a.clone());
            rope.insert(i, ch);

            let mut string = a;
            string.insert(i, ch);

            TestResult::from_bool(rope == string)
        }
        quickcheck(prop as fn(String, char, usize) -> TestResult);
    }

    #[cfg(feature = "unstable")]
    #[test]
    fn rope_insert_str_is_string_insert_str() {
        fn prop(a: String, b: String, i: usize) -> TestResult {
            // if the index is greater than the string's length...
            if i > a.len() || !a.is_char_boundary(i) {
                // ..skip the test
                return TestResult::discard()
            }

            let mut rope = Rope::from(a.clone());
            rope.insert_str(i, &b[..]);

            let mut string = a;
            string.insert_str(i, &b[..]);

            TestResult::from_bool(rope == string)
        }
        quickcheck(prop as fn(String, String, usize) -> TestResult);
    }

}


mod iterator {

    mod Extend {
        use ::Rope;
        #[test]
        fn char_ref_empty () {
            let mut rope = Rope::from("");
            rope.extend(&vec!['a', 'b', 'c', 'd']);
            assert_eq!(&rope, "abcd");
        }

        #[test]
        fn char_refs_nonempty () {
            let mut rope = Rope::from("a");
            rope.extend(vec![&'b', &'c', &'d']);
            assert_eq!(&rope, "abcd");
        }

        fn empty_iter () {
            let mut rope = Rope::from("");
            let empty_vec: Vec<String> = vec![];
            rope.extend(empty_vec);
            assert_eq!(&rope, "");
        }


        #[test]
        fn string_empty () {
            let mut rope = Rope::from("");
            rope.extend(vec![ String::from("aaaa")
                            , String::from("bbbb")
                            , String::from("cccc")
                            ]);
            assert_eq!(&rope, "aaaabbbbcccc");
        }

        #[test]
        fn string_nonempty () {
            let mut rope = Rope::from("aaaa");
            rope.extend(vec![String::from("bbbb"), String::from("cccc")]);
            assert_eq!(&rope, "aaaabbbbcccc");
        }


        #[test]
        fn rope_empty () {
            let mut rope = Rope::from("");
            rope.extend(vec![ Rope::from("aaaa")
                            , Rope::from("bbbb")
                            , Rope::from("cccc")
                            ]);
            assert_eq!(&rope, "aaaabbbbcccc");
        }

        #[test]
        fn rope_nonempty () {
            let mut rope = Rope::from("aaaa");
            rope.extend(vec![Rope::from("bbbb"), Rope::from("cccc")]);
            assert_eq!(&rope, "aaaabbbbcccc");
        }

        #[test]
        fn str_slice_empty () {
            let mut rope = Rope::from("");
            rope.extend(vec![ "aaaa"
                            , "bbbb"
                            , "cccc"
                            ]);
            assert_eq!(&rope, "aaaabbbbcccc");
        }

        #[test]
        fn str_slice_nonempty () {
            let mut rope = Rope::from("aaaa");
            rope.extend(vec!["bbbb", "cccc"]);
            assert_eq!(&rope, "aaaabbbbcccc");
        }

        #[test]
        fn chars_empty () {
            let mut rope = Rope::from("");
            rope.extend(vec!['a', 'b', 'c', 'd']);
            assert_eq!(&rope, "abcd");
        }

        #[test]
        fn chars_nonempty () {
            let mut rope = Rope::from("a");
            rope.extend(vec!['b', 'c', 'd']);
            assert_eq!(&rope, "abcd");
        }
    }


    mod FromIterator {
        use ::Rope;
        quickcheck! {
            fn prop_strings_concat(v: Vec<String>) -> bool {
                let rope: Rope = v.clone().into_iter().collect();
                &rope == &(v.into_iter().collect::<String>()[..])
            }

            fn prop_chars_concat(v: Vec<char>) -> bool {
                let rope: Rope = v.clone().into_iter().collect();
                &rope == &(v.into_iter().collect::<String>()[..])
            }

            fn prop_ropes_concat(v: Vec<String>) -> bool {
                let a: Rope = v.clone().into_iter()
                               .map(Rope::from)
                               .collect();
                let b: Rope = v.clone().into_iter()
                               .collect();
                &a == &b
            }
        }
        #[test]
        fn str_slice () {
            let vec = vec!["aaaa", "bbbb", "cccc"];
            let rope: Rope = vec.into_iter().collect();
            assert_eq!(&rope, "aaaabbbbcccc");
        }

        #[test]
        fn str_slice_empty () {
            let vec: Vec<&str> = vec![];
            let rope: Rope = vec.into_iter().collect();
            assert_eq!(&rope, "");
        }

        #[test]
        fn chars () {
            let vec = vec!['a', 'b', 'c', 'd'];
            let rope: Rope = vec.into_iter().collect();
            assert_eq!(&rope, "abcd");
        }

        #[test]
        fn chars_empty () {
            let vec: Vec<char> = vec![];
            let rope: Rope = vec.into_iter().collect();
            assert_eq!(&rope, "");
        }

        #[test]
        fn char_refs () {
            let vec = vec!['a', 'b', 'c', 'd'];
            let rope: Rope = (&vec).into_iter().collect();
            assert_eq!(&rope, "abcd");
        }

        #[test]
        fn char_refs_empty () {
            let vec: Vec<&char> = vec![];
            let rope: Rope = vec.into_iter().collect();
            assert_eq!(&rope, "");
        }

        #[test]
        fn strings () {
            let vec: Vec<String> = vec![ String::from("aaaa")
                                       , String::from("bbbb")
                                       , String::from("cccc")];
            let rope: Rope = vec.into_iter().collect();
            assert_eq!(&rope, "aaaabbbbcccc");
        }

        #[test]
        fn strings_empty () {
            let vec: Vec<String> = vec![];
            let rope: Rope = vec.into_iter().collect();
            assert_eq!(&rope, "");
        }

        #[test]
        fn ropes () {
            let vec: Vec<Rope> = vec![ Rope::from("aaaa")
                                     , Rope::from("bbbb")
                                     , Rope::from("cccc")];
            let rope: Rope = vec.into_iter().collect();
            assert_eq!(&rope, "aaaabbbbcccc");
        }

        #[test]
        fn ropes_empty () {
            let vec: Vec<Rope> = vec![];
            let rope: Rope = vec.into_iter().collect();
            assert_eq!(&rope, "");
        }

    }

}
