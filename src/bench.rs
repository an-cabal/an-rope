extern crate test;
use self::test::Bencher;

use super::Rope;
use std::iter;

#[bench]
fn rope_add_1000(b: &mut Bencher) {
    let mut rope = Rope::from(iter::repeat('a')
                                    .take(100_000)
                                    .collect::<String>());
    // rope.rebalance();
    b.iter(|| {
        let n = test::black_box(1000);
        let mut r = rope.clone();
        for i in 0..n {
            r = r + i.to_string();
        }
    })

}

#[bench]
fn string_add_1000(b: &mut Bencher) {
    let mut string = iter::repeat('a').take(100_000).collect::<String>();
    b.iter(move || {
        let n = test::black_box(1000);
        let mut s = string.clone();
        for i in 0..n {
            s = s + &i.to_string();
        }
    })

}

#[bench]
fn rope_insert_1000(b: &mut Bencher) {
    let mut rope = Rope::from(iter::repeat('a')
                                    .take(100_000)
                                    .collect::<String>());
    // rope.rebalance();
    b.iter(move || {
        // let n = test::black_box(1000);
        // let mut rope = Rope::from("aaaa");
        for i in 0..1000 {
            rope.insert_str(2, &i.to_string());
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

macro_rules! insert_benches {
    ( long: $lenl:expr, short: $lens:expr, $($name:ident: $frac:expr),* ) => {
        mod insert {
        mod rope {
            $(

                mod $name {
                    use ::Rope;
                    use ::bench::test::Bencher;
                    use std::iter::repeat;

                    macro_rules! mk_bench {
                        ( $n:ident: $fun:ident, $l:expr, $ins:expr) => {
                            #[bench] fn $n(b: &mut Bencher) {
                                let rope = Rope::from(repeat('a').take($l)
                                                        .collect::<String>());
                                b.iter(|| { rope.$fun(
                                    ($l as f64 * $frac as f64) as usize, $ins)
                                })
                            }
                        }
                    }

                    mk_bench! { long: insert_str, $lenl, "bbbbbbb" }
                    mk_bench! { short: insert_str, $lens, "bb" }
                    mk_bench! { char_long: insert, $lenl, 'c' }
                    mk_bench! { char_short: insert, $lens, 'c' }
                    // mk_bench! { rope_long: insert_rope, $lenl, &rope }
                    // mk_bench! { rope_short: insert_rope, $lens, &rope }

                    #[bench] fn rope_long(b: &mut Bencher) {
                        let rope = Rope::from(repeat('a').take($lenl)
                                                .collect::<String>());
                        b.iter(|| { rope.insert_rope(
                            ($lenl as f64 * $frac as f64) as usize, &rope)
                        })
                    }
                    #[bench] fn rope_short(b: &mut Bencher) {
                        let rope = Rope::from(repeat('a').take($lens)
                                                .collect::<String>());
                        b.iter(|| { rope.insert_rope(
                            ($lens as f64 * $frac as f64) as usize, &rope)
                        })
                    }
                }
            )*
        }
        mod string {
            $(
                mod $name {
                    use ::bench::test::Bencher;
                    use std::iter::repeat;
                    macro_rules! mk_bench {
                        ( $n:ident: $fun:ident, $l:expr, $ins:expr) => {
                            #[bench] fn $n(b: &mut Bencher) {
                                let mut string = repeat('a').take($l)
                                                .collect::<String>();
                                b.iter(|| { string.$fun(
                                    ($l as f64 * $frac as f64) as usize, $ins)
                                })
                            }
                        }
                    }
                    mk_bench! { long: insert_str, $lenl, "bbbbbbb" }
                    mk_bench! { short: insert_str, $lens, "bb" }
                    mk_bench! { char_long: insert, $lenl, 'c' }
                    mk_bench! { char_short: insert, $lens, 'c'  }
                }
            )*
        }
    }}
}

macro_rules! split_benches {
    ( long: $lenl:expr, short: $lens:expr, $($name:ident: $frac:expr),* ) => {
        mod split {
            $(
                mod $name {
                    use ::Rope;
                    use ::bench::test::Bencher;
                    use std::iter::repeat;
                    #[bench]
                    fn long(b: &mut Bencher) {
                        let rope = Rope::from(repeat('a').take($lenl)
                                                         .collect::<String>());
                        let split = || {
                            rope.split(($lenl as f64 * $frac as f64) as usize) };
                        b.iter(split)
                    }
                    #[bench]
                    fn short(b: &mut Bencher) {
                        let rope = Rope::from(repeat('a').take($lens)
                                                         .collect::<String>());
                        let split = || {
                            rope.split(($lens as f64 * $frac as f64) as usize) };
                        b.iter(split)
                    }
                }
            )*
        }
    }
}
#[cfg(all( test, feature = "unstable") )]
split_benches! {
    long: 100_000, short: 100,
        at_start: 0,
        at_quarter: 0.25,
        at_half: 0.5,
        at_3quarter: 0.75,
        at_end: 1
}
#[cfg(all( test, feature = "unstable") )]
insert_benches! {
    long: 100_000, short: 100,
        at_start: 0,
        at_quarter: 0.25,
        at_half: 0.5,
        at_3quarter: 0.75,
        at_end: 1
}
