// This file is a modified part of ICU4X's `yoke_derive` crate. For terms of use, please see the
// file called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(unused)]

use std::{borrow::Cow, marker::PhantomData};
use yoke::{Yoke, Yokeable};
use zerovec::{maps::ZeroMapKV, ule::AsULE, VarZeroVec, ZeroMap, ZeroVec};

use solution_macro::YokeableDeriveFixed;

#[derive(YokeableDeriveFixed)]
pub struct StringExample {
    x: String,
}

#[derive(YokeableDeriveFixed)]
#[yoke(prove_covariance_manually)]
pub struct ManualStringExample {
    x: String,
}

#[derive(YokeableDeriveFixed, Copy, Clone)]
pub struct IntExample {
    x: u32,
}

#[derive(YokeableDeriveFixed, Copy, Clone)]
#[yoke(prove_covariance_manually)]
pub struct ManualIntExample {
    x: u32,
}

#[derive(YokeableDeriveFixed, Copy, Clone)]
pub struct GenericsExample<T> {
    x: u32,
    y: T,
}

#[derive(YokeableDeriveFixed, Copy, Clone)]
#[yoke(prove_covariance_manually)]
pub struct ManualGenericsExample<T> {
    x: u32,
    y: T,
}

#[derive(YokeableDeriveFixed, Copy, Clone)]
pub struct GenericsExampleWithDefault<T, U = usize> {
    x: T,
    y: U,
}

#[derive(YokeableDeriveFixed, Copy, Clone)]
#[yoke(prove_covariance_manually)]
pub struct ManualGenericsExampleWithDefault<T, U = usize> {
    x: T,
    y: U,
}

#[derive(YokeableDeriveFixed)]
pub struct CowExample<'a> {
    x: u8,
    y: &'a str,
    z: Cow<'a, str>,
    w: Cow<'a, [u8]>,
}

#[derive(YokeableDeriveFixed)]
#[yoke(prove_covariance_manually)]
pub struct ManualCowExample<'a> {
    x: u8,
    y: &'a str,
    z: Cow<'a, str>,
    w: Cow<'a, [u8]>,
}

#[derive(YokeableDeriveFixed)]
pub struct ZeroVecExample<'a> {
    var: VarZeroVec<'a, str>,
    vec: ZeroVec<'a, u16>,
}

#[derive(YokeableDeriveFixed)]
#[yoke(prove_covariance_manually)]
pub struct ManualZeroVecExample<'a> {
    var: VarZeroVec<'a, str>,
    vec: ZeroVec<'a, u16>,
}

#[derive(YokeableDeriveFixed)]
pub struct ZeroVecExampleWithGenerics<'a, T: AsULE> {
    gen: ZeroVec<'a, T>,
    vec: ZeroVec<'a, u16>,
    bare: T,
}

#[derive(YokeableDeriveFixed)]
#[yoke(prove_covariance_manually)]
pub struct ManualZeroVecExampleWithGenerics<'a, T: AsULE> {
    gen: ZeroVec<'a, T>,
    vec: ZeroVec<'a, u16>,
    bare: T,
}

// Since ZeroMap has generic parameters, the Rust compiler cannot
// prove the covariance of the lifetimes. To use derive(Yokeable)
// with a type such as ZeroMap, you just add the attribute
// yoke(prove_covariance_manually)
#[derive(YokeableDeriveFixed)]
#[yoke(prove_covariance_manually)]
pub struct ZeroMapExample<'a> {
    map: ZeroMap<'a, str, u16>,
}

#[derive(YokeableDeriveFixed)]
#[yoke(prove_covariance_manually)]
pub struct ZeroMapGenericExample<'a, T: for<'b> ZeroMapKV<'b> + ?Sized> {
    map: ZeroMap<'a, str, T>,
}

#[derive(YokeableDeriveFixed)]
pub struct MaybeSizedWrap<T, Q: ?Sized, U: ?Sized> {
    x: T,
    y: Option<T>,
    ignored: PhantomData<U>,
    q: Q,
}

#[derive(YokeableDeriveFixed)]
#[yoke(prove_covariance_manually)]
pub struct ManualMaybeSizedWrap<T, Q: ?Sized, U: ?Sized> {
    x: T,
    y: Option<T>,
    ignored: PhantomData<U>,
    q: Q,
}

// TODO(#4119): Make this example compile
/*
#[derive(YokeableDeriveFixed)]
pub struct MaybeSizedWrapWithLifetime<'a, T, Q: ?Sized, U: ?Sized> {
    x: T,
    y: Option<T>,
    ignored: &'a U,
    q: Q,
}
*/

pub struct AssertYokeable {
    string: Yoke<StringExample, Box<[u8]>>,
    string_manual: Yoke<ManualStringExample, Box<[u8]>>,
    int: Yoke<IntExample, Box<[u8]>>,
    int_manual: Yoke<ManualIntExample, Box<[u8]>>,
    gen1: Yoke<GenericsExample<u32>, Box<[u8]>>,
    gen1_manual: Yoke<ManualGenericsExample<u32>, Box<[u8]>>,
    gen2: Yoke<GenericsExample<String>, Box<[u8]>>,
    gen2_manual: Yoke<ManualGenericsExample<String>, Box<[u8]>>,
    gen_default1: Yoke<GenericsExampleWithDefault<String>, Box<[u8]>>,
    gen_default1_manual: Yoke<ManualGenericsExampleWithDefault<String>, Box<[u8]>>,
    gen_default2: Yoke<GenericsExampleWithDefault<String, u8>, Box<[u8]>>,
    gen_default2_manual: Yoke<ManualGenericsExampleWithDefault<String, u8>, Box<[u8]>>,
    cow: Yoke<CowExample<'static>, Box<[u8]>>,
    cow_manual: Yoke<ManualCowExample<'static>, Box<[u8]>>,
    zv: Yoke<ZeroVecExample<'static>, Box<[u8]>>,
    zv_manual: Yoke<ManualZeroVecExample<'static>, Box<[u8]>>,
    zv_gen1: Yoke<ZeroVecExampleWithGenerics<'static, u8>, Box<[u8]>>,
    zv_gen1_manual: Yoke<ManualZeroVecExampleWithGenerics<'static, u8>, Box<[u8]>>,
    zv_gen2: Yoke<ZeroVecExampleWithGenerics<'static, char>, Box<[u8]>>,
    zv_gen2_manual: Yoke<ManualZeroVecExampleWithGenerics<'static, char>, Box<[u8]>>,
    map: Yoke<ZeroMapExample<'static>, Box<[u8]>>,
    map_gen1: Yoke<ZeroMapGenericExample<'static, u32>, Box<[u8]>>,
    map_gen2: Yoke<ZeroMapGenericExample<'static, str>, Box<[u8]>>,
    maybe_sized_wrap: Yoke<MaybeSizedWrap<usize, usize, str>, Box<[u8]>>,
    maybe_sized_wrap_manual: Yoke<ManualMaybeSizedWrap<usize, usize, str>, Box<[u8]>>,
    // TODO(#4119): Make this example compile
    // maybe_sized_wrap_with_lt: Yoke<MaybeSizedWrapWithLifetime<'static, usize, usize, str>, Box<[u8]>>,
}

fn main() {}
