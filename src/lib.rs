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

pub mod cat;
pub mod color;
#[doc(inline)]
pub use color::{color_spaces, Color, ColorSpace};
pub mod conversion;
#[doc(inline)]
pub use conversion::ColorConversion;
#[allow(clippy::excessive_precision)]
#[cfg(feature = "color-matrices")]
mod generated_matrices;
pub mod transform;
pub mod xyz;
