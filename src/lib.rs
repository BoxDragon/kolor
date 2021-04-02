//! kolor implements conversions between 3-component color spaces.
//!
//! kolor is intended for use in games or other interactive visual applications,
//! where it can help implement correct color management.
//!
//!
//! # Supported color spaces
//! - sRGB / linear sRGB / BT.709
//! - BT.2020
//! - ACEScg
//! - ACES2065-1
//! - Oklab
//!
//! # Design
//! kolor only supports colors with 3-component coordinates, such as RGB, LAB, XYZ, HSL etc.
//!
//! kolor can calculate 3x3 conversion matrices between any linear color coordinate systems
//! defined by RGB primaries and a white point. kolor offers APIs for performing
//! conversions directly or extracting the 3x3 matrix to use in a different context,
//! for example on a GPU.
//!
//! For non-linear color spaces, a set of non-linear transform functions are supported
//! to convert between popular spaces.
//! For GPU contexts, these implementations clearly can't be used, but required transforms
//! between spaces can be read from a [ColorConversion] value so that the user can implement
//! and run these as necessary.
//! Feel free to port the implementations in [details::transform][crate::details::transform]
//! to your shaders.
//!
//! kolor defines conversions from a source color space to a destination color space as three parts:
//! - an optional non-linear transform function to a linear color coordinate system
//! - a linear 3x3 transformation matrix from one linear color coordinate system to another
//! - an optional non-linear transform function from a linear color coordinate system
//!
//! kolor can be used to pre-generate conversion matrices "offline",
//! in which case you probably want to use the `f64` feature for better precision.
//!
//! Conversions between all pre-defined color spaces are bundled with kolor as constants
//! with the default-enabled `color-matrices` feature.
//! When a [ColorConversion] is requested which does not have a pre-calculated conversion matrix,
//! it is calculated transparently, but will be a bit slower to create than if there is a 
//! constant matrix available.

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
