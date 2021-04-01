use glam::f32::{Mat3, Vec3};
#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

mod color_space {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
    #[cfg_attr(feature = "serde1", Serialize, Deserialize)]
    #[allow(non_camel_case_types)]
    pub enum WhitePoint {
        NONE,
        D60,
        D65,
        E,
    }

    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
    #[cfg_attr(feature = "serde1", Serialize, Deserialize)]
    #[allow(non_camel_case_types)]
    pub enum TransformFn {
        NONE,
        /// The SRGB "gamma compensation" function
        SRGB_Gamma,
        /// CIE 1931 XYZ color space
        CIE_XYZ,
        /// ACEScc is a logarithmic transform
        ACES_CC,
        /// ACEScct is a logarithmic transform with toe
        ACES_CCT,
    }

    // Color space definition is split into primaries, whitepoint and optionally a non-linear transform.
    #[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
    #[cfg_attr(feature = "serde1", Serialize, Deserialize)]
    #[repr(C)]
    pub struct ColorSpace {
        primaries: RGBPrimaries,
        white_point: WhitePoint,
        transform_fn: TransformFn,
    }
    impl ColorSpace {
        const fn new(
            primaries: RGBPrimaries,
            white_point: WhitePoint,
            transform_fn: TransformFn,
        ) -> Self {
            Self {
                primaries,
                white_point,
                transform_fn,
            }
        }
        pub(crate) const fn linear(primaries: RGBPrimaries, white_point: WhitePoint) -> Self {
            Self {
                primaries,
                white_point,
                transform_fn: TransformFn::NONE,
            }
        }
        pub fn is_linear(&self) -> bool {
            self.transform_fn == TransformFn::NONE
        }
        pub fn primaries(&self) -> RGBPrimaries {
            self.primaries
        }
        pub fn white_point(&self) -> WhitePoint {
            self.white_point
        }
        pub fn transform_function(&self) -> TransformFn {
            self.transform_fn
        }
        /// LinearSrgb is a linear encoding in BT.709 primaries and whitepoint.
        pub const LINEAR_SRGB: ColorSpace =
            ColorSpace::linear(RGBPrimaries::BT_709, WhitePoint::D65);
        pub const BT_709: ColorSpace = ColorSpace::linear(RGBPrimaries::BT_709, WhitePoint::D65);
        /// Srgb is LinearSrgb with the sRGB tone response curve applied, also called "gamma-compressed".
        pub const SRGB: ColorSpace = ColorSpace::new(
            RGBPrimaries::BT_709,
            WhitePoint::D65,
            TransformFn::SRGB_Gamma,
        );
        /// ACEScg is a linear encoding in AP1 primaries.
        pub const ACES_CG: ColorSpace = ColorSpace::linear(RGBPrimaries::AP1, WhitePoint::D60);
        /// ACES2065-1 is a linear encoding in AP0 primaries.
        pub const ACES2065_1: ColorSpace = ColorSpace::linear(RGBPrimaries::AP0, WhitePoint::D60);
        /// CIE RGB
        pub const CIE_RGB: ColorSpace = ColorSpace::linear(RGBPrimaries::CIE, WhitePoint::E);
        /// BT.2020 is a linear encoding in BT.2020 primaries
        pub const BT_2020: ColorSpace = ColorSpace::linear(RGBPrimaries::BT_2020, WhitePoint::D65);
    }

    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
    #[cfg_attr(feature = "serde1", Serialize, Deserialize)]
    #[allow(non_camel_case_types)]
    pub enum RGBPrimaries {
        // Primaries
        NONE,
        /// BT.709 is the sRGB primaries.
        BT_709,
        BT_2020,
        BT_2100,
        AP0,
        AP1,
        CIE,
    }
}
pub use color_space::{ColorSpace, RGBPrimaries, TransformFn, WhitePoint};

#[rustfmt::skip]
const IDENTITY: [f32; 9] = [
    1.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 
    0.0, 0.0, 1.0];

fn working_space_from_white_point(wp: WhitePoint) -> ColorSpace {
    // could update these as we add support for more spaces
    match wp {
        WhitePoint::NONE => panic!("white point NONE"),
        WhitePoint::D60 => ColorSpace::ACES2065_1,
        WhitePoint::D65 => ColorSpace::BT_2020,
        WhitePoint::E => ColorSpace::CIE_RGB,
    }
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde1", Serialize, Deserialize)]
pub struct LinearColorConversion {
    mat: glam::Mat3,
    input_space: ColorSpace,
    output_space: ColorSpace,
}

#[derive(Copy, Clone)]
#[cfg_attr(feature = "serde1", Serialize, Deserialize)]
pub struct ColorTransform(for<'r> fn(&'r mut glam::f32::Vec3));
impl ColorTransform {
    pub fn new(src_transform: TransformFn, dst_transform: TransformFn) -> Option<Self> {
        use crate::color_transforms::*;
        Some(Self(match (src_transform, dst_transform) {
            (TransformFn::SRGB_Gamma, TransformFn::NONE) => {
                // invert gamma correction
                srgb_gamma_inverse
            }
            (TransformFn::NONE, TransformFn::SRGB_Gamma) => {
                // apply gamma correction
                srgb_gamma
            }
            _ => return None,
        }))
    }
    pub fn apply(&self, color: &mut glam::f32::Vec3) {
        self.0(color);
    }
}

impl LinearColorConversion {
    pub fn apply(&self, color: &mut glam::f32::Vec3) {
        *color = self.mat * *color;
    }
    fn to_working_space(from: ColorSpace) -> Option<(ColorSpace, Mat3)> {
        if from.white_point() == WhitePoint::NONE {
            return None;
        }
        let working_space = working_space_from_white_point(from.white_point());
        Some((working_space, Self::conversion_matrix(from, working_space)?))
    }
    fn conversion_matrix(from: ColorSpace, to: ColorSpace) -> Option<glam::f32::Mat3> {
        if from == to {
            return Some(glam::f32::Mat3::from_cols_array(&IDENTITY));
        }
        use crate::color_matrices::*;
        match (from, to) {
            (ColorSpace::BT_709, ColorSpace::ACES_CG) => {
                Some(glam::f32::Mat3::from_cols_array(&SRGB_TO_ACESCG).transpose())
            }
            (ColorSpace::BT_709, ColorSpace::ACES2065_1) => {
                Some(glam::f32::Mat3::from_cols_array(&SRGB_TO_ACES2065_1).transpose())
            }
            (ColorSpace::ACES2065_1, ColorSpace::BT_709) => {
                Some(glam::f32::Mat3::from_cols_array(&ACES2065_1_TO_SRGB).transpose())
            }
            (ColorSpace::ACES_CG, ColorSpace::BT_709) => {
                Some(glam::f32::Mat3::from_cols_array(&ACESCG_TO_SRGB).transpose())
            }
            (ColorSpace::ACES_CG, ColorSpace::ACES2065_1) => {
                Some(glam::f32::Mat3::from_cols_array(&ACES_CG_TO_ACES2065_1).transpose())
            }
            (ColorSpace::ACES2065_1, ColorSpace::ACES_CG) => {
                Some(glam::f32::Mat3::from_cols_array(&ACES2065_1_TO_ACES_CG).transpose())
            }
            (ColorSpace::BT_2020, ColorSpace::BT_709) => {
                Some(glam::f32::Mat3::from_cols_array(&BT_2020_TO_BT_709).transpose())
            }
            (ColorSpace::BT_709, ColorSpace::BT_2020) => {
                Some(glam::f32::Mat3::from_cols_array(&BT_709_TO_BT_2020).transpose())
            }
            (ColorSpace::BT_2020, ColorSpace::ACES2065_1) => {
                Some(glam::f32::Mat3::from_cols_array(&BT_2020_TO_ACES_2065_1).transpose())
            }
            (ColorSpace::ACES2065_1, ColorSpace::BT_2020) => {
                Some(glam::f32::Mat3::from_cols_array(&ACES_2065_1_TO_BT_2020).transpose())
            }
            (ColorSpace::CIE_RGB, ColorSpace::BT_2020) => {
                Some(glam::f32::Mat3::from_cols_array(&CIE_RGB_TO_BT_2020).transpose())
            }
            (ColorSpace::CIE_RGB, ColorSpace::ACES2065_1) => {
                Some(glam::f32::Mat3::from_cols_array(&CIE_RGB_TO_ACES_2065_1).transpose())
            }
            (ColorSpace::ACES2065_1, ColorSpace::CIE_RGB) => {
                Some(glam::f32::Mat3::from_cols_array(&ACES_2065_1_TO_CIE_RGB).transpose())
            }
            (ColorSpace::BT_2020, ColorSpace::CIE_RGB) => {
                Some(glam::f32::Mat3::from_cols_array(&BT_2020_TO_CIE_RGB).transpose())
            }
            _ => None,
        }
    }
    pub fn new(src: ColorSpace, dst: ColorSpace) -> Self {
        if !src.is_linear() {
            panic!("{:?} is not a linear color space", src);
        }
        if !dst.is_linear() {
            panic!("{:?} is not a linear color space", dst);
        }
        let mat = if let Some(mat) = Self::conversion_matrix(src, dst) {
            mat
        } else {
            let (src_work_space, src_space_to_work_space) = Self::to_working_space(src)
                .unwrap_or_else(|| panic!("no working space for {:?}", src));
            if let Some(src_work_space_to_dst) = Self::conversion_matrix(src_work_space, dst) {
                src_space_to_work_space * src_work_space_to_dst
            } else {
                let (dst_work_space, _) = Self::to_working_space(dst)
                    .unwrap_or_else(|| panic!("no working space for {:?}", dst));
                let src_work_space_to_dst_work_space =
                    Self::conversion_matrix(src_work_space, dst_work_space).unwrap_or_else(|| {
                        panic!(
                            "expected conversion between working spaces {:?} and {:?}",
                            src_work_space, dst_work_space,
                        )
                    });
                let dst_work_space_to_dst = Self::conversion_matrix(dst_work_space, dst)
                    .unwrap_or_else(|| {
                        panic!(
                            "expected conversion between working space {:?} and dst {:?}",
                            dst_work_space, dst,
                        )
                    });
                src_space_to_work_space * src_work_space_to_dst_work_space * dst_work_space_to_dst
            }
        };
        Self {
            mat,
            input_space: src,
            output_space: dst,
        }
    }
}

pub struct ColorConversion {
    src_space: ColorSpace,
    dst_space: ColorSpace,
    src_transform: Option<(ColorTransform, TransformFn)>,
    linear_transform: LinearColorConversion,
    dst_transform: Option<(ColorTransform, TransformFn)>,
}
impl core::fmt::Debug for ColorConversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
        let dst_transform = if !src.is_linear() {
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
    pub fn src_space(&self) -> ColorSpace {
        self.src_space
    }
    pub fn dst_space(&self) -> ColorSpace {
        self.dst_space
    }
    pub fn apply(&self, color: &mut Vec3) {
        if let Some((src_transform, _)) = self.src_transform {
            src_transform.apply(color);
        }
        self.linear_transform.apply(color);
        if let Some((dst_transform, _)) = self.dst_transform {
            dst_transform.apply(color);
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde1", Serialize, Deserialize)]
pub struct Color {
    value: glam::f32::Vec3,
    color_space: ColorSpace,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde1", Serialize, Deserialize)]
pub struct Srgb(glam::f32::Vec3);

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde1", Serialize, Deserialize)]
pub struct LinearSrgb(glam::f32::Vec3);

impl From<Srgb> for Color {
    fn from(v: Srgb) -> Self {
        Color {
            value: v.0,
            color_space: ColorSpace::SRGB,
        }
    }
}

impl From<LinearSrgb> for Color {
    fn from(v: LinearSrgb) -> Self {
        Color {
            value: v.0,
            color_space: ColorSpace::LINEAR_SRGB,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn linear_srgb_to_aces_cg() {
        let conversion = LinearColorConversion::new(ColorSpace::LINEAR_SRGB, ColorSpace::ACES_CG);
        let mut result = Vec3::new(0.35, 0.2, 0.8);
        conversion.apply(&mut result);
        assert_eq!(result, Vec3::new(0.32276854, 0.21838512, 0.72592676));
    }

    #[test]
    fn linear_srgb_to_cie_rgb() {
        let conversion = LinearColorConversion::new(ColorSpace::LINEAR_SRGB, ColorSpace::CIE_RGB);
        let mut result = Vec3::new(0.35, 0.2, 0.8);
        conversion.apply(&mut result);
        assert_eq!(result, Vec3::new(0.3252983, 0.27015764, 0.73588717));
    }

    #[test]
    fn linear_srgb_to_aces_2065_1() {
        let conversion =
            LinearColorConversion::new(ColorSpace::LINEAR_SRGB, ColorSpace::ACES2065_1);
        let mut result = Vec3::new(0.35, 0.2, 0.8);
        conversion.apply(&mut result);
        assert_eq!(result, Vec3::new(0.3741492, 0.27154857, 0.7261116));
    }

    #[test]
    fn linear_srgb_to_srgb() {
        let transform = ColorTransform::new(TransformFn::NONE, TransformFn::SRGB_Gamma).unwrap();
        let mut result = Vec3::new(0.35, 0.1, 0.8);
        transform.apply(&mut result);
        assert_eq!(result, Vec3::new(0.6262097, 0.34919018, 0.9063317));
    }

    // #[test]
    // fn working_space_conversions() {
    //     // just make sure we aren't missing a conversion
    //     for src in &WORKING_SPACE_BY_WHITE_POINT {
    //         for dst in &WORKING_SPACE_BY_WHITE_POINT {
    //             let conversion = LinearColorConversion::new(*src, *dst);
    //             let mut result = Vec3::new(0.35, 0.2, 0.8);
    //             conversion.apply(&mut result);
    //         }
    //     }
    // }

    #[test]
    fn aces_cg_to_srgb() {
        let conversion = ColorConversion::new(ColorSpace::ACES_CG, ColorSpace::SRGB);
        let mut result = Vec3::new(0.35, 0.1, 0.8);
        conversion.apply(&mut result);
        assert_eq!(result, Vec3::new(0.46201152, 0.06078783, 0.8996733));
    }
}
