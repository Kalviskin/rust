// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// asmjs can't even pass i128 as arguments or return values, so ignore it.
// this will hopefully be fixed by the LLVM 5 upgrade (#43370)
// ignore-asmjs
// ignore-emscripten

// compile-flags: -Z lower_128bit_ops=yes -C debug_assertions=no

#![feature(const_fn)]

static TEST_SIGNED: i128 = const_signed(-222);
static TEST_UNSIGNED: u128 = const_unsigned(200);

const fn const_signed(mut x: i128) -> i128 {
    ((((((x + 1) - 2) * 3) / 4) % 5) << 6) >> 7
}

const fn const_unsigned(mut x: u128) -> u128 {
    ((((((x + 1) - 2) * 3) / 4) % 5) << 6) >> 7
}

fn test_signed(mut x: i128) -> i128 {
    x += 1;
    x -= 2;
    x *= 3;
    x /= 4;
    x %= 5;
    x <<= 6;
    x >>= 7;
    x
}

fn test_unsigned(mut x: u128) -> u128 {
    x += 1;
    x -= 2;
    x *= 3;
    x /= 4;
    x %= 5;
    x <<= 6;
    x >>= 7;
    x
}

fn check(x: i128, y: u128) {
    assert_eq!(test_signed(x), -1);
    assert_eq!(const_signed(x), -1);
    assert_eq!(TEST_SIGNED, -1);
    assert_eq!(test_unsigned(y), 2);
    assert_eq!(const_unsigned(y), 2);
    assert_eq!(TEST_UNSIGNED, 2);
}

fn main() {
    check(-222, 200);
}

// END RUST SOURCE

// START rustc.const_signed.Lower128Bit.after.mir
//     _8 = _1;
//     _9 = const compiler_builtins::int::addsub::rust_i128_addo(move _8, const 1i128) -> bb10;
//     ...
//     _7 = move (_9.0: i128);
//     ...
//     _10 = const compiler_builtins::int::addsub::rust_i128_subo(move _7, const 2i128) -> bb11;
//     ...
//     _6 = move (_10.0: i128);
//     ...
//     _11 = const compiler_builtins::int::mul::rust_i128_mulo(move _6, const 3i128) -> bb12;
//     ...
//     _5 = move (_11.0: i128);
//     ...
//     _12 = Eq(const 4i128, const 0i128);
//     assert(!move _12, "attempt to divide by zero") -> bb4;
//     ...
//     _13 = Eq(const 4i128, const -1i128);
//     _14 = Eq(_5, const -170141183460469231731687303715884105728i128);
//     _15 = BitAnd(move _13, move _14);
//     assert(!move _15, "attempt to divide with overflow") -> bb5;
//     ...
//     _4 = const compiler_builtins::int::sdiv::rust_i128_div(move _5, const 4i128) -> bb13;
//     ...
//     _17 = Eq(const 5i128, const -1i128);
//     _18 = Eq(_4, const -170141183460469231731687303715884105728i128);
//     _19 = BitAnd(move _17, move _18);
//     assert(!move _19, "attempt to calculate the remainder with overflow") -> bb7;
//     ...
//     _3 = const compiler_builtins::int::sdiv::rust_i128_rem(move _4, const 5i128) -> bb15;
//     ...
//     _2 = move (_20.0: i128);
//     ...
//     _23 = const 7i32 as u128 (Misc);
//     _21 = const compiler_builtins::int::shift::rust_i128_shro(move _2, move _23) -> bb16;
//     ...
//     _0 = move (_21.0: i128);
//     ...
//     assert(!move (_9.1: bool), "attempt to add with overflow") -> bb1;
//     ...
//     assert(!move (_10.1: bool), "attempt to subtract with overflow") -> bb2;
//     ...
//     assert(!move (_11.1: bool), "attempt to multiply with overflow") -> bb3;
//     ...
//     _16 = Eq(const 5i128, const 0i128);
//     assert(!move _16, "attempt to calculate the remainder with a divisor of zero") -> bb6;
//     ...
//     assert(!move (_20.1: bool), "attempt to shift left with overflow") -> bb8;
//     ...
//     _22 = const 6i32 as u128 (Misc);
//     _20 = const compiler_builtins::int::shift::rust_i128_shlo(move _3, move _22) -> bb14;
//     ...
//     assert(!move (_21.1: bool), "attempt to shift right with overflow") -> bb9;
// END rustc.const_signed.Lower128Bit.after.mir

// START rustc.const_unsigned.Lower128Bit.after.mir
//     _8 = _1;
//     _9 = const compiler_builtins::int::addsub::rust_u128_addo(move _8, const 1u128) -> bb8;
//     ...
//     _7 = move (_9.0: u128);
//     ...
//     _10 = const compiler_builtins::int::addsub::rust_u128_subo(move _7, const 2u128) -> bb9;
//     ...
//     _6 = move (_10.0: u128);
//     ...
//     _11 = const compiler_builtins::int::mul::rust_u128_mulo(move _6, const 3u128) -> bb10;
//     ...
//     _5 = move (_11.0: u128);
//     ...
//     _12 = Eq(const 4u128, const 0u128);
//     assert(!move _12, "attempt to divide by zero") -> bb4;
//     ...
//     _4 = const compiler_builtins::int::udiv::rust_u128_div(move _5, const 4u128) -> bb11;
//     ...
//     _3 = const compiler_builtins::int::udiv::rust_u128_rem(move _4, const 5u128) -> bb13;
//     ...
//     _2 = move (_14.0: u128);
//     ...
//     _17 = const 7i32 as u128 (Misc);
//     _15 = const compiler_builtins::int::shift::rust_u128_shro(move _2, move _17) -> bb14;
//     ...
//     _0 = move (_15.0: u128);
//     ...
//     assert(!move (_9.1: bool), "attempt to add with overflow") -> bb1;
//     ...
//     assert(!move (_10.1: bool), "attempt to subtract with overflow") -> bb2;
//     ...
//     assert(!move (_11.1: bool), "attempt to multiply with overflow") -> bb3;
//     ...
//     _13 = Eq(const 5u128, const 0u128);
//     assert(!move _13, "attempt to calculate the remainder with a divisor of zero") -> bb5;
//     ...
//     assert(!move (_14.1: bool), "attempt to shift left with overflow") -> bb6;
//     ...
//     _16 = const 6i32 as u128 (Misc);
//     _14 = const compiler_builtins::int::shift::rust_u128_shlo(move _3, move _16) -> bb12;
//     ...
//     assert(!move (_15.1: bool), "attempt to shift right with overflow") -> bb7;
// END rustc.const_unsigned.Lower128Bit.after.mir

// START rustc.test_signed.Lower128Bit.after.mir
//     _1 = const compiler_builtins::int::addsub::rust_i128_add(_1, const 1i128) -> bb7;
//     ...
//     _1 = const compiler_builtins::int::sdiv::rust_i128_div(_1, const 4i128) -> bb8;
//     ...
//     _1 = const compiler_builtins::int::sdiv::rust_i128_rem(_1, const 5i128) -> bb11;
//     ...
//     _1 = const compiler_builtins::int::mul::rust_i128_mul(_1, const 3i128) -> bb5;
//     ...
//     _1 = const compiler_builtins::int::addsub::rust_i128_sub(_1, const 2i128) -> bb6;
//     ...
//     _10 = const 7i32 as u32 (Misc);
//     _1 = const compiler_builtins::int::shift::rust_i128_shr(_1, move _10) -> bb9;
//     ...
//     _11 = const 6i32 as u32 (Misc);
//     _1 = const compiler_builtins::int::shift::rust_i128_shl(_1, move _11) -> bb10;
// END rustc.test_signed.Lower128Bit.after.mir

// START rustc.test_unsigned.Lower128Bit.after.mir
//     _1 = const compiler_builtins::int::addsub::rust_u128_add(_1, const 1u128) -> bb5;
//     ...
//     _1 = const compiler_builtins::int::udiv::rust_u128_div(_1, const 4u128) -> bb6;
//     ...
//     _1 = const compiler_builtins::int::udiv::rust_u128_rem(_1, const 5u128) -> bb9;
//     ...
//     _1 = const compiler_builtins::int::mul::rust_u128_mul(_1, const 3u128) -> bb3;
//     ...
//     _1 = const compiler_builtins::int::addsub::rust_u128_sub(_1, const 2u128) -> bb4;
//     ...
//     _4 = const 7i32 as u32 (Misc);
//     _1 = const compiler_builtins::int::shift::rust_u128_shr(_1, move _4) -> bb7;
//     ...
//     _5 = const 6i32 as u32 (Misc);
//     _1 = const compiler_builtins::int::shift::rust_u128_shl(_1, move _5) -> bb8;
// END rustc.test_unsigned.Lower128Bit.after.mir
