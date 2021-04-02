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
