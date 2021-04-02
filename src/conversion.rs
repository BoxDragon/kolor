use crate::{
    color::TransformFn,
    transform::ColorTransform,
    xyz::{rgb_to_xyz, xyz_to_rgb},
    ColorSpace, Mat3, Vec3,
};
#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct LinearColorConversion {
    mat: Mat3,
    input_space: ColorSpace,
    output_space: ColorSpace,
}

impl LinearColorConversion {
    pub fn input_space(&self) -> ColorSpace {
        self.input_space
    }
    pub fn output_space(&self) -> ColorSpace {
        self.output_space
    }
    pub fn apply(&self, color: &mut Vec3) {
        *color = self.mat * *color;
    }
    pub fn matrix(&self) -> Mat3 {
        self.mat
    }

    pub fn new(src: ColorSpace, dst: ColorSpace) -> Self {
        if !src.is_linear() {
            panic!("{:?} is not a linear color space", src);
        }
        if !dst.is_linear() {
            panic!("{:?} is not a linear color space", dst);
        }
        #[cfg(feature = "color-matrices")]
        let const_conversion = crate::generated_matrices::const_conversion_matrix(
            src.primaries(),
            src.white_point(),
            dst.primaries(),
            dst.white_point(),
        );
        #[cfg(not(feature = "color-matrices"))]
        let const_conversion: Option<Mat3> = None;

        let mat = if let Some(const_mat) = const_conversion {
            const_mat
        } else {
            let src_to_xyz = rgb_to_xyz(src.primaries().values(), src.white_point().values());
            let xyz_to_dst = xyz_to_rgb(dst.primaries().values(), dst.white_point().values());
            if src.white_point() != dst.white_point() {
                let white_point_transform = crate::cat::chromatic_adaptation_transform(
                    Vec3::from_slice_unaligned(src.white_point().values()),
                    Vec3::from_slice_unaligned(dst.white_point().values()),
                    crate::cat::LMSConeSpace::Sharp,
                );
                xyz_to_dst * white_point_transform * src_to_xyz
            } else {
                xyz_to_dst * src_to_xyz
            }
        };
        Self {
            mat,
            input_space: src,
            output_space: dst,
        }
    }
}

#[derive(Copy, Clone)]
pub struct ColorConversion {
    src_space: ColorSpace,
    dst_space: ColorSpace,
    src_transform: Option<(ColorTransform, TransformFn)>,
    linear_transform: Option<LinearColorConversion>,
    dst_transform: Option<(ColorTransform, TransformFn)>,
}
impl PartialEq for ColorConversion {
    fn eq(&self, other: &Self) -> bool {
        self.src_space == other.src_space && self.dst_space == other.dst_space
    }
}
impl core::fmt::Debug for ColorConversion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let src_transform = if !self.src_space.is_linear() {
            self.src_space.transform_function()
        } else {
            TransformFn::NONE
        };
        let dst_transform = if !self.dst_space.is_linear() {
            self.dst_space.transform_function()
        } else {
            TransformFn::NONE
        };
        f.debug_struct("ColorConversion")
            .field("src_space", &self.src_space)
            .field("dst_space", &self.dst_space)
            .field("src_transform", &src_transform)
            .field("linear_transform", &self.linear_transform)
            .field("dst_transform", &dst_transform)
            .finish()
    }
}

impl ColorConversion {
    pub fn new(src: ColorSpace, dst: ColorSpace) -> Self {
        let src_transform = if !src.is_linear() {
            ColorTransform::new(src.transform_function(), TransformFn::NONE)
                .map(|t| (t, src.transform_function()))
        } else {
            None
        };
        let src_linear = ColorSpace::linear(src.primaries(), src.white_point());
        let dst_linear = ColorSpace::linear(dst.primaries(), dst.white_point());
        let linear_transform = LinearColorConversion::new(src_linear, dst_linear);
        let linear_transform = if linear_transform.mat == Mat3::IDENTITY {
            None
        } else {
            Some(linear_transform)
        };
        let dst_transform = if !dst.is_linear() {
            ColorTransform::new(TransformFn::NONE, dst.transform_function())
                .map(|t| (t, dst.transform_function()))
        } else {
            None
        };
        Self {
            src_space: src,
            dst_space: dst,
            src_transform,
            dst_transform,
            linear_transform,
        }
    }
    pub fn invert(&self) -> Self {
        ColorConversion::new(self.dst_space, self.src_space)
    }
    pub fn is_linear(&self) -> bool {
        self.src_transform.is_none() && self.dst_transform.is_none()
    }
    pub fn linear_part(&self) -> LinearColorConversion {
        if let Some(transform) = self.linear_transform.as_ref() {
            *transform
        } else {
            LinearColorConversion {
                input_space: self.src_space,
                output_space: self.dst_space,
                mat: Mat3::IDENTITY,
            }
        }
    }
    pub fn src_space(&self) -> ColorSpace {
        self.src_space
    }
    pub fn dst_space(&self) -> ColorSpace {
        self.dst_space
    }
    pub fn apply(&self, color: &mut Vec3) {
        if let Some((src_transform, _)) = self.src_transform.as_ref() {
            src_transform.apply(color);
        }
        if let Some(transform) = self.linear_transform.as_ref() {
            transform.apply(color);
        }
        if let Some((dst_transform, _)) = self.dst_transform.as_ref() {
            dst_transform.apply(color);
        }
    }
}