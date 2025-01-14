// revisions: current next
//[next] compile-flags: -Ztrait-solver=next
// run-pass

#![feature(generators, generator_trait)]
#![allow(drop_copy)]

use std::marker::{PhantomPinned, Unpin};

fn assert_unpin<G: Unpin>(_: G) {
}

fn main() {
    // Even though this generator holds a `PhantomPinned` in its environment, it
    // remains `Unpin`.
    assert_unpin(|| {
        let pinned = PhantomPinned;
        yield;
        drop(pinned);
    });
}
