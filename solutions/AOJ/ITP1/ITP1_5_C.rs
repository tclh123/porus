#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    loop {
        let (mut h, mut w) : (isize, isize) = default();
        read!(&mut h, &mut w);
        if (h == 0) && (w == 0) {
            break;
        }

        for i in 0..h {
            for j in 0..w {
                writef!(
                    "{:s}",
                    match (i % 2) == (j % 2) {
                        false => ".",
                        true => "#",
                    }
                );
            }
            writelnf!("");
        }
        writelnf!("");
    }
}
