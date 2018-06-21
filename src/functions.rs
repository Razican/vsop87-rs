//! Intrinsics module to compute some functions for non-std targets.

use core::intrinsics;

/// Trigonometry functions to calculate VSOP-87. No-std friendly.
pub trait Trigonometry {
    /// Computes the cosine of a number (in radians).
    ///
    /// ```
    /// use core::f64;
    ///
    /// let x = 2.0*f64::consts::PI;
    ///
    /// let abs_difference = (x.cos() - 1.0).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    fn cos(self) -> Self;

    /// Computes the sine of a number (in radians).
    ///
    /// ```
    /// use core::f64;
    ///
    /// let x = f64::consts::PI/2.0;
    ///
    /// let abs_difference = (x.sin() - 1.0).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    fn sin(self) -> Self;

    /// Simultaneously computes the sine and cosine of the number, `x`. Returns
    /// `(sin(x), cos(x))`.
    ///
    /// ```
    /// use std::f64;
    ///
    /// let x = f64::consts::PI/4.0;
    /// let f = x.sin_cos();
    ///
    /// let abs_difference_0 = (f.0 - x.sin()).abs();
    /// let abs_difference_1 = (f.1 - x.cos()).abs();
    ///
    /// assert!(abs_difference_0 < 1e-10);
    /// assert!(abs_difference_1 < 1e-10);
    /// ```
    #[inline]
    fn sin_cos(self) -> (Self, Self)
    where
        Self: Sized + Copy,
    {
        (self.sin(), self.cos())
    }

    // /// Computes the arctangent of a number. Return value is in radians in the
    // /// range [-pi/2, pi/2];
    // ///
    // /// ```
    // /// let f = 1.0_f64;
    // ///
    // /// // atan(tan(1))
    // /// let abs_difference = (f.tan().atan() - 1.0).abs();
    // ///
    // /// assert!(abs_difference < 1e-10);
    // /// ```
    // fn atan(self) -> Self;
    //
    // /// Computes the arccosine of a number. Return value is in radians in
    // /// the range [0, pi] or NaN if the number is outside the range
    // /// [-1, 1].
    // ///
    // /// ```
    // /// use core::f64;
    // ///
    // /// let f = f64::consts::PI / 4.0;
    // ///
    // /// // acos(cos(pi/4))
    // /// let abs_difference = (f.cos().acos() - f64::consts::PI / 4.0).abs();
    // ///
    // /// assert!(abs_difference < 1e-10);
    // /// ```
    // fn acos(self) -> Self;
    //
    // /// Computes the arcsine of a number. Return value is in radians in
    // /// the range [-pi/2, pi/2] or NaN if the number is outside the range
    // /// [-1, 1].
    // ///
    // /// ```
    // /// use std::f64;
    // ///
    // /// let f = f64::consts::PI / 2.0;
    // ///
    // /// // asin(sin(pi/2))
    // /// let abs_difference = (f.sin().asin() - f64::consts::PI / 2.0).abs();
    // ///
    // /// assert!(abs_difference < 1e-10);
    // /// ```
    // fn asin(self) -> Self;
}

// /// Math functions required to compute VSOP-87.
// pub trait Math {
//     /// Takes the square root of a number.
//     ///
//     /// Returns NaN if `self` is a negative number.
//     ///
//     /// ```
//     /// let positive = 4.0_f64;
//     /// let negative = -4.0_f64;
//     ///
//     /// let abs_difference = (positive.sqrt() - 2.0).abs();
//     ///
//     /// assert!(abs_difference < 1e-10);
//     /// assert!(negative.sqrt().is_nan());
//     /// ```
//     fn sqrt(self) -> Self;
// }

impl Trigonometry for f64 {
    #[inline]
    #[allow(unsafe_code)]
    fn cos(self) -> Self {
        unsafe { intrinsics::cosf64(self) }
    }

    #[inline]
    #[allow(unsafe_code)]
    fn sin(self) -> Self {
        unsafe { intrinsics::sinf64(self) }
    }

    // #[inline]
    // fn atan(self) -> Self {
    //     unsafe { cmath::atan(self) }
    // }
    //
    // #[inline]
    // fn acos(self) -> Self {
    //     unsafe { cmath::acos(self) }
    // }
    //
    // #[inline]
    // fn asin(self) -> f64 {
    //     unsafe { cmath::asin(self) }
    // }
}

// impl Math for f64 {
//     #[inline]
//     fn sqrt(self) -> Self {
//         use core::f64::NAN;
//
//         if self < 0.0 {
//             NAN
//         } else {
//             unsafe { intrinsics::sqrtf64(self) }
//         }
//     }
// }
