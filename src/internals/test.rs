use super::{Value, Node};
use self::Value::Leaf;
use metric::Line;

#[test]
fn line_split_test_1() {
    let l1 = Node::new_leaf("asdf");
    let l2 = Node::new_leaf("qwer");
    let b = Node::new_branch(l1, l2);
    let (left, right) = b.split(Line(0));
    assert_eq!(left.strings().collect::<String>(), "asdfqwer");
    if let Leaf(ref s) = **right {
        assert_eq!(&s[..], "");
    } else { assert!(false) }
}

#[test]
fn line_split_test_2() {
    let l1 = Node::new_leaf("asdf");
    let l2 = Node::new_leaf("qwer\n");
    let b = Node::new_branch(l1, l2);
    let (left, right) = b.split(Line(0));
    assert_eq!(left.strings().collect::<String>(), "asdfqwer\n");
    if let Leaf(ref s) = **right {
        assert_eq!(&s[..], "");
    } else { assert!(false) }
}

#[test]
fn line_split_test_3() {
    let l1 = Node::new_leaf("asdf\n");
    let l2 = Node::new_leaf("qwer\n");
    let b = Node::new_branch(l1, l2);
    let (left, right) = b.split(Line(0));
    if let Leaf(ref s) = **left {
        assert_eq!(&s[..], "asdf\n");
    } else { assert!(false) }
    if let Leaf(ref s) = **right {
        assert_eq!(&s[..], "qwer\n");
    } else { assert!(false) }
}

#[test]
#[should_panic(expected = "invalid index!")]
fn line_split_test_4() {
    let l1 = Node::new_leaf("asdf");
    let l2 = Node::new_leaf("qwer");
    let b = Node::new_branch(l1, l2);
    let (left, right) = b.split(Line(1));
}

#[test]
fn line_split_test_5() {
    let l1 = Node::new_leaf("asdf");
    let l2 = Node::new_leaf("qwer\n");
    let b = Node::new_branch(l1, l2);
    let (left, right) = b.split(Line(1));
    assert_eq!(left.strings().collect::<String>(), "asdfqwer\n");
    if let Leaf(ref s) = **right {
        assert_eq!(&s[..], "");
    } else { assert!(false) }
}

#[test]
fn line_split_test_6() {
    let l1 = Node::new_leaf("asdf\n");
    let l2 = Node::new_leaf("qwer\n");
    let b = Node::new_branch(l1, l2);
    let (left, right) = b.split(Line(1));
    assert_eq!(left.strings().collect::<String>(), "asdf\nqwer\n");
    if let Leaf(ref s) = **right {
        assert_eq!(&s[..], "");
    } else { assert!(false) }
}

#[test]
fn line_split_test_7() {
    let l1 = Node::new_leaf("asdf\n");
    let l2 = Node::new_leaf("qwer\n");
    let b = Node::new_branch(l1, l2);
    let (left, right) = b.split(Line(0));
    if let Leaf(ref s) = **left {
        assert_eq!(&s[..], "asdf\n");
    } else { assert!(false) }
    if let Leaf(ref s) = **right {
        assert_eq!(&s[..], "qwer\n");
    } else { assert!(false) }
}

#[test]
fn line_split_test_8() {
    let l1 = Node::new_leaf("");
    let l2 = Node::new_leaf("qwer\n");
    let b = Node::new_branch(l1, l2);
    let (left, right) = b.split(Line(0));
    assert_eq!(left.strings().collect::<String>(), "qwer\n");
    if let Leaf(ref s) = **right {
        assert_eq!(&s[..], "");
    } else { assert!(false) }
}

#[test]
fn line_split_test_9() {
    let l1 = Node::new_leaf("asdf\n");
    let l2 = Node::new_leaf("qwer");
    let l3 = Node::new_leaf("yxcv\n");
    let b1 = Node::new_branch(l1, l2);
    let b2 = Node::new_branch(b1, l3);
    let (left, right) = b2.split(Line(0));
    if let Leaf(ref s) = **left {
        assert_eq!(&s[..], "asdf\n");
    } else { assert!(false) }
    assert_eq!(right.strings().collect::<String>(), "qweryxcv\n");
}

#[test]
fn line_split_test_10() {
    let l1 = Node::new_leaf("asdf");
    let l2 = Node::new_leaf("qwer\n");
    let l3 = Node::new_leaf("yxcv\n");
    let b1 = Node::new_branch(l2, l3);
    let b2 = Node::new_branch(l1, b1);
    let (left, right) = b2.split(Line(0));
    assert_eq!(left.strings().collect::<String>(), "asdfqwer\n");
    if let Leaf(ref s) = **right {
        assert_eq!(&s[..], "yxcv\n");
    } else { assert!(false) }
}
