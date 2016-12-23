use super::Rope;
use std::iter;

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
