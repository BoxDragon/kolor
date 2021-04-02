use crate::{conversion::ColorConversion, transform::ColorTransform, FType, Vec3};
#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[allow(non_camel_case_types)]
pub enum TransformFn {
    NONE,
    /// The SRGB "gamma compensation" function
    SRGB_Gamma,
    /// Oklab conversion from xyz
    Oklab,
    /// ACEScc is a logarithmic transform
    ACES_CC,
    /// ACEScct is a logarithmic transform with toe
    ACES_CCT,
    MAX_VALUE,
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
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
    CIE_RGB,
    CIE_XYZ,
    MAX_VALUE,
}
impl RGBPrimaries {
    pub const fn values(&self) -> &[[FType; 2]; 3] {
        match self {
            Self::NONE | Self::MAX_VALUE => &[[0.0; 2]; 3],
            Self::BT_709 => &[[0.64, 0.33], [0.30, 0.60], [0.15, 0.06]],
            Self::BT_2020 => &[[0.708, 0.292], [0.17, 0.797], [0.131, 0.046]],
            Self::BT_2100 => &[[0.708, 0.292], [0.170, 0.797], [0.131, 0.046]],
            Self::AP0 => &[[0.7347, 0.2653], [0.0000, 1.0000], [0.0001, -0.0770]],
            Self::AP1 => &[[0.713, 0.293], [0.165, 0.830], [0.128, 0.044]],
            Self::CIE_RGB => &[[0.7350, 0.2650], [0.2740, 0.7170], [0.1670, 0.0090]],
            Self::CIE_XYZ => &[[1.0, 0.0], [0.0, 1.0], [0.0, 0.0]],
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[allow(non_camel_case_types)]
pub enum WhitePoint {
    NONE,
    A,
    B,
    C,
    E,
    D50,
    D55,
    D60,
    D65,
    D75,
    F2,
    F7,
    F11,
    MAX_VALUE,
}
impl WhitePoint {
    pub const fn values(&self) -> &'static [FType; 3] {
        match self {
            Self::NONE | Self::MAX_VALUE => &[0.0, 0.0, 0.0],
            Self::A => &[1.09850, 1.00000, 0.35585],
            Self::B => &[0.99072, 1.00000, 0.85223],
            Self::C => &[0.98074, 1.00000, 1.18232],
            Self::D50 => &[0.96422, 1.00000, 0.82521],
            Self::D55 => &[0.95682, 1.00000, 0.92149],
            Self::D60 => &[0.9523, 1.00000, 1.00859],
            Self::D65 => &[0.95047, 1.00000, 1.08883],
            Self::D75 => &[0.94972, 1.00000, 1.22638],
            Self::E => &[1.00000, 1.00000, 1.00000],
            Self::F2 => &[0.99186, 1.00000, 0.67393],
            Self::F7 => &[0.95041, 1.00000, 1.08747],
            Self::F11 => &[1.00962, 1.00000, 0.64350],
        }
    }
}

// Color space definition is split into primaries, whitepoint and optionally a non-linear transform.
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ColorSpace {
    primaries: RGBPrimaries,
    white_point: WhitePoint,
    transform_fn: TransformFn,
}
impl ColorSpace {
    pub const fn new(
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
    pub fn as_linear(&self) -> Self {
        Self {
            primaries: self.primaries,
            white_point: self.white_point,
            transform_fn: TransformFn::NONE,
        }
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
}
#[doc(hidden)]
pub mod color_spaces {
    use super::*;
    /// LinearSrgb is a linear encoding in BT.709 primaries and whitepoint.
    pub const LINEAR_SRGB: ColorSpace = ColorSpace::linear(RGBPrimaries::BT_709, WhitePoint::D65);
    pub const BT_709: ColorSpace = ColorSpace::linear(RGBPrimaries::BT_709, WhitePoint::D65);
    /// Srgb is LinearSrgb with the sRGB tone response curve applied, also called "gamma-compressed".
    pub const SRGB: ColorSpace = ColorSpace::new(
        RGBPrimaries::BT_709,
        WhitePoint::D65,
        TransformFn::SRGB_Gamma,
    );
    /// ACEScg is a linear encoding in AP1 primaries with a D60 whitepoint.
    pub const ACES_CG: ColorSpace = ColorSpace::linear(RGBPrimaries::AP1, WhitePoint::D60);
    /// ACES2065-1 is a linear encoding in AP0 primaries with a D60 whitepoint.
    pub const ACES2065_1: ColorSpace = ColorSpace::linear(RGBPrimaries::AP0, WhitePoint::D60);
    /// CIE RGB is the Original Gangster of RGB spaces.
    pub const CIE_RGB: ColorSpace = ColorSpace::linear(RGBPrimaries::CIE_RGB, WhitePoint::E);
    /// BT.2020 is a linear encoding in BT.2020 primaries with a D65 white point
    pub const BT_2020: ColorSpace = ColorSpace::linear(RGBPrimaries::BT_2020, WhitePoint::D65);
    /// Oklab is a non-linear encoding in XYZ primaries with a D65 whitepoint
    pub const OKLAB: ColorSpace =
        ColorSpace::new(RGBPrimaries::CIE_XYZ, WhitePoint::D65, TransformFn::Oklab);

    pub const ALL_COLOR_SPACES: [ColorSpace; 7] = [
        color_spaces::BT_709,
        color_spaces::SRGB,
        color_spaces::ACES_CG,
        color_spaces::ACES2065_1,
        color_spaces::CIE_RGB,
        color_spaces::BT_2020,
        color_spaces::OKLAB,
    ];
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Color {
    pub value: Vec3,
    pub color_space: ColorSpace,
}
impl Color {
    pub fn new(x: FType, y: FType, z: FType, space: ColorSpace) -> Self {
        Self {
            value: Vec3::new(x, y, z),
            color_space: space,
        }
    }
    pub fn space(&self) -> ColorSpace {
        self.color_space
    }
    pub fn srgb(srgb_value: Vec3) -> Self {
        Self {
            value: srgb_value,
            color_space: color_spaces::SRGB,
        }
    }
    pub fn to(&self, space: ColorSpace) -> Color {
        let conversion = ColorConversion::new(self.color_space, space);
        let mut new_color = *self;
        conversion.apply(&mut new_color.value);
        new_color.color_space = space;
        new_color
    }
    pub fn to_linear(&self) -> Color {
        if self.color_space.is_linear() {
            *self
        } else {
            let mut new_color = *self;
            let transform =
                ColorTransform::new(self.color_space.transform_function(), TransformFn::NONE)
                    .unwrap_or_else(|| {
                        panic!(
                            "expected transform for {:?}",
                            self.color_space.transform_function()
                        )
                    });
            transform.apply(&mut new_color.value);
            new_color.color_space = self.color_space.as_linear();
            new_color
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::conversion::LinearColorConversion;
    use crate::xyz::{rgb_to_xyz, xyz_to_rgb};
    use color_spaces as spaces;
    #[test]
    fn linear_srgb_to_aces_cg() {
        let conversion = LinearColorConversion::new(spaces::LINEAR_SRGB, spaces::ACES_CG);
        let mut result = Vec3::new(0.35, 0.2, 0.8);
        conversion.apply(&mut result);
        assert_eq!(result, Vec3::new(0.32276854, 0.21838512, 0.72592676));
    }

    #[test]
    fn linear_srgb_to_cie_rgb() {
        let conversion = ColorConversion::new(spaces::LINEAR_SRGB, spaces::CIE_RGB);
        let mut result = Vec3::new(0.35, 0.2, 0.8);
        conversion.apply(&mut result);
        assert_eq!(result, Vec3::new(0.3252983, 0.27015764, 0.73588717));
    }

    #[test]
    fn linear_srgb_to_aces_2065_1() {
        let conversion = ColorConversion::new(spaces::LINEAR_SRGB, spaces::ACES2065_1);
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
        let conversion = ColorConversion::new(spaces::ACES_CG, spaces::SRGB);
        let mut result = Vec3::new(0.35, 0.1, 0.8);
        conversion.apply(&mut result);
        assert_eq!(result, Vec3::new(0.46201152, 0.06078783, 0.8996733));
    }

    #[test]
    fn aces2065_1_to_xyz_test() {
        let rgb_to_xyz = rgb_to_xyz(
            spaces::ACES2065_1.primaries().values(),
            spaces::ACES2065_1.white_point().values(),
        );

        let roundtrip = rgb_to_xyz.inverse() * rgb_to_xyz;
        println!("{:?}\n{:?}", rgb_to_xyz, roundtrip,);
        // println!(
        //     "{:?}",
        //     xyz_to_rgb(
        //         ColorSpace::ACES2065_1.primaries().values(),
        //         ColorSpace::ACES2065_1.white_point().values()
        //     )
        // );
    }

    #[test]
    fn rgb_to_xyz_test() {
        println!(
            "{:?}",
            rgb_to_xyz(
                spaces::LINEAR_SRGB.primaries().values(),
                spaces::LINEAR_SRGB.white_point().values()
            )
        );
        println!(
            "{:?}",
            xyz_to_rgb(
                spaces::LINEAR_SRGB.primaries().values(),
                spaces::LINEAR_SRGB.white_point().values()
            )
        );
    }

    #[test]
    fn cat_test() {
        println!(
            "{:?}",
            crate::cat::chromatic_adaptation_transform(
                Vec3::from_slice_unaligned(WhitePoint::D65.values()),
                Vec3::from_slice_unaligned(WhitePoint::E.values()),
                crate::cat::LMSConeSpace::VonKries,
            )
        );
    }

    #[test]
    fn oklab_test() {
        let xyz = Color::new(
            1.0,
            0.0,
            0.0,
            ColorSpace::new(RGBPrimaries::CIE_XYZ, WhitePoint::D65, TransformFn::NONE),
        );
        let oklab = xyz.to(spaces::OKLAB);
        println!(
            "conversion {:?}",
            ColorConversion::new(xyz.space(), oklab.space())
        );
        println!("{:?} - {:?}", xyz, oklab);
    }
}
