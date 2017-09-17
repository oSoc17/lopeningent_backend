               :// Copyright 2016 Dtoa Developers
               ://
               :// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
               :// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
               :// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
               :// option. This file may not be copied, modified, or distributed
               :// except according to those terms.
               :
               :use std::ops;
               :
               :#[derive(Copy, Clone, Debug)]
               :pub struct DiyFp<F, E> {
               :    pub f: F,
               :    pub e: E,
               :}
               :
               :impl<F, E> DiyFp<F, E> {
               :    pub fn new(f: F, e: E) -> Self {
               :        DiyFp { f: f, e: e }
               :    }
               :}
               :
               :impl ops::Mul for DiyFp<u32, i32> {
               :    type Output = Self;
               :    fn mul(self, rhs: Self) -> Self {
               :        let mut tmp = self.f as u64 * rhs.f as u64;
               :        tmp += 1u64 << 31; // mult_round
               :        DiyFp {
               :            f: (tmp >> 32) as u32,
               :            e: self.e + rhs.e + 32,
               :        }
               :    }
               :}
               :
               :impl ops::Mul for DiyFp<u64, isize> {
               :    type Output = Self;
     2 5.3e-05 :    fn mul(self, rhs: Self) -> Self { /* _$LT$dtoa..diyfp..DiyFp$LT$u64$C$$u20$isize$GT$$u20$as$u20$core..ops..Mul$GT$::mul::h646e9375ca1dd24e total:      7 1.8e-04 */
               :        let m32 = 0xFFFFFFFFu64;
               :        let a = self.f >> 32;
               :        let b = self.f & m32;
               :        let c = rhs.f >> 32;
               :        let d = rhs.f & m32;
     1 2.6e-05 :        let ac = a * c;
               :        let bc = b * c;
               :        let ad = a * d;
               :        let bd = b * d;
     2 5.3e-05 :        let mut tmp = (bd >> 32) + (ad & m32) + (bc & m32);
     1 2.6e-05 :        tmp += 1u64 << 31; // mult_round
               :        DiyFp {
     1 2.6e-05 :            f: ac + (ad >> 32) + (bc >> 32) + (tmp >> 32),
               :            e: self.e + rhs.e + 64,
               :        }
               :    }
               :}
               :
               :#[macro_export]
               :macro_rules! diyfp {(
               :    floating_type: $fty:ty,
               :    significand_type: $sigty:ty,
               :    exponent_type: $expty:ty,
               :
               :    diy_significand_size: $diy_significand_size:expr,
               :    significand_size: $significand_size:expr,
               :    exponent_bias: $exponent_bias:expr,
               :    mask_type: $mask_type:ty,
               :    exponent_mask: $exponent_mask:expr,
               :    significand_mask: $significand_mask:expr,
               :    hidden_bit: $hidden_bit:expr,
               :    cached_powers_f: $cached_powers_f:expr,
               :    cached_powers_e: $cached_powers_e:expr,
               :    min_power: $min_power:expr,
               :) => {
               :
               :type DiyFp = diyfp::DiyFp<$sigty, $expty>;
               :
               :impl DiyFp {
               :    // Preconditions:
               :    // `d` must have a positive sign and must not be infinity or NaN.
               :    /*
               :    explicit DiyFp(double d) {
               :        union {
               :            double d;
               :            uint64_t u64;
               :        } u = { d };
               :
               :        int biased_e = static_cast<int>((u.u64 & kDpExponentMask) >> kDpSignificandSize);
               :        uint64_t significand = (u.u64 & kDpSignificandMask);
               :        if (biased_e != 0) {
               :            f = significand + kDpHiddenBit;
               :            e = biased_e - kDpExponentBias;
               :        }
               :        else {
               :            f = significand;
               :            e = kDpMinExponent + 1;
               :        }
               :    }
               :    */
               :    unsafe fn from(d: $fty) -> Self { /* _$LT$f64$u20$as$u20$dtoa..Floating$GT$::write::_$LT$impl$u20$dtoa..diyfp..DiyFp$LT$u64$C$$u20$isize$GT$$GT$::from::h8400a0f2344f7faf total:      2 5.3e-05 */
               :        let u: $mask_type = mem::transmute(d);
               :
               :        let biased_e = ((u & $exponent_mask) >> $significand_size) as $expty;
               :        let significand = u & $significand_mask;
     2 5.3e-05 :        if biased_e != 0 {
               :            DiyFp {
               :                f: significand + $hidden_bit,
               :                e: biased_e - $exponent_bias - $significand_size,
               :            }
               :        } else {
               :            DiyFp {
               :                f: significand,
               :                e: 1 - $exponent_bias - $significand_size,
               :            }
               :        }
               :    }
               :
               :    // Normalizes so that the highest bit of the diy significand is 1.
               :    /*
               :    DiyFp Normalize() const {
               :        DiyFp res = *this;
               :        while (!(res.f & (static_cast<uint64_t>(1) << 63))) {
               :            res.f <<= 1;
               :            res.e--;
               :        }
               :        return res;
               :    }
               :    */
     2 5.3e-05 :    fn normalize(self) -> DiyFp { /* _$LT$f64$u20$as$u20$dtoa..Floating$GT$::write::_$LT$impl$u20$dtoa..diyfp..DiyFp$LT$u64$C$$u20$isize$GT$$GT$::normalize::h41306b8c6ed005e7 total:      5 1.3e-04 */
               :        let mut res = self;
               :        while (res.f & (1 << ($diy_significand_size - 1))) == 0 {
     3 7.9e-05 :            res.f <<= 1;
               :            res.e -= 1;
               :        }
               :        res
               :    }
               :
               :    // Normalizes so that the highest bit of the diy significand is 1.
               :    //
               :    // Precondition:
               :    // `self.f` must be no more than 2 bits longer than the f64 significand.
               :    /*
               :    DiyFp NormalizeBoundary() const {
               :        DiyFp res = *this;
               :        while (!(res.f & (kDpHiddenBit << 1))) {
               :            res.f <<= 1;
               :            res.e--;
               :        }
               :        res.f <<= (kDiySignificandSize - kDpSignificandSize - 2);
               :        res.e = res.e - (kDiySignificandSize - kDpSignificandSize - 2);
               :        return res;
               :    }
               :    */
               :    fn normalize_boundary(self) -> DiyFp {
               :        let mut res = self;
               :        while (res.f & $hidden_bit << 1) == 0 {
               :            res.f <<= 1;
               :            res.e -= 1;
               :        }
     1 2.6e-05 :        res.f <<= $diy_significand_size - $significand_size - 2;
     1 2.6e-05 :        res.e -= $diy_significand_size - $significand_size - 2;
               :        res
               :    }
               :
               :    // Normalizes `self - e` and `self + e` where `e` is half of the least
               :    // significant digit of `self`. The plus is normalized so that the highest
               :    // bit of the diy significand is 1. The minus is normalized so that it has
               :    // the same exponent as the plus.
               :    //
               :    // Preconditions:
               :    // `self` must have been returned directly from `DiyFp::from_f64`.
               :    // `self.f` must not be zero.
               :    /*
               :    void NormalizedBoundaries(DiyFp* minus, DiyFp* plus) const {
               :        DiyFp pl = DiyFp((f << 1) + 1, e - 1).NormalizeBoundary();
               :        DiyFp mi = (f == kDpHiddenBit) ? DiyFp((f << 2) - 1, e - 2) : DiyFp((f << 1) - 1, e - 1);
               :        mi.f <<= mi.e - pl.e;
               :        mi.e = pl.e;
               :        *plus = pl;
               :        *minus = mi;
               :    }
               :    */
     1 2.6e-05 :    fn normalized_boundaries(self) -> (DiyFp, DiyFp) { /* _$LT$f64$u20$as$u20$dtoa..Floating$GT$::write::_$LT$impl$u20$dtoa..diyfp..DiyFp$LT$u64$C$$u20$isize$GT$$GT$::normalized_boundaries::h345221568f9aefeb total:      5 1.3e-04 */
               :        let pl = DiyFp::new((self.f << 1) + 1, self.e - 1).normalize_boundary();
               :        let mut mi = if self.f == $hidden_bit {
               :            DiyFp::new((self.f << 2) - 1, self.e - 2)
               :        } else {
               :            DiyFp::new((self.f << 1) - 1, self.e - 1)
               :        };
     1 2.6e-05 :        mi.f <<= mi.e - pl.e;
               :        mi.e = pl.e;
     1 2.6e-05 :        (mi, pl)
               :    }
               :}
               :
               :impl ops::Sub for DiyFp {
               :    type Output = Self;
     1 2.6e-05 :    fn sub(self, rhs: Self) -> Self { /* _$LT$f64$u20$as$u20$dtoa..Floating$GT$::write::_$LT$impl$u20$core..ops..Sub$u20$for$u20$dtoa..diyfp..DiyFp$LT$u64$C$$u20$isize$GT$$GT$::sub::hb1cd183d829bf457 total:      2 5.3e-05 */
               :        DiyFp {
     1 2.6e-05 :            f: self.f - rhs.f,
               :            e: self.e,
               :        }
               :    }
               :}
               :
               :/*
               :inline DiyFp GetCachedPower(int e, int* K) {
               :    //int k = static_cast<int>(ceil((-61 - e) * 0.30102999566398114)) + 374;
               :    double dk = (-61 - e) * 0.30102999566398114 + 347;  // dk must be positive, so can do ceiling in positive
               :    int k = static_cast<int>(dk);
               :    if (dk - k > 0.0)
               :        k++;
               :
               :    unsigned index = static_cast<unsigned>((k >> 3) + 1);
               :    *K = -(-348 + static_cast<int>(index << 3));    // decimal exponent no need lookup table
               :
               :    return GetCachedPowerByIndex(index);
               :}
               :*/
               :#[inline]
               :fn get_cached_power(e: $expty) -> (DiyFp, isize) {
     4 1.1e-04 :    let dk = (3 - $diy_significand_size - e) as f64 * 0.30102999566398114f64 - ($min_power + 1) as f64;
               :    let mut k = dk as isize;
     5 1.3e-04 :    if dk - k as f64 > 0.0 {
               :        k += 1;
               :    }
               :
     1 2.6e-05 :    let index = ((k >> 3) + 1) as usize;
               :    let k = -($min_power + (index << 3) as isize);
               :
     1 2.6e-05 :    (DiyFp::new($cached_powers_f[index], $cached_powers_e[index] as $expty), k)
               :}
               :
               :}}
/* 
 * Total samples for file : "/home/gerwin/.cargo/registry/src/github.com-1ecc6299db9ec823/dtoa-0.4.2/src/diyfp.rs"
 * 
 *     32 8.5e-04
 */


/* 
 * Command line: opannotate --source --output-dir=annotations ./target/release/routing_server 
 * 
 * Interpretation of command line:
 * Output annotated source file with samples
 * Output all files
 * 
 * CPU: Intel Ivy Bridge microarchitecture, speed 3100 MHz (estimated)
 * Counted CPU_CLK_UNHALTED events (Clock cycles when not halted) with a unit mask of 0x00 (No unit mask) count 90000
 */
