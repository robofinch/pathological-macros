This repository showcases how a pathological attribute macro can cause UB via a derive
macro that implements an `unsafe` trait. (In particular, `derive(yoke::Yokeable)`, though
surely there are other such macros vulnerable to this trick.)

TODO: develop a solution
