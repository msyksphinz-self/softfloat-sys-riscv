// SPDX-License-Identifier: BSD-3-Clause
// See Notices.txt for copyright information
#![no_std]
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

extern crate c99;

use c99::{int_fast32_t, int_fast64_t, uint_fast32_t, uint_fast64_t, uint_fast8_t};

extern "C" {
    pub fn softfloat_detectTininess_read_helper() -> uint_fast8_t;
    pub fn softfloat_detectTininess_write_helper(v: uint_fast8_t);
    pub fn softfloat_roundingMode_read_helper() -> uint_fast8_t;
    pub fn softfloat_roundingMode_write_helper(v: uint_fast8_t);
    pub fn softfloat_exceptionFlags_read_helper() -> uint_fast8_t;
    pub fn softfloat_exceptionFlags_write_helper(v: uint_fast8_t);
    pub fn extF80_roundingPrecision_read_helper() -> uint_fast8_t;
    pub fn extF80_roundingPrecision_write_helper(v: uint_fast8_t);
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct float16_t {
    pub v: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct float32_t {
    pub v: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct float64_t {
    pub v: u64,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct float128_t {
    pub v: [u64; 2],
}

#[cfg(target_endian = "little")]
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct extFloat80M {
    pub signif: u64,
    pub signExp: u16,
}

#[cfg(target_endian = "big")]
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct extFloat80M {
    pub signExp: u16,
    pub signif: u64,
}

pub type extFloat80_t = extFloat80M;

pub const softfloat_tininess_beforeRounding: u8 = 0;
pub const softfloat_tininess_afterRounding: u8 = 1;

pub const softfloat_round_near_even: u8 = 0;
pub const softfloat_round_minMag: u8 = 1;
pub const softfloat_round_min: u8 = 2;
pub const softfloat_round_max: u8 = 3;
pub const softfloat_round_near_maxMag: u8 = 4;
pub const softfloat_round_odd: u8 = 6;

pub const softfloat_flag_inexact: u8 = 1;
pub const softfloat_flag_underflow: u8 = 2;
pub const softfloat_flag_overflow: u8 = 4;
pub const softfloat_flag_infinite: u8 = 8;
pub const softfloat_flag_invalid: u8 = 16;

extern "C" {
    pub fn softfloat_raiseFlags(_: uint_fast8_t);

    pub fn ui32_to_f16(_: u32) -> float16_t;
    pub fn ui32_to_f32(_: u32) -> float32_t;
    pub fn ui32_to_f64(_: u32) -> float64_t;
    pub fn ui32_to_extF80(_: u32) -> extFloat80_t;
    pub fn ui32_to_f128(_: u32) -> float128_t;
    pub fn ui32_to_extF80M(_: u32, _: *mut extFloat80_t);
    pub fn ui32_to_f128M(_: u32, _: *mut float128_t);
    pub fn ui64_to_f16(_: u64) -> float16_t;
    pub fn ui64_to_f32(_: u64) -> float32_t;
    pub fn ui64_to_f64(_: u64) -> float64_t;
    pub fn ui64_to_extF80(_: u64) -> extFloat80_t;
    pub fn ui64_to_f128(_: u64) -> float128_t;

    pub fn ui64_to_extF80M(_: u64, _: *mut extFloat80_t);
    pub fn ui64_to_f128M(_: u64, _: *mut float128_t);
    pub fn i32_to_f16(_: i32) -> float16_t;
    pub fn i32_to_f32(_: i32) -> float32_t;
    pub fn i32_to_f64(_: i32) -> float64_t;
    pub fn i32_to_extF80(_: i32) -> extFloat80_t;
    pub fn i32_to_f128(_: i32) -> float128_t;

    pub fn i32_to_extF80M(_: i32, _: *mut extFloat80_t);
    pub fn i32_to_f128M(_: i32, _: *mut float128_t);
    pub fn i64_to_f16(_: i64) -> float16_t;
    pub fn i64_to_f32(_: i64) -> float32_t;
    pub fn i64_to_f64(_: i64) -> float64_t;
    pub fn i64_to_extF80(_: i64) -> extFloat80_t;
    pub fn i64_to_f128(_: i64) -> float128_t;

    pub fn i64_to_extF80M(_: i64, _: *mut extFloat80_t);
    pub fn i64_to_f128M(_: i64, _: *mut float128_t);

    pub fn f16_to_ui32(_: float16_t, _: uint_fast8_t, _: bool) -> uint_fast32_t;
    pub fn f16_to_ui64(_: float16_t, _: uint_fast8_t, _: bool) -> uint_fast64_t;
    pub fn f16_to_i32(_: float16_t, _: uint_fast8_t, _: bool) -> int_fast32_t;
    pub fn f16_to_i64(_: float16_t, _: uint_fast8_t, _: bool) -> int_fast64_t;
    pub fn f16_to_ui32_r_minMag(_: float16_t, _: bool) -> uint_fast32_t;
    pub fn f16_to_ui64_r_minMag(_: float16_t, _: bool) -> uint_fast64_t;
    pub fn f16_to_i32_r_minMag(_: float16_t, _: bool) -> int_fast32_t;
    pub fn f16_to_i64_r_minMag(_: float16_t, _: bool) -> int_fast64_t;
    pub fn f16_to_f32(_: float16_t) -> float32_t;
    pub fn f16_to_f64(_: float16_t) -> float64_t;
    pub fn f16_to_extF80(_: float16_t) -> extFloat80_t;
    pub fn f16_to_f128(_: float16_t) -> float128_t;

    pub fn f16_to_extF80M(_: float16_t, _: *mut extFloat80_t);
    pub fn f16_to_f128M(_: float16_t, _: *mut float128_t);
    pub fn f16_roundToInt(_: float16_t, _: uint_fast8_t, _: bool) -> float16_t;
    pub fn f16_add(_: float16_t, _: float16_t) -> float16_t;
    pub fn f16_sub(_: float16_t, _: float16_t) -> float16_t;
    pub fn f16_mul(_: float16_t, _: float16_t) -> float16_t;
    pub fn f16_mulAdd(_: float16_t, _: float16_t, _: float16_t) -> float16_t;
    pub fn f16_div(_: float16_t, _: float16_t) -> float16_t;
    pub fn f16_rem(_: float16_t, _: float16_t) -> float16_t;
    pub fn f16_sqrt(_: float16_t) -> float16_t;
    pub fn f16_eq(_: float16_t, _: float16_t) -> bool;
    pub fn f16_le(_: float16_t, _: float16_t) -> bool;
    pub fn f16_lt(_: float16_t, _: float16_t) -> bool;
    pub fn f16_eq_signaling(_: float16_t, _: float16_t) -> bool;
    pub fn f16_le_quiet(_: float16_t, _: float16_t) -> bool;
    pub fn f16_lt_quiet(_: float16_t, _: float16_t) -> bool;
    pub fn f16_isSignalingNaN(_: float16_t) -> bool;

    pub fn f32_to_ui32(_: float32_t, _: uint_fast8_t, _: bool) -> uint_fast32_t;
    pub fn f32_to_ui64(_: float32_t, _: uint_fast8_t, _: bool) -> uint_fast64_t;
    pub fn f32_to_i32(_: float32_t, _: uint_fast8_t, _: bool) -> int_fast32_t;
    pub fn f32_to_i64(_: float32_t, _: uint_fast8_t, _: bool) -> int_fast64_t;
    pub fn f32_to_ui32_r_minMag(_: float32_t, _: bool) -> uint_fast32_t;
    pub fn f32_to_ui64_r_minMag(_: float32_t, _: bool) -> uint_fast64_t;
    pub fn f32_to_i32_r_minMag(_: float32_t, _: bool) -> int_fast32_t;
    pub fn f32_to_i64_r_minMag(_: float32_t, _: bool) -> int_fast64_t;
    pub fn f32_to_f16(_: float32_t) -> float16_t;
    pub fn f32_to_f64(_: float32_t) -> float64_t;
    pub fn f32_to_extF80(_: float32_t) -> extFloat80_t;
    pub fn f32_to_f128(_: float32_t) -> float128_t;

    pub fn f32_to_extF80M(_: float32_t, _: *mut extFloat80_t);
    pub fn f32_to_f128M(_: float32_t, _: *mut float128_t);
    pub fn f32_roundToInt(_: float32_t, _: uint_fast8_t, _: bool) -> float32_t;
    pub fn f32_add(_: float32_t, _: float32_t) -> float32_t;
    pub fn f32_sub(_: float32_t, _: float32_t) -> float32_t;
    pub fn f32_mul(_: float32_t, _: float32_t) -> float32_t;
    pub fn f32_mulAdd(_: float32_t, _: float32_t, _: float32_t) -> float32_t;
    pub fn f32_div(_: float32_t, _: float32_t) -> float32_t;
    pub fn f32_rem(_: float32_t, _: float32_t) -> float32_t;
    pub fn f32_sqrt(_: float32_t) -> float32_t;
    pub fn f32_eq(_: float32_t, _: float32_t) -> bool;
    pub fn f32_le(_: float32_t, _: float32_t) -> bool;
    pub fn f32_lt(_: float32_t, _: float32_t) -> bool;
    pub fn f32_eq_signaling(_: float32_t, _: float32_t) -> bool;
    pub fn f32_le_quiet(_: float32_t, _: float32_t) -> bool;
    pub fn f32_lt_quiet(_: float32_t, _: float32_t) -> bool;
    pub fn f32_isSignalingNaN(_: float32_t) -> bool;

    pub fn f64_to_ui32(_: float64_t, _: uint_fast8_t, _: bool) -> uint_fast32_t;
    pub fn f64_to_ui64(_: float64_t, _: uint_fast8_t, _: bool) -> uint_fast64_t;
    pub fn f64_to_i32(_: float64_t, _: uint_fast8_t, _: bool) -> int_fast32_t;
    pub fn f64_to_i64(_: float64_t, _: uint_fast8_t, _: bool) -> int_fast64_t;
    pub fn f64_to_ui32_r_minMag(_: float64_t, _: bool) -> uint_fast32_t;
    pub fn f64_to_ui64_r_minMag(_: float64_t, _: bool) -> uint_fast64_t;
    pub fn f64_to_i32_r_minMag(_: float64_t, _: bool) -> int_fast32_t;
    pub fn f64_to_i64_r_minMag(_: float64_t, _: bool) -> int_fast64_t;
    pub fn f64_to_f16(_: float64_t) -> float16_t;
    pub fn f64_to_f32(_: float64_t) -> float32_t;
    pub fn f64_to_extF80(_: float64_t) -> extFloat80_t;
    pub fn f64_to_f128(_: float64_t) -> float128_t;

    pub fn f64_to_extF80M(_: float64_t, _: *mut extFloat80_t);
    pub fn f64_to_f128M(_: float64_t, _: *mut float128_t);
    pub fn f64_roundToInt(_: float64_t, _: uint_fast8_t, _: bool) -> float64_t;
    pub fn f64_add(_: float64_t, _: float64_t) -> float64_t;
    pub fn f64_sub(_: float64_t, _: float64_t) -> float64_t;
    pub fn f64_mul(_: float64_t, _: float64_t) -> float64_t;
    pub fn f64_mulAdd(_: float64_t, _: float64_t, _: float64_t) -> float64_t;
    pub fn f64_div(_: float64_t, _: float64_t) -> float64_t;
    pub fn f64_rem(_: float64_t, _: float64_t) -> float64_t;
    pub fn f64_sqrt(_: float64_t) -> float64_t;
    pub fn f64_eq(_: float64_t, _: float64_t) -> bool;
    pub fn f64_le(_: float64_t, _: float64_t) -> bool;
    pub fn f64_lt(_: float64_t, _: float64_t) -> bool;
    pub fn f64_eq_signaling(_: float64_t, _: float64_t) -> bool;
    pub fn f64_le_quiet(_: float64_t, _: float64_t) -> bool;
    pub fn f64_lt_quiet(_: float64_t, _: float64_t) -> bool;
    pub fn f64_isSignalingNaN(_: float64_t) -> bool;

    pub fn extF80_to_ui32(_: extFloat80_t, _: uint_fast8_t, _: bool) -> uint_fast32_t;
    pub fn extF80_to_ui64(_: extFloat80_t, _: uint_fast8_t, _: bool) -> uint_fast64_t;
    pub fn extF80_to_i32(_: extFloat80_t, _: uint_fast8_t, _: bool) -> int_fast32_t;
    pub fn extF80_to_i64(_: extFloat80_t, _: uint_fast8_t, _: bool) -> int_fast64_t;
    pub fn extF80_to_ui32_r_minMag(_: extFloat80_t, _: bool) -> uint_fast32_t;
    pub fn extF80_to_ui64_r_minMag(_: extFloat80_t, _: bool) -> uint_fast64_t;
    pub fn extF80_to_i32_r_minMag(_: extFloat80_t, _: bool) -> int_fast32_t;
    pub fn extF80_to_i64_r_minMag(_: extFloat80_t, _: bool) -> int_fast64_t;
    pub fn extF80_to_f16(_: extFloat80_t) -> float16_t;
    pub fn extF80_to_f32(_: extFloat80_t) -> float32_t;
    pub fn extF80_to_f64(_: extFloat80_t) -> float64_t;
    pub fn extF80_to_f128(_: extFloat80_t) -> float128_t;
    pub fn extF80_roundToInt(_: extFloat80_t, _: uint_fast8_t, _: bool) -> extFloat80_t;
    pub fn extF80_add(_: extFloat80_t, _: extFloat80_t) -> extFloat80_t;
    pub fn extF80_sub(_: extFloat80_t, _: extFloat80_t) -> extFloat80_t;
    pub fn extF80_mul(_: extFloat80_t, _: extFloat80_t) -> extFloat80_t;
    pub fn extF80_div(_: extFloat80_t, _: extFloat80_t) -> extFloat80_t;
    pub fn extF80_rem(_: extFloat80_t, _: extFloat80_t) -> extFloat80_t;
    pub fn extF80_sqrt(_: extFloat80_t) -> extFloat80_t;
    pub fn extF80_eq(_: extFloat80_t, _: extFloat80_t) -> bool;
    pub fn extF80_le(_: extFloat80_t, _: extFloat80_t) -> bool;
    pub fn extF80_lt(_: extFloat80_t, _: extFloat80_t) -> bool;
    pub fn extF80_eq_signaling(_: extFloat80_t, _: extFloat80_t) -> bool;
    pub fn extF80_le_quiet(_: extFloat80_t, _: extFloat80_t) -> bool;
    pub fn extF80_lt_quiet(_: extFloat80_t, _: extFloat80_t) -> bool;
    pub fn extF80_isSignalingNaN(_: extFloat80_t) -> bool;

    pub fn extF80M_to_ui32(_: *const extFloat80_t, _: uint_fast8_t, _: bool) -> uint_fast32_t;
    pub fn extF80M_to_ui64(_: *const extFloat80_t, _: uint_fast8_t, _: bool) -> uint_fast64_t;
    pub fn extF80M_to_i32(_: *const extFloat80_t, _: uint_fast8_t, _: bool) -> int_fast32_t;
    pub fn extF80M_to_i64(_: *const extFloat80_t, _: uint_fast8_t, _: bool) -> int_fast64_t;
    pub fn extF80M_to_ui32_r_minMag(_: *const extFloat80_t, _: bool) -> uint_fast32_t;
    pub fn extF80M_to_ui64_r_minMag(_: *const extFloat80_t, _: bool) -> uint_fast64_t;
    pub fn extF80M_to_i32_r_minMag(_: *const extFloat80_t, _: bool) -> int_fast32_t;
    pub fn extF80M_to_i64_r_minMag(_: *const extFloat80_t, _: bool) -> int_fast64_t;
    pub fn extF80M_to_f16(_: *const extFloat80_t) -> float16_t;
    pub fn extF80M_to_f32(_: *const extFloat80_t) -> float32_t;
    pub fn extF80M_to_f64(_: *const extFloat80_t) -> float64_t;
    pub fn extF80M_to_f128M(_: *const extFloat80_t, _: *mut float128_t);
    pub fn extF80M_roundToInt(
        _: *const extFloat80_t,
        _: uint_fast8_t,
        _: bool,
        _: *mut extFloat80_t,
    );
    pub fn extF80M_add(_: *const extFloat80_t, _: *const extFloat80_t, _: *mut extFloat80_t);
    pub fn extF80M_sub(_: *const extFloat80_t, _: *const extFloat80_t, _: *mut extFloat80_t);
    pub fn extF80M_mul(_: *const extFloat80_t, _: *const extFloat80_t, _: *mut extFloat80_t);
    pub fn extF80M_div(_: *const extFloat80_t, _: *const extFloat80_t, _: *mut extFloat80_t);
    pub fn extF80M_rem(_: *const extFloat80_t, _: *const extFloat80_t, _: *mut extFloat80_t);
    pub fn extF80M_sqrt(_: *const extFloat80_t, _: *mut extFloat80_t);
    pub fn extF80M_eq(_: *const extFloat80_t, _: *const extFloat80_t) -> bool;
    pub fn extF80M_le(_: *const extFloat80_t, _: *const extFloat80_t) -> bool;
    pub fn extF80M_lt(_: *const extFloat80_t, _: *const extFloat80_t) -> bool;
    pub fn extF80M_eq_signaling(_: *const extFloat80_t, _: *const extFloat80_t) -> bool;
    pub fn extF80M_le_quiet(_: *const extFloat80_t, _: *const extFloat80_t) -> bool;
    pub fn extF80M_lt_quiet(_: *const extFloat80_t, _: *const extFloat80_t) -> bool;
    pub fn extF80M_isSignalingNaN(_: *const extFloat80_t) -> bool;

    pub fn f128_to_ui32(_: float128_t, _: uint_fast8_t, _: bool) -> uint_fast32_t;
    pub fn f128_to_ui64(_: float128_t, _: uint_fast8_t, _: bool) -> uint_fast64_t;
    pub fn f128_to_i32(_: float128_t, _: uint_fast8_t, _: bool) -> int_fast32_t;
    pub fn f128_to_i64(_: float128_t, _: uint_fast8_t, _: bool) -> int_fast64_t;
    pub fn f128_to_ui32_r_minMag(_: float128_t, _: bool) -> uint_fast32_t;
    pub fn f128_to_ui64_r_minMag(_: float128_t, _: bool) -> uint_fast64_t;
    pub fn f128_to_i32_r_minMag(_: float128_t, _: bool) -> int_fast32_t;
    pub fn f128_to_i64_r_minMag(_: float128_t, _: bool) -> int_fast64_t;
    pub fn f128_to_f16(_: float128_t) -> float16_t;
    pub fn f128_to_f32(_: float128_t) -> float32_t;
    pub fn f128_to_f64(_: float128_t) -> float64_t;
    pub fn f128_to_extF80(_: float128_t) -> extFloat80_t;
    pub fn f128_roundToInt(_: float128_t, _: uint_fast8_t, _: bool) -> float128_t;
    pub fn f128_add(_: float128_t, _: float128_t) -> float128_t;
    pub fn f128_sub(_: float128_t, _: float128_t) -> float128_t;
    pub fn f128_mul(_: float128_t, _: float128_t) -> float128_t;
    pub fn f128_mulAdd(_: float128_t, _: float128_t, _: float128_t) -> float128_t;
    pub fn f128_div(_: float128_t, _: float128_t) -> float128_t;
    pub fn f128_rem(_: float128_t, _: float128_t) -> float128_t;
    pub fn f128_sqrt(_: float128_t) -> float128_t;
    pub fn f128_eq(_: float128_t, _: float128_t) -> bool;
    pub fn f128_le(_: float128_t, _: float128_t) -> bool;
    pub fn f128_lt(_: float128_t, _: float128_t) -> bool;
    pub fn f128_eq_signaling(_: float128_t, _: float128_t) -> bool;
    pub fn f128_le_quiet(_: float128_t, _: float128_t) -> bool;
    pub fn f128_lt_quiet(_: float128_t, _: float128_t) -> bool;
    pub fn f128_isSignalingNaN(_: float128_t) -> bool;

    pub fn f128M_to_ui32(_: *const float128_t, _: uint_fast8_t, _: bool) -> uint_fast32_t;
    pub fn f128M_to_ui64(_: *const float128_t, _: uint_fast8_t, _: bool) -> uint_fast64_t;
    pub fn f128M_to_i32(_: *const float128_t, _: uint_fast8_t, _: bool) -> int_fast32_t;
    pub fn f128M_to_i64(_: *const float128_t, _: uint_fast8_t, _: bool) -> int_fast64_t;
    pub fn f128M_to_ui32_r_minMag(_: *const float128_t, _: bool) -> uint_fast32_t;
    pub fn f128M_to_ui64_r_minMag(_: *const float128_t, _: bool) -> uint_fast64_t;
    pub fn f128M_to_i32_r_minMag(_: *const float128_t, _: bool) -> int_fast32_t;
    pub fn f128M_to_i64_r_minMag(_: *const float128_t, _: bool) -> int_fast64_t;
    pub fn f128M_to_f16(_: *const float128_t) -> float16_t;
    pub fn f128M_to_f32(_: *const float128_t) -> float32_t;
    pub fn f128M_to_f64(_: *const float128_t) -> float64_t;
    pub fn f128M_to_extF80M(_: *const float128_t, _: *mut extFloat80_t);
    pub fn f128M_roundToInt(_: *const float128_t, _: uint_fast8_t, _: bool, _: *mut float128_t);
    pub fn f128M_add(_: *const float128_t, _: *const float128_t, _: *mut float128_t);
    pub fn f128M_sub(_: *const float128_t, _: *const float128_t, _: *mut float128_t);
    pub fn f128M_mul(_: *const float128_t, _: *const float128_t, _: *mut float128_t);
    pub fn f128M_mulAdd(
        _: *const float128_t,
        _: *const float128_t,
        _: *const float128_t,
        _: *mut float128_t,
    );
    pub fn f128M_div(_: *const float128_t, _: *const float128_t, _: *mut float128_t);
    pub fn f128M_rem(_: *const float128_t, _: *const float128_t, _: *mut float128_t);
    pub fn f128M_sqrt(_: *const float128_t, _: *mut float128_t);
    pub fn f128M_eq(_: *const float128_t, _: *const float128_t) -> bool;
    pub fn f128M_le(_: *const float128_t, _: *const float128_t) -> bool;
    pub fn f128M_lt(_: *const float128_t, _: *const float128_t) -> bool;
    pub fn f128M_eq_signaling(_: *const float128_t, _: *const float128_t) -> bool;
    pub fn f128M_le_quiet(_: *const float128_t, _: *const float128_t) -> bool;
    pub fn f128M_lt_quiet(_: *const float128_t, _: *const float128_t) -> bool;
    pub fn f128M_isSignalingNaN(_: *const float128_t) -> bool;
}
