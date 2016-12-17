use super::Rope;
use std::iter;

#[test]
fn rebalance_test_1() {
    let r = Rope::from("This is a large string \
                        that will need to be rebalanced.".to_string());
    let r = r.rebalance();
    assert!(r.is_balanced());
}

#[test]
fn rebalance_test_2() {
    let r = Rope::from("Lorem ipsum dolor sit amet, consectetur adipiscing eli\
                        t, sed do eiusmod tempor incididunt ut labore et dolor\
                        e magna aliqua. Ut enim ad minim veniam, quis nostrud \
                        exercitation ullamco laboris nisi ut aliquip ex ea com\
                        modo consequat. Duis aute irure dolor in reprehenderit\
                         in voluptate velit esse cillum dolore eu fugiat nulla\
                         pariatur. Excepteur sint occaecat cupidatat non proid\
                        ent, sunt in culpa qui officia deserunt mollit anim id\
                         est laborum.".to_string());
    let r = r.rebalance();
    assert!(r.is_balanced());
}

#[test]
fn big_rebalance() {
    let s: String = iter::repeat('a').take(10_000).collect();
    let r = Rope::from(s);
    let r = r.rebalance();
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
     let u = t + s;
     let u = u.rebalance();
     assert!(u.is_balanced());
}
