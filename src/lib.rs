//! kolor implements conversions between 3-component color spaces.
//!
//! kolor is intended for use in games or other interactive visual applications,
//! where it can help implement correct color management.
//!
//! # Named color spaces
//! Named color spaces can be found in [spaces]. You can also construct custom [ColorSpace]s
//! from a combination of primaries, whitepoint and transform function.
//! - sRGB / linear sRGB / BT.709
//! - BT.2020
//! - ACEScg
//! - ACES2065-1
//! - Oklab
//! - CIE RGB
//!
//! # Design
//! kolor supports color spaces with 3-component coordinates, such as RGB, LAB, XYZ, HSL and more.
//!
//! In the spirit of keeping things simple, kolor uses a single type, [Color], to represent
//! a color coordinate in any supported color space.
//!
//! kolor recognizes that users may want to perform conversions on colors stored in types defined
//! by the user. [ColorConversion] represents a conversion between two color spaces
//! and is intended to be compatible with 3-component vectors in many math libraries.
//!
//! kolor defines conversions from a source [ColorSpace] to a destination [ColorSpace] as three parts:
//! - if the source color space is a non-linear coordinate space,
//!     apply the inverse of its non-linear transform function to convert to its linear color coordinate system
//! - a linear 3x3 transformation matrix from one linear color coordinate system to another
//! - if the destination color space is a non-linear coordinate space,
//!     apply its non-linear transform function
//!
//! A "non-linear transform function" means any function that is not a linear transformation
//! of the CIE XYZ color space. Examples include the sRGB logarithmic gamma compensation function,
//! the Oklab transform function, and the HSL/HSV hexagonal/circular transform.
//!
//! For non-linear color spaces, many non-linear transform functions are supported
//! to convert between popular spaces.
//! For GPU contexts, these implementations clearly can't be used directly, but required transforms
//! between spaces can be read from a [ColorConversion] value so that the user can implement
//! and run these as necessary.
//! Feel free to port the implementations in [details::transform] to your shaders or other code.
//!
//! ### Gamut-agnostic transforms
//! Certain non-linear color spaces like CIELAB or HSL can be used with any linear RGB coordinate
//! system. Use [`ColorSpace::with_transform`] based on the relevant [ColorSpace] to convert to
//! these color spaces.
//!
//! # Details
//! kolor can calculate 3x3 conversion matrices between any linear color coordinate systems
//! defined by RGB primaries and a white point. kolor offers APIs for performing
//! conversions directly, and for extracting the 3x3 matrix to use in a different context,
//! for example on a GPU.
//!
//! ### Generating conversion matrices between RGB coordinate systems
//! [LinearColorConversion][details::conversion::LinearColorConversion] can be used
//! to generate conversion matrices "offline",
//! in which case you probably want to use the `f64` feature for better precision.
//! The precision of the derived matrices won't be perfect, but probably good enough for games.
//!
//! Conversions between all combinations of built-in primaries and whitepoints color spaces
//! are bundled with kolor as constants with the `color-matrices` feature, which is enabled by default.
//! When a [ColorConversion] without a bundled pre-calculated conversion matrix is created,
//! it is calculated on-demand, meaning the creation will be a bit slower to create than
//! if there is a constant matrix available.
//!
//! ### Chromatic Adaptation Transformation (CAT)
//! kolor implements CAT in [details::cat] and supports the LMS cone spaces defined
//! in [LMSConeSpace][details::cat::LMSConeSpace]. Chromatic Adaptation Transformation means converting
//! a linear RGB color space from one reference [WhitePoint][details::color::WhitePoint] to another.
//!
//! Use [ColorSpace::with_whitepoint] to change [WhitePoint][details::color::WhitePoint] for a color space.
//!  
//! ### XYZ-RGB conversions
//! All RGB color spaces use the CIE XYZ color space as its reference, "global" coordinate system.
//! Functions in [details::xyz] can be used to create conversion matrices to/from an RGB coordinate
//! system given a set of primaries and a white point.
//!
//! # no_std support
//! kolor supports `no_std` by disabling the default-enabled `std` feature.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "f64")]
type FType = f64;
#[cfg(feature = "f64")]
use glam::f64::DMat3 as Mat3;
#[cfg(feature = "f64")]
use glam::f64::DVec3 as Vec3;

#[cfg(not(feature = "f64"))]
type FType = f32;
#[cfg(not(feature = "f64"))]
use glam::f32::Mat3;
#[cfg(not(feature = "f64"))]
use glam::f32::Vec3;

pub mod details {
    pub mod cat;
    pub mod color;
    pub mod conversion;
    #[allow(clippy::excessive_precision)]
    #[cfg(feature = "color-matrices")]
    pub mod generated_matrices;
    pub mod transform;
    pub mod xyz;
}
#[doc(inline)]
pub use details::color::color_spaces as spaces;
#[doc(inline)]
pub use details::color::{Color, ColorSpace};
#[doc(inline)]
pub use details::conversion::ColorConversion;
