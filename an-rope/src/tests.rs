use super::test;
use test::Bencher;

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
    let an_rope = Rope::from("");
    let another_rope = an_rope.append(Rope::from("abcd"));
    assert_eq!(&another_rope, "abcd");
    assert_eq!(&an_rope, "");

    let an_rope = Rope::from("abcd");
    let an_rope = an_rope.append(Rope::from(""));
    assert_eq!(&an_rope, "abcd");
}

#[test]
fn append() {
    let an_rope = Rope::from("abcd");
    let another_rope = an_rope.append(Rope::from("efgh"));
    assert_eq!(&another_rope, "abcdefgh");
    assert_eq!(&an_rope, "abcd");
}

#[test]
fn prepend_empty_rope() {
    let an_rope = Rope::from("");
    let another_rope = an_rope.prepend(Rope::from("abcd"));
    assert_eq!(&an_rope, "");
    assert_eq!(&another_rope, "abcd");

    let an_rope = Rope::from("abcd");
    let another_rope = an_rope.prepend(Rope::from(""));
    assert_eq!(&an_rope, "abcd");
    assert_eq!(&another_rope, &an_rope);
    assert_eq!(&another_rope, "abcd");
}

#[test]
fn prepend() {
    let an_rope = Rope::from("efgh");
    let another_rope = an_rope.prepend(Rope::from("abcd"));
    assert_eq!(&an_rope, "efgh");
    assert_eq!(&another_rope, "abcdefgh");
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
fn insert_rope_balance_test() {
    let s: String = iter::repeat('a').take(10).collect();
    let mut r_1 = Rope::from(s);
    for _ in 0..99 {
        let t: String = iter::repeat('a').take(10).collect();
        r_1 = r_1.insert_rope(5, Rope::from(t));
    }
    //  this isn't necessary, insert_rope() will automatically rebalance
    //      - eliza, 12/18/2016
    // r_1.rebalance();
    assert!(r_1.is_balanced());
}

#[test]
fn insert_rope_test_1() {
    let s: String = iter::repeat('a').take(1_000).collect();
    let r_1 = Rope::from(s);
    let mut r_2 = Rope::new();
    for _ in 0..100 {
        let t: String = iter::repeat('a').take(10).collect();
        r_2 = r_2.insert_rope(0, Rope::from(t));
    }
    assert_eq!(r_1, r_2);
}

#[test]
fn insert_rope_test_2() {
    let s: String = iter::repeat('a').take(10).collect();
    let mut r_1 = Rope::from(s);
    for _ in 0..99 {
        let t: String = iter::repeat('a').take(10).collect();
        r_1 = r_1.insert_rope(5, Rope::from(t));
    }

    let q: String = iter::repeat('a').take(1_000).collect();
    let r_2 = Rope::from(q);
    assert_eq!(r_1, r_2);
}
//
// #[test]
// fn mutable_insert_rope_test_1() {
//     let mut s_1 = Rope::from(String::from("aaaaa"));
//     let mut s_2 = Rope::from(String::from("bbbbb"));
//     let s_3 = Rope::from(String::from("ccccc"));
//     s_2.insert_rope(0, s_3);
//     s_1.insert_rope(0, s_2);
//     assert_eq!(&s_1, "cccccbbbbbaaaaa");
// }
//
// #[test]
// fn mutable_insert_str_test_1() {
//     let mut s = Rope::from("aaaaa");
//     s.insert_str(0, "bbbbb");
//     s.insert_str(10, "ccccc");
//     assert_eq!(&s, "bbbbbaaaaaccccc");
// }
//
// #[test]
// fn mutable_insert_char_test_1() {
//     let mut s = Rope::from("aaaaa");
//     for _ in 0..5 { s.insert(0, 'b')}
//     for _ in 0..5 { s.insert(10, 'c')}
//     assert_eq!(&s, "bbbbbaaaaaccccc");
// }
//
// #[test]
// fn mutable_insert_char_test_2() {
//     // this is the same as insert_char_test1() except mutable
//     let mut s = Rope::from("aaaaa");
//     assert_eq!(&s, "aaaaa");
//     s.insert(5, 'b');
//     assert_eq!(&s, "aaaaab");
//     s.insert(4, 'b');
//     assert_eq!(&s, "aaaabab");
//     s.insert(3, 'b');
//     assert_eq!(&s, "aaababab");
//     s.insert(2, 'b');
//     assert_eq!(&s, "aabababab");
//     s.insert(1, 'b');
//     assert_eq!(&s, "ababababab");
//
// }

#[test]
fn insert_char_test_1() {
    let s = Rope::from("aaaaa");
    let s_1 = s.insert(5, 'b');
    let s_2 = s_1.insert(4, 'b');
    let s_3 = s_2.insert(3, 'b');
    let s_4 = s_3.insert(2, 'b');
    let s_5 = s_4.insert(1, 'b');
    assert_eq!(&s, "aaaaa");
    assert_eq!(&s_1, "aaaaab");
    assert_eq!(&s_2, "aaaabab");
    assert_eq!(&s_3, "aaababab");
    assert_eq!(&s_4, "aabababab");
    assert_eq!(&s_5, "ababababab");

}

#[test]
fn insert_str_test_1() {
    let s = Rope::from("aaaaa");
    let s_1 = s.insert_str(5, "ccccc");
    let s_2 = s_1.insert_str(5, "bbbbb");
    assert_eq!(&s, "aaaaa");
    assert_eq!(&s_1, "aaaaaccccc");
    assert_eq!(&s_2, "aaaaabbbbbccccc");
}

#[bench]
fn rope_add_1000(b: &mut Bencher) {
    let rope = Rope::from(iter::repeat('a')
                                    .take(100_000)
                                    .collect::<String>());
    // rope.rebalance();
    b.iter(|| {
        let n = test::black_box(1000);
        // let mut rope = Rope::new();
        (0..n).fold(rope.clone(), |ref r, i| r + &i.to_string()[..] )
    })

}

#[bench]
fn string_add_1000(b: &mut Bencher) {
    let mut string = iter::repeat('a').take(100_000).collect::<String>();
    b.iter(|| {
        let n = test::black_box(1000);
        // let mut string = String::new();
        for i in 0..n {
            string += &i.to_string();
        }
    })

}

#[bench]
fn rope_insert_1000(b: &mut Bencher) {
    let mut rope = Rope::from(iter::repeat('a')
                                    .take(100_000)
                                    .collect::<String>());
    // rope.rebalance();
    b.iter(|| {
        // let n = test::black_box(1000);
        // let mut rope = Rope::from("aaaa");
        for i in 0..1000 {
            rope = rope.insert_str(2, &i.to_string());
        }
    })

}

#[bench]
fn string_insert_1000(b: &mut Bencher) {
    let mut string = iter::repeat('a').take(100_000).collect::<String>();
    b.iter(|| {
        // let n = test::black_box(1000);
        // let mut string = String::from("aaaa");
        for i in 0..1000 {
            string.insert_str(2, &i.to_string());
        }
    })

}

#[bench]
fn string_insert_at_half_long(b: &mut Bencher) {
    let mut string = iter::repeat('a').take(100_000).collect::<String>();
    b.iter(|| { string.insert_str(50_000, "bbbbbbb") })
}

#[bench]
fn rope_insert_at_half_long(b: &mut Bencher) {
    let mut rope = Rope::from(iter::repeat('a')
                                    .take(100_000)
                                    .collect::<String>());
    b.iter(|| { rope.insert_str(50_000, "bbbbbbb") })
}

#[bench]
fn string_insert_at_start_long(b: &mut Bencher) {
    let mut string = iter::repeat('a').take(100_000).collect::<String>();
    b.iter(|| { string.insert_str(1, "bbbbbbb") })
}

#[bench]
fn rope_insert_at_start_long(b: &mut Bencher) {
    let mut rope = Rope::from(iter::repeat('a')
                                    .take(100_000)
                                    .collect::<String>());
    b.iter(|| { rope.insert_str(1, "bbbbbbb") })
}


#[bench]
fn string_insert_at_half_short(b: &mut Bencher) {
    let mut string = iter::repeat('a').take(100).collect::<String>();
    b.iter(|| { string.insert_str(50, "bbbbbbb") })
}

#[bench]
fn rope_insert_at_half_short(b: &mut Bencher) {
    let mut rope = Rope::from(iter::repeat('a')
                                    .take(100)
                                    .collect::<String>());
    b.iter(|| { rope.insert_str(50, "bbbbbbb") })
}


#[bench]
fn string_insert_at_start_short(b: &mut Bencher) {
    let mut string = iter::repeat('a').take(100).collect::<String>();
    b.iter(|| { string.insert_str(1, "bbbbbbb") })
}

#[bench]
fn rope_insert_at_start_short(b: &mut Bencher) {
    let mut rope = Rope::from(iter::repeat('a')
                                    .take(100)
                                    .collect::<String>());
    b.iter(|| { rope.insert_str(1, "bbbbbbb") })
}

#[bench]
fn string_insert_char_at_half_long(b: &mut Bencher) {
    let mut string = iter::repeat('a').take(100_000).collect::<String>();
    b.iter(|| { string.insert(50_000, 'b') })
}

#[bench]
fn rope_insert_char_at_half_long(b: &mut Bencher) {
    let mut rope = Rope::from(iter::repeat('a')
                                    .take(100_000)
                                    .collect::<String>());
    b.iter(|| { rope.insert(50_000, 'b') })
}

#[bench]
fn string_insert_char_at_start_long(b: &mut Bencher) {
    let mut string = iter::repeat('a').take(100_000).collect::<String>();
    b.iter(|| { string.insert(1, 'b') })
}

#[bench]
fn rope_insert_char_at_start_long(b: &mut Bencher) {
    let mut rope = Rope::from(iter::repeat('a')
                                    .take(100_000)
                                    .collect::<String>());
    b.iter(|| { rope.insert(1, 'b') })
}


#[bench]
fn string_insert_char_at_half_short(b: &mut Bencher) {
    let mut string = iter::repeat('a').take(100).collect::<String>();
    b.iter(|| { string.insert(50, 'b') })
}

#[bench]
fn rope_insert_char_at_half_short(b: &mut Bencher) {
    let mut rope = Rope::from(iter::repeat('a')
                                    .take(100)
                                    .collect::<String>());
    b.iter(|| { rope.insert(50, 'b') })
}


#[bench]
fn string_insert_char_at_start_short(b: &mut Bencher) {
    let mut string = iter::repeat('a').take(100).collect::<String>();
    b.iter(|| { string.insert(1, 'b') })
}

#[bench]
fn rope_insert_char_at_start_short(b: &mut Bencher) {
    let mut rope = Rope::from(iter::repeat('a')
                                    .take(100)
                                    .collect::<String>());
    b.iter(|| { rope.insert(1, 'b') })
}
