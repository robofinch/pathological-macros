This repository showcases how a pathological attribute macro can cause UB via a derive
macro that implements an `unsafe` trait. (In particular, `derive(yoke::Yokeable)`, though
surely there are other such macros vulnerable to this trick.)

Note that the `solution-macro` and `solution-against-normal` crates are based on `yoke_derive` and
are therefore under the same Unicode 3.0 license as `yoke_derive`.
