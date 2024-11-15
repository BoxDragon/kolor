use super::{conversion::ColorConversion, transform::ColorTransform};
use crate::{Float, Vec3};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Identifies an invertible mapping of colors in a linear [`ColorSpace`].
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TransformFn {
    None,
    /// The sRGB transfer functions (aka 'gamma correction').
    Srgb,
    /// Oklab conversion from xyz.
    OkLab,
    /// Oklch (Oklab's LCh variant) conversion from xyz.
    OkLch,
    /// CIE xyY transform.
    CieXyY,
    /// CIELAB transform.
    CieLab,
    /// CIELCh transform.
    CieLch,
    /// CIE 1960 UCS transform.
    Cie1960Ucs,
    /// CIE 1960 UCS transform in uvV form.
    Cie1960UcsUvV,
    /// CIE 1964 UVW transform.
    Cie1964Uvw,
    /// CIE 1976 Luv transform.
    Cie1976Luv,
    /// (Hue, Saturation, Lightness), where L is defined as the average of the
    /// largest and smallest color components.
    Hsl,
    /// (Hue, Saturation, Value),
    /// where V is defined as the largest component of a color
    Hsv,
    /// (Hue, Saturation, Intensity), where I is defined as the average of the
    /// three components.
    Hsi,
    /// BT.2100 ICtCp with PQ transfer function.
    IctCpPq,
    /// BT.2100 ICtCp with HLG transfer function.
    IctCpHlg,
    /// The BT.601/BT.709/BT.2020 (they are equivalent) OETF and inverse.
    Bt601,
    /// SMPTE ST 2084:2014 aka "Perceptual Quantizer" transfer functions used in
    /// BT.2100 for digitally created/distributed HDR content.
    Pq,
    // ACEScc is a logarithmic transform
    // ACES_CC,
    // ACEScct is a logarithmic transform with toe
    // ACES_CCT,
}

impl TransformFn {
    pub const ENUM_COUNT: TransformFn = TransformFn::Pq;
}

/// A set of primary colors picked to define an RGB color space.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum RgbPrimaries {
    // Primaries
    None,
    /// BT.709 is the sRGB primaries.
    Bt709,
    // BT 2020 uses the same primaries as BT 2100.
    Bt2020,
    Ap0,
    Ap1,
    /// P3 is the primaries for DCI-P3 and the variations with different white
    /// points.
    P3,
    Adobe1998,
    AdobeWide,
    Apple,
    ProPhoto,
    CieRgb,
    /// The reference XYZ color space
    CieXyz,
}
impl RgbPrimaries {
    pub const ENUM_COUNT: RgbPrimaries = RgbPrimaries::CieXyz;

    pub const fn values(&self) -> &[[Float; 2]; 3] {
        match self {
            Self::None => &[[0.0; 2]; 3],
            Self::Bt709 => &[[0.64, 0.33], [0.30, 0.60], [0.15, 0.06]],
            Self::Bt2020 => &[[0.708, 0.292], [0.17, 0.797], [0.131, 0.046]],
            Self::Ap0 => &[[0.7347, 0.2653], [0.0000, 1.0000], [0.0001, -0.0770]],
            Self::Ap1 => &[[0.713, 0.293], [0.165, 0.830], [0.128, 0.044]],
            Self::Adobe1998 => &[[0.64, 0.33], [0.21, 0.71], [0.15, 0.06]],
            Self::AdobeWide => &[[0.735, 0.265], [0.115, 0.826], [0.157, 0.018]],
            Self::ProPhoto => &[
                [0.734699, 0.265301],
                [0.159597, 0.840403],
                [0.036598, 0.000105],
            ],
            Self::Apple => &[[0.625, 0.34], [0.28, 0.595], [0.155, 0.07]],
            Self::P3 => &[[0.680, 0.320], [0.265, 0.690], [0.150, 0.060]],
            Self::CieRgb => &[[0.7350, 0.2650], [0.2740, 0.7170], [0.1670, 0.0090]],
            Self::CieXyz => &[[1.0, 0.0], [0.0, 1.0], [0.0, 0.0]],
        }
    }
}

/// Defines the color white ("achromatic point") in an RGB color system.
///
/// White points are derived from an "illuminant" which are defined
/// as some reference lighting condition based on a Spectral Power Distribution.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum WhitePoint {
    None,
    /// Incandescent/tungsten
    A,
    /// Old direct sunlight at noon
    B,
    /// Old daylight
    C,
    /// Equal energy
    E,
    /// ICC profile PCS
    D50,
    /// Mid-morning daylight
    D55,
    D60,
    /// Daylight, sRGB, Adobe-RGB
    D65,
    /// North sky daylight
    D75,
    /// P3-DCI white point, sort of greenish
    P3Dci,
    /// Cool fluorescent
    F2,
    /// Daylight fluorescent, D65 simulator
    F7,
    /// Ultralume 40, Philips TL84
    F11,
}
impl WhitePoint {
    pub const ENUM_COUNT: WhitePoint = WhitePoint::F11;

    // Pulled from http://www.brucelindbloom.com/index.html?Eqn_ChromAdapt.html
    // Originally from ASTM E308-01 except B which comes from Wyszecki & Stiles, p.
    // 769 P3Dci is something I calculated myself from wikipedia constants
    pub const fn values(&self) -> &'static [Float; 3] {
        match self {
            Self::None => &[0.0, 0.0, 0.0],
            Self::A => &[1.09850, 1.00000, 0.35585],
            Self::B => &[0.99072, 1.00000, 0.85223],
            Self::C => &[0.98074, 1.00000, 1.18232],
            Self::D50 => &[0.96422, 1.00000, 0.82521],
            Self::D55 => &[0.95682, 1.00000, 0.92149],
            Self::D60 => &[0.9523, 1.00000, 1.00859],
            Self::D65 => &[0.95047, 1.00000, 1.08883],
            Self::D75 => &[0.94972, 1.00000, 1.22638],
            #[allow(clippy::excessive_precision)]
            Self::P3Dci => &[0.89458689458, 1.00000, 0.95441595441],
            Self::E => &[1.00000, 1.00000, 1.00000],
            Self::F2 => &[0.99186, 1.00000, 0.67393],
            Self::F7 => &[0.95041, 1.00000, 1.08747],
            Self::F11 => &[1.00962, 1.00000, 0.64350],
        }
    }
}

/// A color space defined in data by its [primaries][RgbPrimaries], [white
/// point][WhitePoint], and an optional [invertible transform
/// function][TransformFn].
///
/// See the [`spaces`][crate::spaces] module for defined color spaces.
///
/// `ColorSpace` assumes that a color space is one of:
///
/// * The CIE XYZ color space.
///
/// * An RGB color space.
///
/// * A color space which may be defined as an invertible mapping from one of
///   the above ([`TransformFn`]).
///
/// An example of a `TransformFn` is the sRGB "opto-eletronic transfer
/// function", or 'gamma compensation'.
///
/// `kolor` makes the distinction between 'linear' and 'non-linear' color
/// spaces, where a linear color space can be defined as a linear transformation
/// from the CIE XYZ color space.
///
/// `ColorSpace` contains a reference [WhitePoint] to represent a color space's
/// reference illuminant.
///
/// A linear RGB `ColorSpace` can be thought of as defining a relative
/// coordinate system in the CIE XYZ color coordinate space, where three RGB
/// primaries each define an axis pointing from the black point (0,0,0) in CIE
/// XYZ.
///
/// Non-linear `ColorSpace`s -- such as sRGB with gamma compensation applied --
/// are defined as a non-linear mapping from a linear `ColorSpace`'s coordinate
/// system.
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ColorSpace {
    primaries: RgbPrimaries,
    white_point: WhitePoint,
    transform_fn: TransformFn,
}
impl ColorSpace {
    pub const fn new(
        primaries: RgbPrimaries,
        white_point: WhitePoint,
        transform_fn: TransformFn,
    ) -> Self {
        Self {
            primaries,
            white_point,
            transform_fn,
        }
    }

    pub(crate) const fn linear(primaries: RgbPrimaries, white_point: WhitePoint) -> Self {
        Self {
            primaries,
            white_point,
            transform_fn: TransformFn::None,
        }
    }

    /// Whether the color space has a non-linear transform applied
    pub fn is_linear(&self) -> bool {
        self.transform_fn == TransformFn::None
    }

    pub fn as_linear(&self) -> Self {
        Self {
            primaries: self.primaries,
            white_point: self.white_point,
            transform_fn: TransformFn::None,
        }
    }

    pub fn primaries(&self) -> RgbPrimaries {
        self.primaries
    }

    pub fn white_point(&self) -> WhitePoint {
        self.white_point
    }

    pub fn transform_function(&self) -> TransformFn {
        self.transform_fn
    }

    /// Creates a new color space with the primaries and white point from
    /// `this`, but with the provided [`TransformFn`].
    pub fn with_transform(&self, new_transform: TransformFn) -> Self {
        Self {
            primaries: self.primaries,
            white_point: self.white_point,
            transform_fn: new_transform,
        }
    }

    /// Creates a new color space with the transform function and white point
    /// from `this`, but with the provided [`WhitePoint`].
    pub fn with_whitepoint(&self, new_wp: WhitePoint) -> Self {
        Self {
            primaries: self.primaries,
            white_point: new_wp,
            transform_fn: self.transform_fn,
        }
    }

    /// Creates a new color space with the primaries and transform function from
    /// `this`, but with the provided [`RgbPrimaries`].
    pub fn with_primaries(&self, primaries: RgbPrimaries) -> Self {
        Self {
            primaries,
            white_point: self.white_point,
            transform_fn: self.transform_fn,
        }
    }

    /// Creates a CIE LAB color space using this space's white point.
    pub fn to_cie_lab(&self) -> Self {
        Self::new(RgbPrimaries::CieXyz, self.white_point, TransformFn::CieLab)
    }

    /// Creates a CIE uvV color space using this space's white point.
    pub fn to_cie_xyy(&self) -> Self {
        Self::new(RgbPrimaries::CieXyz, self.white_point, TransformFn::CieXyY)
    }

    /// Creates a CIE LCh color space using this space's white point.
    pub fn to_cie_lch(&self) -> Self {
        Self::new(RgbPrimaries::CieXyz, self.white_point, TransformFn::CieLch)
    }
}

pub mod color_spaces {
    use super::*;

    /// Linear sRGB is a linear encoding in [BT.709
    /// primaries][RgbPrimaries::Bt709] with a [D65
    /// whitepoint.][WhitePoint::D65] Linear sRGB is equivalent to [BT_709].
    pub const LINEAR_SRGB: ColorSpace = ColorSpace::linear(RgbPrimaries::Bt709, WhitePoint::D65);

    /// Encoded sRGB is [linear sRGB][LINEAR_SRGB] with the [sRGB
    /// OETF](TransformFn::Srgb) applied (also called 'gamma-compressed').
    pub const ENCODED_SRGB: ColorSpace =
        ColorSpace::new(RgbPrimaries::Bt709, WhitePoint::D65, TransformFn::Srgb);

    /// BT.709 is a linear encoding in [BT.709 primaries][RgbPrimaries::Bt709]
    /// with a [D65 whitepoint.][WhitePoint::D65]. It's equivalent to [Linear
    /// sRGB][LINEAR_SRGB]
    pub const BT_709: ColorSpace = ColorSpace::linear(RgbPrimaries::Bt709, WhitePoint::D65);

    /// Encoded BT.709 is [BT.709](BT_709) with the [BT.709
    /// OETF](TransformFn::Bt601) applied.
    pub const ENCODED_BT_709: ColorSpace =
        ColorSpace::new(RgbPrimaries::Bt709, WhitePoint::D65, TransformFn::Bt601);

    /// ACEScg is a linear encoding in [AP1 primaries][RgbPrimaries::Ap1]
    /// with a [D60 whitepoint][WhitePoint::D60].
    pub const ACES_CG: ColorSpace = ColorSpace::linear(RgbPrimaries::Ap1, WhitePoint::D60);

    /// ACES2065-1 is a linear encoding in [AP0 primaries][RgbPrimaries::Ap0]
    /// with a [D60 whitepoint][WhitePoint::D60].
    pub const ACES_2065_1: ColorSpace = ColorSpace::linear(RgbPrimaries::Ap0, WhitePoint::D60);

    /// CIE RGB is the original RGB space, defined in [CIE RGB
    /// primaries][RgbPrimaries::CieRgb] with white point
    /// [E][WhitePoint::E].
    pub const CIE_RGB: ColorSpace = ColorSpace::linear(RgbPrimaries::CieRgb, WhitePoint::E);

    /// CIE XYZ reference color space. Uses [CIE XYZ
    /// primaries][RgbPrimaries::CieXyz] with white point
    /// [D65][WhitePoint::D65].
    pub const CIE_XYZ: ColorSpace = ColorSpace::linear(RgbPrimaries::CieXyz, WhitePoint::D65);

    /// BT.2020 is a linear encoding in [BT.2020
    /// primaries][RgbPrimaries::Bt2020] with a [D65 white
    /// point][WhitePoint::D65] BT.2100 has the same linear color space as
    /// BT.2020.
    pub const BT_2020: ColorSpace = ColorSpace::linear(RgbPrimaries::Bt2020, WhitePoint::D65);

    /// Encoded BT.2020 is [BT.2020](BT_2020) with the [BT.2020
    /// OETF][TransformFn::Bt601] applied.
    pub const ENCODED_BT_2020: ColorSpace =
        ColorSpace::new(RgbPrimaries::Bt2020, WhitePoint::D65, TransformFn::Bt601);

    /// Encoded BT.2100 PQ is [BT.2020](BT_2020) (equivalent to the linear
    /// BT.2100 space) with the [Perceptual Quantizer inverse
    /// EOTF][TransformFn::Pq] applied.
    pub const ENCODED_BT_2100_PQ: ColorSpace =
        ColorSpace::new(RgbPrimaries::Bt2020, WhitePoint::D65, TransformFn::Pq);

    /// Oklab is a non-linear, perceptual encoding in
    /// [XYZ][RgbPrimaries::CieXyz], with a [D65 whitepoint][WhitePoint::D65].
    ///
    /// Oklab's perceptual qualities make it a very attractive color space for
    /// performing blend operations between two colors which you want to be
    /// perceptually pleasing. See [this article](https://bottosson.github.io/posts/oklab/)
    /// for more on why you might want to use the Oklab colorspace.
    pub const OK_LAB: ColorSpace =
        ColorSpace::new(RgbPrimaries::CieXyz, WhitePoint::D65, TransformFn::OkLab);

    /// Oklch is a non-linear, perceptual encoding in
    /// [XYZ][RgbPrimaries::CieXyz], with a [D65
    /// whitepoint][WhitePoint::D65]. It is a variant of [Oklab](OK_LAB) with
    /// LCh coordinates intead of Lab.
    ///
    /// Oklch's qualities make it a very attractive color space for performing
    /// computational modifications to a color. You can think of it as an
    /// improved version of an HSL/HSV-style color space. See
    /// [this article](https://bottosson.github.io/posts/oklab/) for more on why
    /// you might want to use the Oklch colorspace.
    pub const OK_LCH: ColorSpace =
        ColorSpace::new(RgbPrimaries::CieXyz, WhitePoint::D65, TransformFn::OkLch);

    /// ICtCp_PQ is a non-linear encoding in [BT.2020
    /// primaries][RgbPrimaries::Bt2020], with a [D65
    /// whitepoint][WhitePoint::D65], using the PQ transfer function
    pub const ICT_CP_PQ: ColorSpace =
        ColorSpace::new(RgbPrimaries::Bt2020, WhitePoint::D65, TransformFn::IctCpPq);
    /// ICtCp_HLG is a non-linear encoding in [BT.2020
    /// primaries][RgbPrimaries::Bt2020], with a [D65
    /// whitepoint][WhitePoint::D65], using the HLG transfer function
    pub const ICT_CP_HLG: ColorSpace =
        ColorSpace::new(RgbPrimaries::Bt2020, WhitePoint::D65, TransformFn::IctCpHlg);

    /// Encoded Display P3 is [Display P3][DISPLAY_P3] with the [sRGB
    /// OETF](TransformFn::Srgb) applied.
    pub const ENCODED_DISPLAY_P3: ColorSpace =
        ColorSpace::new(RgbPrimaries::P3, WhitePoint::D65, TransformFn::Srgb);

    /// Display P3 by Apple is a linear encoding in [P3
    /// primaries][RgbPrimaries::P3] with a [D65 white
    /// point][WhitePoint::D65]
    pub const DISPLAY_P3: ColorSpace = ColorSpace::linear(RgbPrimaries::P3, WhitePoint::D65);

    /// P3-D60 (ACES Cinema) is a linear encoding in [P3
    /// primaries][RgbPrimaries::P3] with a [D60 white
    /// point][WhitePoint::D60]
    pub const P3_D60: ColorSpace = ColorSpace::linear(RgbPrimaries::P3, WhitePoint::D60);

    /// P3-DCI (Theater) is a linear encoding in [P3
    /// primaries][RgbPrimaries::P3] with a [P3-DCI white
    /// point][WhitePoint::P3Dci]
    pub const P3_THEATER: ColorSpace = ColorSpace::linear(RgbPrimaries::P3, WhitePoint::P3Dci);

    /// Adobe RGB (1998) is a linear encoding in [Adobe 1998
    /// primaries][RgbPrimaries::Adobe1998] with a [D65 white
    /// point][WhitePoint::D65]
    pub const ADOBE_1998: ColorSpace = ColorSpace::linear(RgbPrimaries::Adobe1998, WhitePoint::D65);

    /// Adobe Wide Gamut RGB is a linear encoding in [Adobe Wide
    /// primaries][RgbPrimaries::AdobeWide] with a [D50 white
    /// point][WhitePoint::D50]
    pub const ADOBE_WIDE: ColorSpace = ColorSpace::linear(RgbPrimaries::AdobeWide, WhitePoint::D50);

    /// Pro Photo RGB is a linear encoding in [Pro Photo
    /// primaries][RgbPrimaries::ProPhoto] with a [D50 white
    /// point][WhitePoint::D50]
    pub const PRO_PHOTO: ColorSpace = ColorSpace::linear(RgbPrimaries::ProPhoto, WhitePoint::D50);

    /// Apple RGB is a linear encoding in [Apple primaries][RgbPrimaries::Apple]
    /// with a [D65 white point][WhitePoint::D65]
    pub const APPLE: ColorSpace = ColorSpace::linear(RgbPrimaries::Apple, WhitePoint::D65);

    /// Array containing all built-in color spaces.
    pub const ALL_COLOR_SPACES: [ColorSpace; 22] = [
        color_spaces::LINEAR_SRGB,
        color_spaces::ENCODED_SRGB,
        color_spaces::BT_709,
        color_spaces::ENCODED_BT_709,
        color_spaces::BT_2020,
        color_spaces::ENCODED_BT_2020,
        color_spaces::ENCODED_BT_2100_PQ,
        color_spaces::ACES_CG,
        color_spaces::ACES_2065_1,
        color_spaces::CIE_RGB,
        color_spaces::CIE_XYZ,
        color_spaces::OK_LAB,
        color_spaces::ICT_CP_PQ,
        color_spaces::ICT_CP_HLG,
        color_spaces::PRO_PHOTO,
        color_spaces::APPLE,
        color_spaces::P3_D60,
        color_spaces::P3_THEATER,
        color_spaces::DISPLAY_P3,
        color_spaces::ENCODED_DISPLAY_P3,
        color_spaces::ADOBE_1998,
        color_spaces::ADOBE_WIDE,
    ];
}

/// A 3-component vector defined in a [`ColorSpace`].
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Color {
    pub value: Vec3,
    pub space: ColorSpace,
}
impl Color {
    pub const fn new(x: Float, y: Float, z: Float, space: ColorSpace) -> Self {
        #[cfg(all(feature = "glam", feature = "f64"))]
        return Self {
            value: glam::f64::DVec3::new(x, y, z),
            space,
        };
        #[cfg(all(feature = "glam", feature = "f32"))]
        return Self {
            value: glam::f32::Vec3::new(x, y, z),
            space,
        };
        #[cfg(not(feature = "glam"))]
        return Self {
            value: Vec3::new(x, y, z),
            space,
        };
    }

    pub const fn space(&self) -> ColorSpace {
        self.space
    }

    /// Equivalent to `Color::new(x, y, z, kolor::spaces::ENCODED_SRGB)`
    pub const fn srgb(x: Float, y: Float, z: Float) -> Self {
        #[cfg(all(feature = "glam", feature = "f64"))]
        return Self {
            value: glam::f64::DVec3::new(x, y, z),
            space: color_spaces::ENCODED_SRGB,
        };
        #[cfg(all(feature = "glam", feature = "f32"))]
        return Self {
            value: glam::f32::Vec3::new(x, y, z),
            space: color_spaces::ENCODED_SRGB,
        };
        #[cfg(not(feature = "glam"))]
        return Self {
            value: Vec3::new(x, y, z),
            space: color_spaces::ENCODED_SRGB,
        };
    }

    /// Returns a `Color` converted into the provided [`ColorSpace`].
    pub fn to(&self, space: ColorSpace) -> Color {
        let conversion = ColorConversion::new(self.space, space);
        let new_color = conversion.convert(self.value);
        Color {
            space,
            value: new_color,
        }
    }

    pub fn to_linear(&self) -> Color {
        if self.space.is_linear() {
            *self
        } else {
            let transform = ColorTransform::new(self.space.transform_function(), TransformFn::None)
                .unwrap_or_else(|| {
                    panic!(
                        "expected transform for {:?}",
                        self.space.transform_function()
                    )
                });
            let new_color_value = transform.apply(self.value, self.space().white_point);
            Self {
                value: new_color_value,
                space: self.space.as_linear(),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::details::conversion::LinearColorConversion;
    use color_spaces as spaces;
    #[test]
    fn linear_srgb_to_aces_cg() {
        let conversion = LinearColorConversion::new(spaces::LINEAR_SRGB, spaces::ACES_CG);
        let result = conversion.convert(Vec3::new(0.35, 0.2, 0.8));
        assert!(result.abs_diff_eq(Vec3::new(0.32276854, 0.21838512, 0.72592676), 0.001));
    }

    #[test]
    fn linear_srgb_to_aces_2065_1() {
        let conversion = ColorConversion::new(spaces::LINEAR_SRGB, spaces::ACES_2065_1);
        let result = conversion.convert(Vec3::new(0.35, 0.2, 0.8));
        assert!(result.abs_diff_eq(Vec3::new(0.3741492, 0.27154857, 0.7261116), 0.001));
    }

    #[test]
    fn linear_srgb_to_srgb() {
        let transform = ColorTransform::new(TransformFn::None, TransformFn::Srgb).unwrap();
        let test = Vec3::new(0.35, 0.1, 0.8);
        let result = transform.apply(test, WhitePoint::D65);
        let expected = Vec3::new(0.6262097, 0.34919018, 0.9063317);
        assert!(
            result.abs_diff_eq(expected, 0.001),
            "{} != {}",
            result,
            expected
        );
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
        let conversion = ColorConversion::new(spaces::ACES_CG, spaces::ENCODED_SRGB);
        let result = conversion.convert(Vec3::new(0.35, 0.1, 0.8));
        let expected = Vec3::new(0.713855624199, 0.271821975708, 0.955197274685);
        assert!(
            result.abs_diff_eq(expected, 0.01),
            "{} != {}",
            result,
            expected
        );
    }
}
