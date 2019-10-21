use super::*;

cfg_if! {
  if #[cfg(target_feature="sse")] {
    #[repr(C, align(16))]
    pub struct f32x4 {
      sse: m128
    }
  } else {
    #[repr(C, align(16))]
    pub struct f32x4 {
      arr: [f32; 4]
    }
  }
}
#[test]
fn declaration_tests() {
  use core::mem::{align_of, size_of};
  assert_eq!(size_of::<f32x4>(), 16);
  assert_eq!(align_of::<f32x4>(), 16);
}

impl f32x4 {
  #[inline(always)]
  pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
    cfg_if! {if #[cfg(target_feature="sse")] {
      Self { sse: m128::set(a,b,c,d) }
    } else {
      Self { arr: [a,b,c,d] }
    }}
  }
}

impl From<f32> for f32x4 {
  #[inline]
  fn from(val: f32) -> Self {
    cfg_if! {if #[cfg(target_feature="sse")] {
      Self { sse: m128::splat(val) }
    } else {
      Self::new(val,val,val,val)
    }}
  }
}

impl Add for f32x4 {
  type Output = Self;
  #[inline]
  fn add(self, rhs: Self) -> Self {
    cfg_if! {if #[cfg(target_feature="sse")] {
      Self { sse: self.sse.add(rhs.sse) }
    } else {
      Self { arr: [
        self.arr[0] + rhs.arr[0],
        self.arr[1] + rhs.arr[1],
        self.arr[2] + rhs.arr[2],
        self.arr[3] + rhs.arr[3],
      ] }
    }}
  }
}

impl Add<&'_ f32x4> for f32x4 {
  type Output = Self;
  #[inline]
  fn add(self, rhs: &Self) -> Self {
    cfg_if! {if #[cfg(target_feature="sse")] {
      Self { sse: self.sse.add(rhs.sse) }
    } else {
      Self { arr: [
        self.arr[0] + rhs.arr[0],
        self.arr[1] + rhs.arr[1],
        self.arr[2] + rhs.arr[2],
        self.arr[3] + rhs.arr[3],
      ] }
    }}
  }
}

impl Div for f32x4 {
  type Output = Self;
  #[inline]
  fn div(self, rhs: Self) -> Self {
    cfg_if! {if #[cfg(target_feature="sse")] {
      Self { sse: self.sse.div(rhs.sse) }
    } else {
      Self { arr: [
        self.arr[0] / rhs.arr[0],
        self.arr[1] / rhs.arr[1],
        self.arr[2] / rhs.arr[2],
        self.arr[3] / rhs.arr[3],
      ] }
    }}
  }
}

impl Div<&'_ f32x4> for f32x4 {
  type Output = Self;
  #[inline]
  fn div(self, rhs: &Self) -> Self {
    cfg_if! {if #[cfg(target_feature="sse")] {
      Self { sse: self.sse.div(rhs.sse) }
    } else {
      Self { arr: [
        self.arr[0] / rhs.arr[0],
        self.arr[1] / rhs.arr[1],
        self.arr[2] / rhs.arr[2],
        self.arr[3] / rhs.arr[3],
      ] }
    }}
  }
}

impl Mul for f32x4 {
  type Output = Self;
  #[inline]
  fn mul(self, rhs: Self) -> Self {
    cfg_if! {if #[cfg(target_feature="sse")] {
      Self { sse: self.sse.mul(rhs.sse) }
    } else {
      Self { arr: [
        self.arr[0] * rhs.arr[0],
        self.arr[1] * rhs.arr[1],
        self.arr[2] * rhs.arr[2],
        self.arr[3] * rhs.arr[3],
      ] }
    }}
  }
}

impl Mul<&'_ f32x4> for f32x4 {
  type Output = Self;
  #[inline]
  fn mul(self, rhs: &Self) -> Self {
    cfg_if! {if #[cfg(target_feature="sse")] {
      Self { sse: self.sse.mul(rhs.sse) }
    } else {
      Self { arr: [
        self.arr[0] * rhs.arr[0],
        self.arr[1] * rhs.arr[1],
        self.arr[2] * rhs.arr[2],
        self.arr[3] * rhs.arr[3],
      ] }
    }}
  }
}

impl Rem for f32x4 {
  type Output = Self;
  #[inline]
  fn rem(self, rhs: Self) -> Self {
    cfg_if! {if #[cfg(target_feature="sse")] {
      let arr1: [f32; 4] = cast(self.sse);
      let arr2: [f32; 4] = cast(rhs.sse);
      Self { sse: cast([
        arr1[0] % arr2[0],
        arr1[1] % arr2[1],
        arr1[2] % arr2[2],
        arr1[3] % arr2[3],
      ]) }
    } else {
      Self { arr: [
        self.arr[0] % rhs.arr[0],
        self.arr[1] % rhs.arr[1],
        self.arr[2] % rhs.arr[2],
        self.arr[3] % rhs.arr[3],
      ] }
    }}
  }
}

impl Rem<&'_ f32x4> for f32x4 {
  type Output = Self;
  #[inline]
  fn rem(self, rhs: &Self) -> Self {
    cfg_if! {if #[cfg(target_feature="sse")] {
      let arr1: [f32; 4] = cast(self.sse);
      let arr2: [f32; 4] = cast(rhs.sse);
      Self { sse: cast([
        arr1[0] % arr2[0],
        arr1[1] % arr2[1],
        arr1[2] % arr2[2],
        arr1[3] % arr2[3],
      ]) }
    } else {
      Self { arr: [
        self.arr[0] % rhs.arr[0],
        self.arr[1] % rhs.arr[1],
        self.arr[2] % rhs.arr[2],
        self.arr[3] % rhs.arr[3],
      ] }
    }}
  }
}

impl Sub for f32x4 {
  type Output = Self;
  #[inline]
  fn sub(self, rhs: Self) -> Self {
    cfg_if! {if #[cfg(target_feature="sse")] {
      Self { sse: self.sse.sub(rhs.sse) }
    } else {
      Self { arr: [
        self.arr[0] - rhs.arr[0],
        self.arr[1] - rhs.arr[1],
        self.arr[2] - rhs.arr[2],
        self.arr[3] - rhs.arr[3],
      ] }
    }}
  }
}

impl Sub<&'_ f32x4> for f32x4 {
  type Output = Self;
  #[inline]
  fn sub(self, rhs: &Self) -> Self {
    cfg_if! {if #[cfg(target_feature="sse")] {
      Self { sse: self.sse.sub(rhs.sse) }
    } else {
      Self { arr: [
        self.arr[0] - rhs.arr[0],
        self.arr[1] - rhs.arr[1],
        self.arr[2] - rhs.arr[2],
        self.arr[3] - rhs.arr[3],
      ] }
    }}
  }
}

impl f32x4 {
  #[inline]
  pub fn abs(self) -> Self {
    let a: [f32; 4] = cast(self);
    cast([a[0].abs(), a[1].abs(), a[2].abs(), a[3].abs()])
  }

  #[inline]
  pub fn acos(self) -> Self {
    let a: [f32; 4] = cast(self);
    cast([a[0].acos(), a[1].acos(), a[2].acos(), a[3].acos()])
  }

  #[inline]
  pub fn acosh(self) -> Self {
    let a: [f32; 4] = cast(self);
    cast([a[0].acosh(), a[1].acosh(), a[2].acosh(), a[3].acosh()])
  }

  #[inline]
  pub fn asin(self) -> Self {
    let a: [f32; 4] = cast(self);
    cast([a[0].asin(), a[1].asin(), a[2].asin(), a[3].asin()])
  }

  #[inline]
  pub fn asinh(self) -> Self {
    let a: [f32; 4] = cast(self);
    cast([a[0].asinh(), a[1].asinh(), a[2].asinh(), a[3].asinh()])
  }

  #[inline]
  pub fn atan(self) -> Self {
    let a: [f32; 4] = cast(self);
    cast([a[0].atan(), a[1].atan(), a[2].atan(), a[3].atan()])
  }

  #[inline]
  pub fn atanh(self) -> Self {
    let a: [f32; 4] = cast(self);
    cast([a[0].atanh(), a[1].atanh(), a[2].atanh(), a[3].atanh()])
  }

  #[inline]
  pub fn cbrt(self) -> Self {
    let a: [f32; 4] = cast(self);
    cast([a[0].cbrt(), a[1].cbrt(), a[2].cbrt(), a[3].cbrt()])
  }

  #[inline]
  pub fn ceil(self) -> Self {
    let a: [f32; 4] = cast(self);
    cast([a[0].ceil(), a[1].ceil(), a[2].ceil(), a[3].ceil()])
  }

  #[inline]
  pub fn classify(self) -> [std::num::FpCategory; 4] {
    let a: [f32; 4] = cast(self);
    [
      a[0].classify(),
      a[1].classify(),
      a[2].classify(),
      a[3].classify(),
    ]
  }

  #[inline]
  pub fn cos(self) -> Self {
    let a: [f32; 4] = cast(self);
    cast([a[0].cos(), a[1].cos(), a[2].cos(), a[3].cos()])
  }

  #[inline]
  pub fn cosh(self) -> Self {
    let a: [f32; 4] = cast(self);
    cast([a[0].cosh(), a[1].cosh(), a[2].cosh(), a[3].cosh()])
  }

  #[inline]
  pub fn exp(self) -> Self {
    let a: [f32; 4] = cast(self);
    cast([a[0].exp(), a[1].exp(), a[2].exp(), a[3].exp()])
  }

  #[inline]
  pub fn exp2(self) -> Self {
    let a: [f32; 4] = cast(self);
    cast([a[0].exp2(), a[1].exp2(), a[2].exp2(), a[3].exp2()])
  }

  #[inline]
  pub fn exp_m1(self) -> Self {
    let a: [f32; 4] = cast(self);
    cast([a[0].exp_m1(), a[1].exp_m1(), a[2].exp_m1(), a[3].exp_m1()])
  }

  #[inline]
  pub fn floor(self) -> Self {
    let a: [f32; 4] = cast(self);
    cast([a[0].floor(), a[1].floor(), a[2].floor(), a[3].floor()])
  }

  #[inline]
  pub fn fract(self) -> Self {
    let a: [f32; 4] = cast(self);
    cast([a[0].fract(), a[1].fract(), a[2].fract(), a[3].fract()])
  }

  #[inline]
  pub fn ln(self) -> Self {
    let a: [f32; 4] = cast(self);
    cast([a[0].ln(), a[1].ln(), a[2].ln(), a[3].ln()])
  }

  #[inline]
  pub fn ln_1p(self) -> Self {
    let a: [f32; 4] = cast(self);
    cast([a[0].ln_1p(), a[1].ln_1p(), a[2].ln_1p(), a[3].ln_1p()])
  }

  #[inline]
  pub fn log10(self) -> Self {
    let a: [f32; 4] = cast(self);
    cast([a[0].log10(), a[1].log10(), a[2].log10(), a[3].log10()])
  }

  #[inline]
  pub fn log2(self) -> Self {
    let a: [f32; 4] = cast(self);
    cast([a[0].log2(), a[1].log2(), a[2].log2(), a[3].log2()])
  }

  #[inline]
  pub fn recip(self) -> Self {
    let a: [f32; 4] = cast(self);
    cast([a[0].recip(), a[1].recip(), a[2].recip(), a[3].recip()])
  }

  #[inline]
  pub fn round(self) -> Self {
    let a: [f32; 4] = cast(self);
    cast([a[0].round(), a[1].round(), a[2].round(), a[3].round()])
  }

  #[inline]
  pub fn signum(self) -> Self {
    let a: [f32; 4] = cast(self);
    cast([a[0].signum(), a[1].signum(), a[2].signum(), a[3].signum()])
  }

  #[inline]
  pub fn sin(self) -> Self {
    let a: [f32; 4] = cast(self);
    cast([a[0].sin(), a[1].sin(), a[2].sin(), a[3].sin()])
  }

  #[inline]
  pub fn sinh(self) -> Self {
    let a: [f32; 4] = cast(self);
    cast([a[0].sinh(), a[1].sinh(), a[2].sinh(), a[3].sinh()])
  }

  #[inline]
  pub fn sqrt(self) -> Self {
    let a: [f32; 4] = cast(self);
    cast([a[0].sqrt(), a[1].sqrt(), a[2].sqrt(), a[3].sqrt()])
  }

  #[inline]
  pub fn tan(self) -> Self {
    let a: [f32; 4] = cast(self);
    cast([a[0].tan(), a[1].tan(), a[2].tan(), a[3].tan()])
  }

  #[inline]
  pub fn tanh(self) -> Self {
    let a: [f32; 4] = cast(self);
    cast([a[0].tanh(), a[1].tanh(), a[2].tanh(), a[3].tanh()])
  }

  #[inline]
  pub fn to_degrees(self) -> Self {
    let a: [f32; 4] = cast(self);
    cast([
      a[0].to_degrees(),
      a[1].to_degrees(),
      a[2].to_degrees(),
      a[3].to_degrees(),
    ])
  }

  #[inline]
  pub fn to_radians(self) -> Self {
    let a: [f32; 4] = cast(self);
    cast([
      a[0].to_radians(),
      a[1].to_radians(),
      a[2].to_radians(),
      a[3].to_radians(),
    ])
  }

  #[inline]
  pub fn trunc(self) -> Self {
    let a: [f32; 4] = cast(self);
    cast([a[0].trunc(), a[1].trunc(), a[2].trunc(), a[3].trunc()])
  }

  #[inline]
  pub fn atan2(self, b: Self) -> Self {
    let a: [f32; 4] = cast(self);
    let b: [f32; 4] = cast(b);
    cast([
      a[0].atan2(b[0]),
      a[1].atan2(b[1]),
      a[2].atan2(b[2]),
      a[3].atan2(b[3]),
    ])
  }

  #[inline]
  pub fn copysign(self, b: Self) -> Self {
    let a: [f32; 4] = cast(self);
    let b: [f32; 4] = cast(b);
    cast([
      a[0].copysign(b[0]),
      a[1].copysign(b[1]),
      a[2].copysign(b[2]),
      a[3].copysign(b[3]),
    ])
  }

  #[inline]
  pub fn hypot(self, b: Self) -> Self {
    let a: [f32; 4] = cast(self);
    let b: [f32; 4] = cast(b);
    cast([
      a[0].hypot(b[0]),
      a[1].hypot(b[1]),
      a[2].hypot(b[2]),
      a[3].hypot(b[3]),
    ])
  }

  #[inline]
  pub fn log(self, b: Self) -> Self {
    let a: [f32; 4] = cast(self);
    let b: [f32; 4] = cast(b);
    cast([
      a[0].log(b[0]),
      a[1].log(b[1]),
      a[2].log(b[2]),
      a[3].log(b[3]),
    ])
  }

  #[inline]
  pub fn max(self, b: Self) -> Self {
    let a: [f32; 4] = cast(self);
    let b: [f32; 4] = cast(b);
    cast([
      a[0].max(b[0]),
      a[1].max(b[1]),
      a[2].max(b[2]),
      a[3].max(b[3]),
    ])
  }

  #[inline]
  pub fn min(self, b: Self) -> Self {
    let a: [f32; 4] = cast(self);
    let b: [f32; 4] = cast(b);
    cast([
      a[0].min(b[0]),
      a[1].min(b[1]),
      a[2].min(b[2]),
      a[3].min(b[3]),
    ])
  }

  #[inline]
  pub fn powf(self, b: Self) -> Self {
    let a: [f32; 4] = cast(self);
    let b: [f32; 4] = cast(b);
    cast([
      a[0].powf(b[0]),
      a[1].powf(b[1]),
      a[2].powf(b[2]),
      a[3].powf(b[3]),
    ])
  }

  #[inline]
  pub fn powi(self, b: [i32; 4]) -> Self {
    let a: [f32; 4] = cast(self);
    cast([
      a[0].powi(b[0]),
      a[1].powi(b[1]),
      a[2].powi(b[2]),
      a[3].powi(b[3]),
    ])
  }

  #[inline]
  pub fn mul_add(self, b: Self, c: Self) -> Self {
    let a: [f32; 4] = cast(self);
    let b: [f32; 4] = cast(b);
    let c: [f32; 4] = cast(c);
    cast([
      a[0].mul_add(b[0], c[0]),
      a[1].mul_add(b[1], c[1]),
      a[2].mul_add(b[2], c[2]),
      a[3].mul_add(b[3], c[3]),
    ])
  }

  #[inline]
  pub fn sin_cos(self) -> (Self, Self) {
    let a: [f32; 4] = cast(self);
    let (zero_sin, zero_cos) = a[0].sin_cos();
    let (one_sin, one_cos) = a[1].sin_cos();
    let (two_sin, two_cos) = a[2].sin_cos();
    let (three_sin, three_cos) = a[3].sin_cos();
    (
      cast([zero_sin, one_sin, two_sin, three_sin]),
      cast([zero_cos, one_cos, two_cos, three_cos]),
    )
  }
}

// // //
// CODE AFTER HERE SHOULD NOT USE CONDITIONAL COMPILATION!
// // //
// (We want to keep per-platform code away from the universal code)
// // //

impl Clone for f32x4 {
  #[inline(always)]
  fn clone(&self) -> Self {
    *self
  }
}
impl Copy for f32x4 {}
impl Default for f32x4 {
  #[inline(always)]
  fn default() -> Self {
    Self::zeroed()
  }
}
unsafe impl Zeroable for f32x4 {}
unsafe impl Pod for f32x4 {}

impl Index<usize> for f32x4 {
  type Output = f32;
  #[inline(always)]
  fn index(&self, index: usize) -> &f32 {
    let r: &[f32; 4] = cast_ref(self);
    &r[index]
  }
}
impl IndexMut<usize> for f32x4 {
  #[inline(always)]
  fn index_mut(&mut self, index: usize) -> &mut f32 {
    let r: &mut [f32; 4] = cast_mut(self);
    &mut r[index]
  }
}

impl AddAssign for f32x4 {
  #[inline]
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs
  }
}

impl AddAssign<&'_ f32x4> for f32x4 {
  #[inline]
  fn add_assign(&mut self, rhs: &Self) {
    *self = *self + rhs
  }
}

impl DivAssign for f32x4 {
  #[inline]
  fn div_assign(&mut self, rhs: Self) {
    *self = *self / rhs
  }
}

impl DivAssign<&'_ f32x4> for f32x4 {
  #[inline]
  fn div_assign(&mut self, rhs: &Self) {
    *self = *self / rhs
  }
}

impl MulAssign for f32x4 {
  #[inline]
  fn mul_assign(&mut self, rhs: Self) {
    *self = *self * rhs
  }
}

impl MulAssign<&'_ f32x4> for f32x4 {
  #[inline]
  fn mul_assign(&mut self, rhs: &Self) {
    *self = *self * rhs
  }
}

impl RemAssign for f32x4 {
  #[inline]
  fn rem_assign(&mut self, rhs: Self) {
    *self = *self % rhs
  }
}

impl RemAssign<&'_ f32x4> for f32x4 {
  #[inline]
  fn rem_assign(&mut self, rhs: &Self) {
    *self = *self % rhs
  }
}

impl SubAssign for f32x4 {
  #[inline]
  fn sub_assign(&mut self, rhs: Self) {
    *self = *self - rhs
  }
}

impl SubAssign<&'_ f32x4> for f32x4 {
  #[inline]
  fn sub_assign(&mut self, rhs: &Self) {
    *self = *self - rhs
  }
}

impl Debug for f32x4 {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "f32x4(")?;
    Debug::fmt(&self[0], f)?;
    Debug::fmt(&self[1], f)?;
    Debug::fmt(&self[2], f)?;
    Debug::fmt(&self[3], f)?;
    write!(f, ")")
  }
}

impl Display for f32x4 {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "f32x4(")?;
    Display::fmt(&self[0], f)?;
    Display::fmt(&self[1], f)?;
    Display::fmt(&self[2], f)?;
    Display::fmt(&self[3], f)?;
    write!(f, ")")
  }
}

impl LowerExp for f32x4 {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "f32x4(")?;
    LowerExp::fmt(&self[0], f)?;
    LowerExp::fmt(&self[1], f)?;
    LowerExp::fmt(&self[2], f)?;
    LowerExp::fmt(&self[3], f)?;
    write!(f, ")")
  }
}

impl UpperExp for f32x4 {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "f32x4(")?;
    UpperExp::fmt(&self[0], f)?;
    UpperExp::fmt(&self[1], f)?;
    UpperExp::fmt(&self[2], f)?;
    UpperExp::fmt(&self[3], f)?;
    write!(f, ")")
  }
}

impl Neg for f32x4 {
  type Output = f32x4;
  #[inline]
  fn neg(self) -> f32x4 {
    f32x4::new(0.0, 0.0, 0.0, 0.0) - self
  }
}
impl Neg for &'_ f32x4 {
  type Output = f32x4;
  #[inline]
  fn neg(self) -> f32x4 {
    f32x4::new(0.0, 0.0, 0.0, 0.0) - self
  }
}

impl core::iter::Product for f32x4 {
  #[inline]
  fn product<I: Iterator<Item = f32x4>>(iter: I) -> f32x4 {
    let mut total = f32x4::new(1.0, 1.0, 1.0, 1.0);
    for i in iter {
      total *= i;
    }
    total
  }
}
impl<'a> core::iter::Product<&'a f32x4> for f32x4 {
  #[inline]
  fn product<I: Iterator<Item = &'a f32x4>>(iter: I) -> f32x4 {
    let mut total = f32x4::new(1.0, 1.0, 1.0, 1.0);
    for i in iter {
      total *= i;
    }
    total
  }
}

impl core::iter::Sum for f32x4 {
  #[inline]
  fn sum<I: Iterator<Item = f32x4>>(iter: I) -> f32x4 {
    let mut total = f32x4::new(0.0, 0.0, 0.0, 0.0);
    for i in iter {
      total += i;
    }
    total
  }
}
impl<'a> core::iter::Sum<&'a f32x4> for f32x4 {
  #[inline]
  fn sum<I: Iterator<Item = &'a f32x4>>(iter: I) -> f32x4 {
    let mut total = f32x4::new(0.0, 0.0, 0.0, 0.0);
    for i in iter {
      total += i;
    }
    total
  }
}

impl AsRef<[f32; 4]> for f32x4 {
  #[inline(always)]
  fn as_ref(&self) -> &[f32; 4] {
    cast_ref(self)
  }
}
impl AsMut<[f32; 4]> for f32x4 {
  #[inline(always)]
  fn as_mut(&mut self) -> &mut [f32; 4] {
    cast_mut(self)
  }
}

impl From<[f32; 4]> for f32x4 {
  #[inline(always)]
  fn from(arr: [f32; 4]) -> Self {
    cast(arr)
  }
}
impl From<(f32, f32, f32, f32)> for f32x4 {
  #[inline(always)]
  fn from((a, b, c, d): (f32, f32, f32, f32)) -> Self {
    Self::new(a, b, c, d)
  }
}

impl From<[i8; 4]> for f32x4 {
  #[inline]
  fn from([a, b, c, d]: [i8; 4]) -> Self {
    Self::new(f32::from(a), f32::from(b), f32::from(c), f32::from(d))
  }
}

impl From<[u8; 4]> for f32x4 {
  #[inline]
  fn from([a, b, c, d]: [u8; 4]) -> Self {
    Self::new(f32::from(a), f32::from(b), f32::from(c), f32::from(d))
  }
}

impl From<[i16; 4]> for f32x4 {
  #[inline]
  fn from([a, b, c, d]: [i16; 4]) -> Self {
    Self::new(f32::from(a), f32::from(b), f32::from(c), f32::from(d))
  }
}

impl From<[u16; 4]> for f32x4 {
  #[inline]
  fn from([a, b, c, d]: [u16; 4]) -> Self {
    Self::new(f32::from(a), f32::from(b), f32::from(c), f32::from(d))
  }
}

/// Various `f32` related consts, duplicated into x4 array form.
///
/// Rust doesn't let you declare SIMD values in a `const` context, so you have
/// to use something like `let c = f32x4::from(CONST_NAME);`
pub mod consts {
  pub const EPSILON: [f32; 4] = [
    std::f32::EPSILON,
    std::f32::EPSILON,
    std::f32::EPSILON,
    std::f32::EPSILON,
  ];
  pub const INFINITY: [f32; 4] = [
    std::f32::INFINITY,
    std::f32::INFINITY,
    std::f32::INFINITY,
    std::f32::INFINITY,
  ];
  pub const MAX: [f32; 4] = [std::f32::MAX, std::f32::MAX, std::f32::MAX, std::f32::MAX];
  pub const MIN: [f32; 4] = [std::f32::MIN, std::f32::MIN, std::f32::MIN, std::f32::MIN];
  pub const MIN_POSITIVE: [f32; 4] = [
    std::f32::MIN_POSITIVE,
    std::f32::MIN_POSITIVE,
    std::f32::MIN_POSITIVE,
    std::f32::MIN_POSITIVE,
  ];
  pub const NAN: [f32; 4] = [std::f32::NAN, std::f32::NAN, std::f32::NAN, std::f32::NAN];
  pub const NEG_INFINITY: [f32; 4] = [
    std::f32::NEG_INFINITY,
    std::f32::NEG_INFINITY,
    std::f32::NEG_INFINITY,
    std::f32::NEG_INFINITY,
  ];
  pub const DIGITS: [u32; 4] = [
    std::f32::DIGITS,
    std::f32::DIGITS,
    std::f32::DIGITS,
    std::f32::DIGITS,
  ];
  pub const MANTISSA_DIGITS: [u32; 4] = [
    std::f32::MANTISSA_DIGITS,
    std::f32::MANTISSA_DIGITS,
    std::f32::MANTISSA_DIGITS,
    std::f32::MANTISSA_DIGITS,
  ];
  pub const RADIX: [u32; 4] = [
    std::f32::RADIX,
    std::f32::RADIX,
    std::f32::RADIX,
    std::f32::RADIX,
  ];
  pub const MAX_10_EXP: [i32; 4] = [
    std::f32::MAX_10_EXP,
    std::f32::MAX_10_EXP,
    std::f32::MAX_10_EXP,
    std::f32::MAX_10_EXP,
  ];
  pub const MAX_EXP: [i32; 4] = [
    std::f32::MAX_EXP,
    std::f32::MAX_EXP,
    std::f32::MAX_EXP,
    std::f32::MAX_EXP,
  ];
  pub const MIN_10_EXP: [i32; 4] = [
    std::f32::MIN_10_EXP,
    std::f32::MIN_10_EXP,
    std::f32::MIN_10_EXP,
    std::f32::MIN_10_EXP,
  ];
  pub const MIN_EXP: [i32; 4] = [
    std::f32::MIN_EXP,
    std::f32::MIN_EXP,
    std::f32::MIN_EXP,
    std::f32::MIN_EXP,
  ];
  pub const E: [f32; 4] = [
    std::f32::consts::E,
    std::f32::consts::E,
    std::f32::consts::E,
    std::f32::consts::E,
  ];
  pub const FRAC_1_PI: [f32; 4] = [
    std::f32::consts::FRAC_1_PI,
    std::f32::consts::FRAC_1_PI,
    std::f32::consts::FRAC_1_PI,
    std::f32::consts::FRAC_1_PI,
  ];
  pub const FRAC_2_PI: [f32; 4] = [
    std::f32::consts::FRAC_2_PI,
    std::f32::consts::FRAC_2_PI,
    std::f32::consts::FRAC_2_PI,
    std::f32::consts::FRAC_2_PI,
  ];
  pub const FRAC_2_SQRT_PI: [f32; 4] = [
    std::f32::consts::FRAC_2_SQRT_PI,
    std::f32::consts::FRAC_2_SQRT_PI,
    std::f32::consts::FRAC_2_SQRT_PI,
    std::f32::consts::FRAC_2_SQRT_PI,
  ];
  pub const FRAC_1_SQRT_2: [f32; 4] = [
    std::f32::consts::FRAC_1_SQRT_2,
    std::f32::consts::FRAC_1_SQRT_2,
    std::f32::consts::FRAC_1_SQRT_2,
    std::f32::consts::FRAC_1_SQRT_2,
  ];
  pub const FRAC_PI_2: [f32; 4] = [
    std::f32::consts::FRAC_PI_2,
    std::f32::consts::FRAC_PI_2,
    std::f32::consts::FRAC_PI_2,
    std::f32::consts::FRAC_PI_2,
  ];
  pub const FRAC_PI_3: [f32; 4] = [
    std::f32::consts::FRAC_PI_3,
    std::f32::consts::FRAC_PI_3,
    std::f32::consts::FRAC_PI_3,
    std::f32::consts::FRAC_PI_3,
  ];
  pub const FRAC_PI_4: [f32; 4] = [
    std::f32::consts::FRAC_PI_4,
    std::f32::consts::FRAC_PI_4,
    std::f32::consts::FRAC_PI_4,
    std::f32::consts::FRAC_PI_4,
  ];
  pub const FRAC_PI_6: [f32; 4] = [
    std::f32::consts::FRAC_PI_6,
    std::f32::consts::FRAC_PI_6,
    std::f32::consts::FRAC_PI_6,
    std::f32::consts::FRAC_PI_6,
  ];
  pub const FRAC_PI_8: [f32; 4] = [
    std::f32::consts::FRAC_PI_8,
    std::f32::consts::FRAC_PI_8,
    std::f32::consts::FRAC_PI_8,
    std::f32::consts::FRAC_PI_8,
  ];
  pub const LN_2: [f32; 4] = [
    std::f32::consts::LN_2,
    std::f32::consts::LN_2,
    std::f32::consts::LN_2,
    std::f32::consts::LN_2,
  ];
  pub const LN_10: [f32; 4] = [
    std::f32::consts::LN_10,
    std::f32::consts::LN_10,
    std::f32::consts::LN_10,
    std::f32::consts::LN_10,
  ];
  pub const LOG2_E: [f32; 4] = [
    std::f32::consts::LOG2_E,
    std::f32::consts::LOG2_E,
    std::f32::consts::LOG2_E,
    std::f32::consts::LOG2_E,
  ];
  pub const LOG10_E: [f32; 4] = [
    std::f32::consts::LOG10_E,
    std::f32::consts::LOG10_E,
    std::f32::consts::LOG10_E,
    std::f32::consts::LOG10_E,
  ];
  pub const PI: [f32; 4] = [
    std::f32::consts::PI,
    std::f32::consts::PI,
    std::f32::consts::PI,
    std::f32::consts::PI,
  ];
  pub const SQRT_2: [f32; 4] = [
    std::f32::consts::SQRT_2,
    std::f32::consts::SQRT_2,
    std::f32::consts::SQRT_2,
    std::f32::consts::SQRT_2,
  ];
}
