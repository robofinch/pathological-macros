/// The below is a compile_fail doctest.
///
/** ```compile_fail
use std::{cell::Cell, marker::PhantomData, ops::Deref, rc::Rc};

use yoke::{Yoke, Yokeable};


macro_rules! transform_to_invariant {
    (&$lt:lifetime Covariant<$lifetime:lifetime>) => {
        &$lifetime Invariant<$lifetime>
    };
}

// This fails to compile because `derive(Yokeable)` fails to identify that
// `transform_to_invariant`'s output involves the `'a` lifetime. Even if you get it to
// recognize that the `transformed_to_invariant` field uses `'a`, it tries to change the
// `'a` lifetime to `'static` and require that the modified type implements `Yokeable`;
// but that fails.
//
// Essentially... ** macro types are required to be `'static` by `derive(Yokeable)` **.
//
// Even if non-`'static` macro types did work, a pure macro type would be recognized as invariant
// and therefore fail.
// A tuned nonpure proc macro (using some method to output something different each time it's
// called) would, in theory, still be able to cause unsoundness.
#[derive(Yokeable)]
// Rely on `derive(Yokeable)`'s flawed checks for covariance instead of the compiler's unevadable
// checks for covariance.
#[yoke(prove_covariance_manually)]
struct MightLookCovariant<'a> {
    // Change `&'a Covariant<'a>` to `&'a Invariant<'a>` after `derive(Yokeable)`
    // has already been run/expanded, so that its manual covariance checks are flawed.
    transformed_to_invariant: transform_to_invariant!(&'a Covariant<'a>),
}

// We somehow manage to dodge the problems caused by autoderef cycles.
// Removing the first `PhantomData` field of `Invariant` lets the cycle cause a compiler
// error, though. No clue why.
struct Invariant<'a>(PhantomData<&'a ()>, Cell<&'a str>);
struct Covariant<'a>(PhantomData<&'a ()>);

impl Invariant<'_> {
    fn new_invariant() -> Self {
        Self(PhantomData, Cell::new("hi"))
    }
}

impl Covariant<'_> {
    fn new_covariant() -> Self {
        Self(PhantomData)
    }
}

impl<'a> Deref for Invariant<'a> {
    type Target = &'a Covariant<'a>;

    fn deref(&self) -> &Self::Target {
        Box::leak(Box::new(&*Box::leak(Box::new(Covariant::new_covariant()))))
    }
}

impl<'a> Deref for Covariant<'a> {
    type Target = Invariant<'a>;

    fn deref(&self) -> &Self::Target {
        Box::leak(Box::new(Invariant::new_invariant()))
    }
}

fn main() {
    let yoke: Yoke<MightLookCovariant<'static>, Rc<&str>> = Yoke::attach_to_cart(
        Rc::new("hi"),
        |_source| {
            MightLookCovariant {
                transformed_to_invariant: Box::leak(Box::new(Invariant::new_invariant())),
            }
        },
    );

    {
        let short_lived = String::from("goodbye, world");
        yoke.get().transformed_to_invariant.1.set(&short_lived);
    };

    // use-after-free
    println!("{}", yoke.get().transformed_to_invariant.1.get());
}
``` */
#[expect(dead_code)]
const UNSUPPORTED: () = ();
