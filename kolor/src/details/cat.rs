//! Implements [Chromatic Adaptation](https://en.wikipedia.org/wiki/Chromatic_adaptation)
//! Transformation (CAT).
//!
//! Chromatic Adaptation Transformation means transforming a linear color
//! space's coordinate system from one white point reference to another.
//!
//! [`Sharp`(LmsConeSpace::Sharp) is used as the default for conversions by
//! [`ColorConversion`][crate::details::conversion::ColorConversion].
use crate::{Mat3, Vec3};

/// Supported conversion methods.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub enum LmsConeSpace {
    VonKries,
    Bradford,
    #[default]
    Sharp,
    CmcCat2000,
    Cat02,
}

impl LmsConeSpace {
    /// Returns the matrix for the given cone space variant.
    // from S. Bianco. "Two New von Kries Based Chromatic Adapatation Transforms
    // Found by Numerical Optimization."
    #[rustfmt::skip]
    pub fn matrix(&self) -> Mat3 {
        match self {
            LmsConeSpace::VonKries => {
                Mat3::from_cols_array(&[0.40024, -0.2263, 0.0, 0.7076, 1.16532, 0.0, -0.08081, 0.0457, 0.91822])
            }
            LmsConeSpace::Bradford => {
                Mat3::from_cols_array(&[0.8951, -0.7502, 0.0389, 0.2664, 1.7135, -0.0685, -0.1614, 0.0367, 1.0296])
            }
            LmsConeSpace::Sharp => {
                Mat3::from_cols_array(&[1.2694, -0.8364, 0.0297, -0.0988, 1.8006, -0.0315, -0.1706, 0.0357, 1.0018])
            }
            LmsConeSpace::CmcCat2000 => {
                Mat3::from_cols_array(&[0.7982, -0.5918, 0.0008, 0.3389, 1.5512, 0.239, -0.1371, 0.0406, 0.9753])
            }
            LmsConeSpace::Cat02 => {
                Mat3::from_cols_array(&[0.7328, -0.7036, 0.0030, 0.4296, 1.6975, 0.0136, -0.1624, 0.0061, 0.9834])
            }
        }
    }

    /// Calculate the CAT matrix for converting from one white point to another.
    #[rustfmt::skip]
    pub fn chromatic_adaptation_transform(
        &self,
        src_illuminant: Vec3,
        dst_illuminant: Vec3,
    ) -> Mat3 {
        let cone_space_transform = self.matrix();
        let src_cone_response = cone_space_transform * src_illuminant;
        let dst_cone_response = cone_space_transform * dst_illuminant;
        let src_to_dst_cone = Mat3::from_cols_array(&[
            dst_cone_response.x / src_cone_response.x, 0.0, 0.0,
            0.0, dst_cone_response.y / src_cone_response.y, 0.0,
            0.0, 0.0, dst_cone_response.z / src_cone_response.z,
        ]);

        cone_space_transform.inverse() * src_to_dst_cone * cone_space_transform
    }
}
